//! Undo for writes into tool configs (and the Brain's own files).
//!
//! A Brain snapshot only covers the Brain. `project`, `sync`, `doctor --fix`
//! and `onboard` also overwrite files that belong to the user's AI tools,
//! such as `~/.claude/CLAUDE.md` or `~/.cursor/rules/*.mdc`. Wrapping one of
//! those operations in [`with_backup`] saves the original of every file
//! just before its first change. The copies go to
//! `<brain>/.brain/backups/<id>/`, and [`restore`] puts them back.
//!
//! Every write path calls [`before_write`] first. If the original cannot
//! be saved, the write is refused, so nothing is ever overwritten without
//! a copy. Files that are rewritten with identical bytes are never
//! touched, so an operation that changes nothing leaves no backup behind.
//!
//! The backups folder holds its own `.gitignore` (`*`). That keeps copies
//! of tool configs out of Brain snapshots, and a Brain rollback leaves
//! them alone.

use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// Where backups live, relative to the Brain root.
pub const BACKUPS_DIR: &str = ".brain/backups";
const MANIFEST: &str = "manifest.json";

/// What a path looked like before the operation touched it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Original {
    /// Nothing was there. Undo deletes whatever the operation created.
    Absent,
    /// A regular file, saved under `files/<copy>` in the backup folder.
    File { copy: String },
    /// A symbolic link pointing at `target`.
    Symlink { target: PathBuf },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Entry {
    /// Absolute path of the file that was changed.
    pub path: PathBuf,
    pub original: Original,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Manifest {
    pub id: String,
    /// The command that made the change, e.g. `"project"` or `"doctor --fix"`.
    pub label: String,
    /// Milliseconds since the Unix epoch.
    pub created_ms: u64,
    pub entries: Vec<Entry>,
    /// Set once the backup has been restored with [`restore`].
    #[serde(default)]
    pub restored: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BackupError {
    NotFound(String),
    Io(String),
}

impl std::fmt::Display for BackupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BackupError::NotFound(msg) => write!(f, "{msg}"),
            BackupError::Io(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for BackupError {}

struct Recorder {
    dir: Option<PathBuf>,
    backups_root: PathBuf,
    manifest: Manifest,
}

thread_local! {
    static ACTIVE: RefCell<Option<Recorder>> = const { RefCell::new(None) };
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

fn io_err(what: &str, path: &Path, e: io::Error) -> io::Error {
    io::Error::new(e.kind(), format!("{what} {}: {e}", path.display()))
}

/// Runs `op` with every file change recorded into a new backup labelled
/// `label`. Returns the operation's result and the backup, which is `None`
/// when nothing changed. A call nested inside another `with_backup` (for
/// example `project` inside `sync`) joins the outer backup instead of
/// starting its own.
pub fn with_backup<T, E>(
    brain_root: &Path,
    label: &str,
    op: impl FnOnce() -> Result<T, E>,
) -> (Result<T, E>, Option<Manifest>) {
    let nested = ACTIVE.with(|a| a.borrow().is_some());
    if nested {
        return (op(), None);
    }
    let created_ms = now_ms();
    ACTIVE.with(|a| {
        *a.borrow_mut() = Some(Recorder {
            dir: None,
            backups_root: brain_root.join(BACKUPS_DIR),
            manifest: Manifest {
                id: created_ms.to_string(),
                label: label.to_string(),
                created_ms,
                entries: Vec::new(),
                restored: false,
            },
        });
    });
    // Clear the recorder even if `op` panics, so a later operation on this
    // thread does not write into a stale backup.
    struct Reset;
    impl Drop for Reset {
        fn drop(&mut self) {
            ACTIVE.with(|a| a.borrow_mut().take());
        }
    }
    let reset = Reset;
    let result = op();
    let recorder = ACTIVE.with(|a| a.borrow_mut().take());
    drop(reset);
    let manifest = recorder
        .filter(|r| r.dir.is_some() && !r.manifest.entries.is_empty())
        .map(|r| r.manifest);
    (result, manifest)
}

/// Saves the original of `path` into the active backup, if there is one
/// and this path has not been saved yet. Call it right before creating,
/// overwriting or deleting `path`. An error means the original could not
/// be saved, so the caller must not go ahead with the write.
pub fn before_write(path: &Path) -> io::Result<()> {
    ACTIVE.with(|a| {
        let mut guard = a.borrow_mut();
        let Some(rec) = guard.as_mut() else {
            return Ok(());
        };
        let path = absolute(path);
        if path.starts_with(&rec.backups_root) {
            return Ok(());
        }
        if rec.manifest.entries.iter().any(|e| e.path == path) {
            return Ok(());
        }
        let dir = match &rec.dir {
            Some(d) => d.clone(),
            None => {
                let d = create_backup_dir(&rec.backups_root, &mut rec.manifest)?;
                rec.dir = Some(d.clone());
                d
            }
        };
        let original = match fs::symlink_metadata(&path) {
            Err(e) if e.kind() == io::ErrorKind::NotFound => Original::Absent,
            Err(e) => return Err(io_err("couldn't inspect", &path, e)),
            Ok(meta) if meta.file_type().is_symlink() => Original::Symlink {
                target: fs::read_link(&path).map_err(|e| io_err("couldn't read", &path, e))?,
            },
            Ok(meta) if meta.is_file() => {
                let copy = rec.manifest.entries.len().to_string();
                let files = dir.join("files");
                fs::create_dir_all(&files).map_err(|e| io_err("couldn't create", &files, e))?;
                fs::copy(&path, files.join(&copy))
                    .map_err(|e| io_err("couldn't back up", &path, e))?;
                Original::File { copy }
            }
            // Directories are never overwritten by a projection; nothing to save.
            Ok(_) => return Ok(()),
        };
        rec.manifest.entries.push(Entry { path, original });
        // Written after every entry, so a crash part-way still leaves a
        // backup that `restore` can use.
        write_manifest(&dir, &rec.manifest)
    })
}

fn absolute(path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .map(|c| c.join(path))
            .unwrap_or_else(|_| path.to_path_buf())
    }
}

fn create_backup_dir(backups_root: &Path, manifest: &mut Manifest) -> io::Result<PathBuf> {
    fs::create_dir_all(backups_root).map_err(|e| io_err("couldn't create", backups_root, e))?;
    let ignore = backups_root.join(".gitignore");
    if !ignore.exists() {
        fs::write(&ignore, "*\n").map_err(|e| io_err("couldn't write", &ignore, e))?;
    }
    // Two backups in the same millisecond get -1, -2, ... suffixes.
    let base = manifest.id.clone();
    let mut n = 0;
    loop {
        let id = if n == 0 {
            base.clone()
        } else {
            format!("{base}-{n}")
        };
        let dir = backups_root.join(&id);
        match fs::create_dir(&dir) {
            Ok(()) => {
                manifest.id = id;
                return Ok(dir);
            }
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => n += 1,
            Err(e) => return Err(io_err("couldn't create", &dir, e)),
        }
    }
}

fn write_manifest(dir: &Path, manifest: &Manifest) -> io::Result<()> {
    let json = serde_json::to_string_pretty(manifest)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
    crate::security::write_atomic(&dir.join(MANIFEST), json.as_bytes())
}

/// Every backup under `brain_root`, newest first. Folders without a
/// readable manifest are skipped.
pub fn list(brain_root: &Path) -> Result<Vec<Manifest>, BackupError> {
    let root = brain_root.join(BACKUPS_DIR);
    let entries = match fs::read_dir(&root) {
        Ok(e) => e,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => {
            return Err(BackupError::Io(format!(
                "couldn't read {}: {e}",
                root.display()
            )))
        }
    };
    let mut out: Vec<Manifest> = entries
        .flatten()
        .filter(|e| e.path().is_dir())
        .filter_map(|e| fs::read_to_string(e.path().join(MANIFEST)).ok())
        .filter_map(|raw| serde_json::from_str::<Manifest>(&raw).ok())
        .collect();
    out.sort_by(|a, b| b.created_ms.cmp(&a.created_ms).then(b.id.cmp(&a.id)));
    Ok(out)
}

/// Puts back every file saved in backup `id`, or in the newest backup not
/// yet restored when `id` is `None`. Files that did not exist before are
/// deleted. Returns the manifest, now marked restored.
pub fn restore(brain_root: &Path, id: Option<&str>) -> Result<Manifest, BackupError> {
    let all = list(brain_root)?;
    let mut manifest = match id {
        Some(id) => all
            .into_iter()
            .find(|m| m.id == id)
            .ok_or_else(|| BackupError::NotFound(format!("no backup called {id}")))?,
        None => all
            .into_iter()
            .find(|m| !m.restored)
            .ok_or_else(|| BackupError::NotFound("there is nothing to undo".to_string()))?,
    };
    // An id from the command line must name a folder directly under the
    // backups folder, never a path that climbs out of it.
    if manifest.id.contains(['/', '\\']) || manifest.id.contains("..") {
        return Err(BackupError::NotFound(format!(
            "no backup called {}",
            manifest.id
        )));
    }
    let dir = brain_root.join(BACKUPS_DIR).join(&manifest.id);
    for entry in &manifest.entries {
        restore_entry(&dir, entry).map_err(|e| BackupError::Io(e.to_string()))?;
    }
    manifest.restored = true;
    write_manifest(&dir, &manifest).map_err(|e| BackupError::Io(e.to_string()))?;
    Ok(manifest)
}

fn remove_existing(path: &Path) -> io::Result<()> {
    match fs::symlink_metadata(path) {
        Ok(meta) if meta.is_dir() => Ok(()),
        Ok(_) => fs::remove_file(path).map_err(|e| io_err("couldn't remove", path, e)),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(io_err("couldn't inspect", path, e)),
    }
}

fn restore_entry(dir: &Path, entry: &Entry) -> io::Result<()> {
    let path = &entry.path;
    match &entry.original {
        Original::Absent => remove_existing(path),
        Original::File { copy } => {
            let saved = dir.join("files").join(copy);
            if !saved.is_file() {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("the saved copy of {} is missing", path.display()),
                ));
            }
            // Remove a symlink first: copying onto it would write through
            // to whatever it points at.
            remove_existing(path)?;
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).map_err(|e| io_err("couldn't create", parent, e))?;
            }
            fs::copy(&saved, path)
                .map(|_| ())
                .map_err(|e| io_err("couldn't restore", path, e))
        }
        Original::Symlink { target } => {
            remove_existing(path)?;
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).map_err(|e| io_err("couldn't create", parent, e))?;
            }
            make_symlink(target, path).map_err(|e| io_err("couldn't restore link", path, e))
        }
    }
}

#[cfg(unix)]
fn make_symlink(target: &Path, link: &Path) -> io::Result<()> {
    std::os::unix::fs::symlink(target, link)
}

#[cfg(windows)]
fn make_symlink(target: &Path, link: &Path) -> io::Result<()> {
    std::os::windows::fs::symlink_file(target, link)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write(path: &Path, content: &str) {
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, content).unwrap();
    }

    /// Stands in for any write path: saves the original, then writes.
    fn guarded_write(path: &Path, content: &str) -> io::Result<()> {
        before_write(path)?;
        fs::create_dir_all(path.parent().unwrap())?;
        fs::write(path, content)
    }

    #[test]
    fn no_backup_is_left_when_nothing_changes() {
        let brain = tempfile::tempdir().unwrap();
        let (res, backup) = with_backup(brain.path(), "project", || Ok::<_, io::Error>(()));
        assert!(res.is_ok());
        assert!(backup.is_none());
        assert!(!brain.path().join(BACKUPS_DIR).exists());
    }

    #[test]
    fn undo_restores_changed_files_and_removes_new_ones() {
        let brain = tempfile::tempdir().unwrap();
        let tools = tempfile::tempdir().unwrap();
        let existing = tools.path().join(".claude/CLAUDE.md");
        let created = tools.path().join(".cursor/rules/new.mdc");
        write(&existing, "my own rules");

        let (res, backup) = with_backup(brain.path(), "project", || {
            guarded_write(&existing, "generated")?;
            guarded_write(&existing, "generated twice")?;
            guarded_write(&created, "generated")
        });
        res.unwrap();
        let backup = backup.expect("a backup is recorded");
        assert_eq!(backup.entries.len(), 2, "each path is saved once");
        assert_eq!(fs::read_to_string(&existing).unwrap(), "generated twice");

        let restored = restore(brain.path(), None).unwrap();
        assert_eq!(restored.id, backup.id);
        assert!(restored.restored);
        assert_eq!(fs::read_to_string(&existing).unwrap(), "my own rules");
        assert!(!created.exists());

        // Nothing left to undo by default; the backup is still listed.
        assert!(matches!(
            restore(brain.path(), None),
            Err(BackupError::NotFound(_))
        ));
        assert_eq!(list(brain.path()).unwrap().len(), 1);
    }

    #[test]
    fn backups_are_hidden_from_brain_snapshots() {
        let brain = tempfile::tempdir().unwrap();
        let tools = tempfile::tempdir().unwrap();
        let file = tools.path().join("AGENTS.md");
        write(&file, "before");
        let (res, _) = with_backup(brain.path(), "project", || guarded_write(&file, "after"));
        res.unwrap();
        let ignore = fs::read_to_string(brain.path().join(BACKUPS_DIR).join(".gitignore")).unwrap();
        assert_eq!(ignore.trim(), "*");
    }

    #[test]
    fn nested_calls_join_the_outer_backup() {
        let brain = tempfile::tempdir().unwrap();
        let tools = tempfile::tempdir().unwrap();
        let a = tools.path().join("a.md");
        let b = tools.path().join("b.md");
        let (res, outer) = with_backup(brain.path(), "sync", || {
            guarded_write(&a, "a")?;
            let (inner_res, inner) =
                with_backup(brain.path(), "project", || guarded_write(&b, "b"));
            assert!(inner.is_none());
            inner_res
        });
        res.unwrap();
        assert_eq!(outer.unwrap().entries.len(), 2);
        assert_eq!(list(brain.path()).unwrap().len(), 1);
    }

    #[test]
    fn writes_outside_a_backup_are_not_recorded() {
        let brain = tempfile::tempdir().unwrap();
        let tools = tempfile::tempdir().unwrap();
        guarded_write(&tools.path().join("x.md"), "x").unwrap();
        assert!(list(brain.path()).unwrap().is_empty());
    }

    #[test]
    fn restore_by_id_and_unknown_id() {
        let brain = tempfile::tempdir().unwrap();
        let tools = tempfile::tempdir().unwrap();
        let file = tools.path().join("x.md");
        write(&file, "one");
        let (_, first) = with_backup(brain.path(), "project", || guarded_write(&file, "two"));
        let first = first.unwrap();
        let (_, second) = with_backup(brain.path(), "project", || guarded_write(&file, "three"));
        let second = second.unwrap();
        assert_ne!(first.id, second.id);
        let all = list(brain.path()).unwrap();
        assert_eq!(all[0].id, second.id, "newest first");

        restore(brain.path(), Some(&first.id)).unwrap();
        assert_eq!(fs::read_to_string(&file).unwrap(), "one");
        assert!(matches!(
            restore(brain.path(), Some("../../etc")),
            Err(BackupError::NotFound(_))
        ));
    }

    #[cfg(unix)]
    #[test]
    fn a_replaced_symlink_comes_back_as_a_symlink() {
        let brain = tempfile::tempdir().unwrap();
        let tools = tempfile::tempdir().unwrap();
        let target = tools.path().join("real.md");
        let link = tools.path().join(".cursorrules");
        write(&target, "real");
        std::os::unix::fs::symlink(&target, &link).unwrap();

        let (res, _) = with_backup(brain.path(), "doctor --fix", || {
            before_write(&link)?;
            fs::remove_file(&link)?;
            fs::write(&link, "plain file")
        });
        res.unwrap();
        restore(brain.path(), None).unwrap();
        let meta = fs::symlink_metadata(&link).unwrap();
        assert!(meta.file_type().is_symlink());
        assert_eq!(fs::read_link(&link).unwrap(), target);
        assert_eq!(fs::read_to_string(&target).unwrap(), "real");
    }
}
