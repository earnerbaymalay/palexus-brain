# 🔌 Tool Adapters Overview
### Palexus — 15 Verified Adapters

[Docs Hub](../README.md) > **Adapters**

Palexus includes 15 verified tool adapters, each purpose-built for bi-directional configuration translation.

---

## 📊 Supported Ecosystem Matrix

- **Claude Code / Desktop**
  - Specification: [Spec](claude.md)
  - Config Location: `CLAUDE.md`, `.claude/skills/`, `.claude/agents/`, `.mcp.json`
  - Format: Markdown, JSON
  - Import Capabilities: Skills, Agents, MCP Servers
  - Projection Target: `.claude/` structure
- **Gemini CLI**
  - Specification: [Spec](gemini.md)
  - Config Location: `GEMINI.md`, `.gemini/settings.json`
  - Format: Markdown, JSON
  - Import Capabilities: Rules, Settings
  - Projection Target: Stamped `GEMINI.md`
- **OpenAI Codex CLI**
  - Specification: [Spec](openai-codex.md)
  - Config Location: `.codex/config.toml`, `.codex/instructions.md`
  - Format: TOML, Markdown
  - Import Capabilities: Rules, Config
  - Projection Target: `AGENTS.md` + `.codex/`
- **Cursor**
  - Specification: [Spec](cursor.md)
  - Config Location: `.cursorrules`, `.cursor/rules/*.mdc`
  - Format: MDC Frontmatter
  - Import Capabilities: MDC Rules & Globs
  - Projection Target: `.cursor/rules/*.mdc`
- **Windsurf**
  - Specification: [Spec](windsurf.md)
  - Config Location: `.windsurfrules`, `mcp_config.json`
  - Format: Text, JSON
  - Import Capabilities: Rules & MCP Servers
  - Projection Target: `.windsurfrules`
- **Cline**
  - Specification: [Spec](cline.md)
  - Config Location: `.clinerules`, `cline_mcp_settings.json`
  - Format: Text, JSON
  - Import Capabilities: Custom Rules & MCP
  - Projection Target: `.clinerules`
- **Roo Code**
  - Specification: [Spec](roo-code.md)
  - Config Location: `.roomodes`, `.clinerules`
  - Format: JSON, Text
  - Import Capabilities: Mode Rules
  - Projection Target: `.roomodes`
- **Aider**
  - Specification: [Spec](aider.md)
  - Config Location: `CONVENTIONS.md`, `.aider.conf.yml`
  - Format: Markdown, YAML
  - Import Capabilities: Conventions & Config
  - Projection Target: `CONVENTIONS.md`
- **Continue**
  - Specification: [Spec](continue.md)
  - Config Location: `.continue/rules/*.md`, `.continue/config.json`
  - Format: MDC, JSON
  - Import Capabilities: MDC Rules & Config
  - Projection Target: `.continue/rules/`
- **GitHub Copilot**
  - Specification: [Spec](github-copilot.md)
  - Config Location: `.github/copilot-instructions.md`
  - Format: Markdown
  - Import Capabilities: Scoped Instructions
  - Projection Target: `.github/copilot-instructions.md`
- **Zed**
  - Specification: [Spec](zed.md)
  - Config Location: `.rules`, `.zed/settings.json`, `AGENTS.md`
  - Format: Text, JSON, Markdown
  - Import Capabilities: Settings & Rules
  - Projection Target: `.rules` + `AGENTS.md`
- **OpenCode**
  - Specification: [Spec](opencode.md)
  - Config Location: `AGENTS.md`
  - Format: Markdown
  - Import Capabilities: Multi-Agent Rules
  - Projection Target: `AGENTS.md`
- **Antigravity CLI**
  - Specification: [Spec](antigravity.md)
  - Config Location: `AGENTS.md`, `.agy/skills/`, `.gemini/settings.json`
  - Format: Markdown, YAML
  - Import Capabilities: Skills & Settings
  - Projection Target: `AGENTS.md` + `.agy/skills/`
- **AWS Kiro IDE**
  - Specification: [Spec](kiro.md)
  - Config Location: `.kiro/steering/*.md`, `.kiro/settings/mcp.json`
  - Format: Frontmatter, JSONC
  - Import Capabilities: Steering Rules & MCP
  - Projection Target: `.kiro/steering/` + `.kiro/settings/`
- **Trae IDE**
  - Specification: [Spec](trae.md)
  - Config Location: `.trae/rules/*.md`, `.trae/mcp.json`
  - Format: MDC Frontmatter, JSONC
  - Import Capabilities: Scoped Rules & MCP
  - Projection Target: `.trae/rules/` + `.trae/mcp.json`

---

## 🛠️ Authoring a Custom Adapter

Want to add support for a new AI developer tool? Read the **[Adapter Authoring Guide](../ADAPTER_AUTHORING_GUIDE.md)** for step-by-step instructions on implementing the Rust `Adapter` trait.

---

[⬅️ Back to Docs Hub](../README.md)
