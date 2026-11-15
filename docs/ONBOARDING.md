# Onboarding Guide — PALEXUS
### The Clinical Engine for AI Context — Four-Phase Journey

Welcome to **PALEXUS**. This guide walks you through the four phases of setting up and taking complete control of your AI tool configurations.

```mermaid
flowchart TD
    Phase1[Phase 1: The Hook\nIdentify fragmented tool configs] --> Phase2[Phase 2: The Solution\nInitialize canonical ~/AIBrain]
    Phase2 --> Phase3[Phase 3: Immediate Value\nExecute scan -> import -> project]
    Phase3 --> Phase4[Phase 4: Long-Term Power\nDaemon sync + Time Machine snapshots]
```

---

## Phase 1 — The Hook

You have more than one AI coding tool installed. Each tool keeps its own rules file, in its own format, in its own place. You've already copy-pasted the same instructions into two or three of them this month.

That's the problem PALEXUS exists to remove.

> [!WARNING]
> Maintaining separate `CLAUDE.md`, `.cursorrules`, `GEMINI.md`, `.windsurfrules`, and `.kiro/` files leads to configuration drift, stale skills, and fragmented model knowledge across your workspace.

---

## Phase 2 — The Solution

> **The Brain.** One canonical, git-backed directory — `~/AIBrain` — holding every skill, rule, agent, and MCP server you use. Every tool on your machine reads from it, directly or through a generated file. Edit once. Every model stays equally skilled.

```text
~/AIBrain/
├── skills/<slug>/          # SKILL.md + skill.yaml
├── agents/<slug>.md        # Canonical agent definitions
├── rules/                  # global.md + scoped/<glob>.md
├── memory/                 # MEMORY.md + topic files
├── prompts/                # Reusable command templates
├── mcp/servers/<id>.yaml   # Transports & env placeholders
├── .brain/                 # Active state & mappings.json
└── .git/                   # Full Git history (Time Machine)
```

---

## Phase 3 — Immediate Value (The Three-Command Loop)

```bash
# 1. Discover installed tools (read-only)
palexus scan

# 2. Ingest existing configs (dry-run first)
palexus import --dry-run
palexus import

# 3. Project Brain back out to all tools
palexus project
```

> [!TIP]
> Run `palexus scan` first. It is read-only and takes seconds. `palexus import --dry-run` shows you exactly what will move into the Brain before anything happens — expect it to report something like:
> `"Ingesting 7 skills, 6 agents, 2 rules, 3 MCP servers into ~/AIBrain"`.

From here, `palexus project` pushes the Brain's contents back out to every tool it found. That's the whole loop: **scan → import → project.**

---

## Phase 4 — Long-Term Power

Once your Brain is initialized, unlock long-term features:

| Feature | Command | Benefits |
|---|---|---|
| **Time Machine** | `palexus snapshot "msg"` | Every sync is a Git commit. Revert anytime with `palexus rollback <hash>`. |
| **Auto-Sync Daemon** | `palexus sync --daemon` | Watches the Brain and all tools; resolves drift with a 3-way merge in real time. |
| **Doctor Auto-Repair** | `palexus doctor --fix` | Diagnoses broken symlinks and config drift, repairing them automatically. |
| **Runtime Gateway** | `palexus serve` | Local reverse proxy normalizing tool-calls and executing 95% sensory gating compression. |
| **Context Retrieval** | `palexus mem <query>` | 3-layer semantic retrieval (Layer 1 search, Layer 2 timeline, Layer 3 topic get). |
| **Vital Signs HUD** | `palexus hud` | Zero-RAM live terminal cockpit displaying token burn, latency, and Brain lockstep. |

---

> [!NOTE]
> Read the full [docs/USER_GUIDE.md](USER_GUIDE.md) for complete CLI, Desktop GUI, MCP Hub, and Doctor documentation.
