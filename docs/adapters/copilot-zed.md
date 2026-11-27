# GitHub Copilot & Zed Adapter Specifications
### Palexus

[Docs Hub](../README.md) > [Adapters](README.md) > **Copilot & Zed**

---

## 📋 Overview

- **Adapter ID**
  - GitHub Copilot: `github-copilot`
  - Zed: `zed`
- **Native Configs**
  - GitHub Copilot: `.github/copilot-instructions.md`
  - Zed: `.rules`, `.zed/settings.json`, `AGENTS.md`
- **Parsing Engine**
  - GitHub Copilot: Markdown instruction parser
  - Zed: Markdown + JSON settings parser
- **Projection Target**
  - GitHub Copilot: `.github/copilot-instructions.md`
  - Zed: `.rules` + `AGENTS.md`
- **Symlink Support**
  - GitHub Copilot: Direct symlink supported
  - Zed: Direct symlink for `.rules`

---

## 📥 Ingestion & Projection Strategy

- **Copilot**: Ingests workspace instructions from `.github/copilot-instructions.md`.
- **Zed**: Ingests rules from `.rules` and multi-agent directives from `AGENTS.md`.

---

[⬅️ Back to Adapters Overview](README.md)
