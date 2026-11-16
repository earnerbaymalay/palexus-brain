# Identity — PALEXUS

One identity, committed. This supersedes both the Cortex / Palexus / Cerebra
three-way split in DESIGN_PACK.md and this file's own prior draft ("The
Operating Theatre"). That draft's structural rules turned out to be right —
they're kept below — but its ad-hoc palette and naming are now replaced by
the real, fully-specified brand system in `brands/palexus/`.

**Canonical references:**
- [`.ai/marketing/BRAND_BOOK.md`](.ai/marketing/BRAND_BOOK.md) — voice, palette, type, structural system
- [`brands/palexus/tokens.json`](brands/palexus/tokens.json) — machine-readable tokens
- [`brands/palexus/index.html`](brands/palexus/index.html) — visual reference, open in a browser

## The idea

The product is **Palexus** (wordmark: PALEXUS; command: `palexus`). Take
the metaphor seriously. A developer's AI tooling is not a "dashboard" — it
is a patient. One organ (the Brain at `~/AIBrain`), thirteen grafts onto it
(the tool adapters), a surgeon accountable for what they cut. `doctor`
diagnoses. `snapshot` is pre-op imaging. `rollback` reverses the operation.

## The name: Palexus and the Plexus

Two names, one anatomy. They are not competing brands; one is the product,
the other is the part of it that does the work.

- **Palexus** — the product. Read it as *pal + plexus*: a plexus that works
  for you. It is the only name on the wordmark, the app, and the command.
- **the Plexus** — the network inside Palexus. In anatomy a plexus is where
  one trunk of nerves splits, crosses, and recombines so a single source can
  reach many limbs (the brachial plexus wires one spinal column to an entire
  arm). Palexus does exactly that: one Brain, branching out to every tool.

How the anatomy maps onto the system, so copy never invents a new metaphor:

| Anatomy | Palexus | Where it lives |
|---|---|---|
| Brain | the canonical store | `~/AIBrain` |
| Trunk | a projection run | `palexus project` |
| Nerve | an adapter | `packages/core/src/adapters/` |
| Limb | an installed tool | `CLAUDE.md`, `.cursorrules`, ... |
| Reflex | a foreign edit synced back | `palexus sync` (3-way merge) |
| The Plexus | all of the above, together | the whole routing layer |

Rules for using it:

1. "the Plexus" is lowercase-*the*, capital-*P*, and always refers to the
   routing network — never to the product, the company, or the app.
2. Lead with mechanism, then the image: "One Brain. Thirteen tools. Every
   edit routed through the Plexus." — not "harness the power of the Plexus".
3. No new code names. The command stays `palexus`; there is no `plexus`
   binary, crate, or env var. The Plexus is a story, not a namespace.

So the product reads like a **clinical record**, not a SaaS console — now
rendered in PALEXUS's actual palette (ink/accent-blue/gold-rare) instead of
a placeholder one.

## What still holds, unconditionally

These are the rules that made the CLI and desktop app stop reading as
generated. They are brand-agnostic and apply regardless of which palette is
current:

1. **Every command ends by naming the next one.** No output is a dead end.
2. **Absence is reported, not hidden.** A tool that isn't installed gets a
   row saying so.
3. **Nothing is displayed that isn't measured.** No placeholder rows, no
   sample projects, no invented counts.
4. **No emoji, anywhere, ever.** Status is carried by a small fixed glyph
   set (`● ◐ ▲ ■ ○ ·`), one cell wide, first column.
5. **No cards, no rounded corners, no shadows, no gradients.** Structure
   comes from ruled lines and column alignment — PALEXUS's own version of
   this is the blueprint corner-mark system in the brand book, not a return
   to soft UI.

## What changed from the prior draft

- Palette: PALEXUS's ink/accent-blue/gold-rare/semantic-green, not the
  bone/teal "Operating Theatre" palette. Dark-only — the brand system has no
  light variant, so the app no longer follows `prefers-color-scheme`.
- Status color is no longer the same as the UI accent: "present/healthy"
  uses semantic green, the accent blue is reserved for links, buttons, and
  the wordmark — see `BRAND_BOOK.md` Do/Don't.
- Typography: three fonts (Rubik Mono One for the wordmark, Inter for UI,
  JetBrains Mono for data), vendored locally under
  `apps/desktop/src/assets/fonts/` rather than fetched from a CDN — the
  brand's own claim is "local-first, zero telemetry," so a font fetch on
  every launch would contradict the product it's branding.
- Renamed from Synapse to Palexus (Nov 2026): CLI `synapse` → `palexus`,
  crate `synapse-core` → `palexus-core`, env vars `SYNAPSE_*` →
  `PALEXUS_*`, data folder `~/.synapse` → `~/.palexus`. Existing data is
  moved on first run by `core::legacy`.

## What this still rules out

- Emoji in any surface, including commit-facing docs and CLI output.
- Placeholder or sample data rendered as if it were real.
- Gold used for anything but the marketing hero device — never a button,
  link, or status color (brand book rule, now enforced: the app's Tailwind
  config has no gold token at all).
- Rounded corners, drop shadows, glassmorphism, gradients on UI chrome.
- Exclamation marks in product copy.
