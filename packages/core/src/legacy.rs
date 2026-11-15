//! Carries data over from the pre-rename "Synapse" names, so upgrading never
//! strands a user's memory, logs or master prompt.
//!
//! Each helper moves the old file or folder to its new name the first time it
//! is needed. A rename is reversible and keeps every byte. If the move fails
//! (permissions, a different filesystem), the old location keeps being used,
//! so nothing goes missing.

use crate::master_prompt::MASTER_PROMPT_FILENAME;
use std::fs;
use std::path::{Path, PathBuf};

/// Per-user data folder name before the rename.
pub const LEGACY_DATA_DIR: &str = ".synapse";
/// Master prompt file name before the rename.
pub const LEGACY_MASTER_PROMPT: &str = "SYNAPSE.md";

/// `~/.palexus`, moving a pre-rename `~/.synapse` there on first use.
pub fn data_dir(home: &Path) -> PathBuf {
    adopt(home.join(".palexus"), home.join(LEGACY_DATA_DIR))
}

/// `<brain>/PALEXUS.md`, moving a pre-rename `SYNAPSE.md` there on first use.
pub fn master_prompt_path(brain_dir: &Path) -> PathBuf {
    adopt(
        brain_dir.join(MASTER_PROMPT_FILENAME),
        brain_dir.join(LEGACY_MASTER_PROMPT),
    )
}

fn adopt(new: PathBuf, old: PathBuf) -> PathBuf {
    if fs::symlink_metadata(&new).is_ok() || fs::symlink_metadata(&old).is_err() {
        return new;
    }
    match fs::rename(&old, &new) {
        Ok(()) => new,
        Err(_) => old,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moves_the_old_data_folder_and_keeps_its_contents() {
        let home = tempfile::tempdir().unwrap();
        fs::create_dir_all(home.path().join(".synapse/memory")).unwrap();
        fs::write(home.path().join(".synapse/memory/observations.jsonl"), "{}\n").unwrap();

        let dir = data_dir(home.path());
        assert_eq!(dir, home.path().join(".palexus"));
        assert_eq!(
            fs::read_to_string(dir.join("memory/observations.jsonl")).unwrap(),
            "{}\n"
        );
        assert!(!home.path().join(".synapse").exists());
    }

    #[test]
    fn never_overwrites_an_existing_new_folder() {
        let home = tempfile::tempdir().unwrap();
        fs::create_dir_all(home.path().join(".synapse")).unwrap();
        fs::write(home.path().join(".synapse/old"), "old").unwrap();
        fs::create_dir_all(home.path().join(".palexus")).unwrap();

        assert_eq!(data_dir(home.path()), home.path().join(".palexus"));
        // The old folder is left alone for the user to merge by hand.
        assert!(home.path().join(".synapse/old").exists());
    }

    #[test]
    fn fresh_install_uses_the_new_name() {
        let home = tempfile::tempdir().unwrap();
        assert_eq!(data_dir(home.path()), home.path().join(".palexus"));
        assert!(!home.path().join(".palexus").exists());
    }

    #[test]
    fn moves_a_pre_rename_master_prompt() {
        let brain = tempfile::tempdir().unwrap();
        fs::write(brain.path().join("SYNAPSE.md"), "zone 2 notes").unwrap();

        let path = master_prompt_path(brain.path());
        assert_eq!(path, brain.path().join("PALEXUS.md"));
        assert_eq!(fs::read_to_string(path).unwrap(), "zone 2 notes");
    }
}
