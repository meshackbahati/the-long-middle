# The Long Middle: Built First, Seen Last

**Written by Meshack Bahati Ouma**

> *I was a decade early. That's the loneliest kind of on time.*

---

## Path A: The Web Experience

Read the interactive web version with immersive layout, progress tracking, and theme control.

**[Read the web version](https://thelongmiddle.com)**

Features:
- Reading progress indicator
- Dark/light theme (follows system preference, manual toggle)
- Chapter navigation sidebar
- SEO-optimized, server-rendered pages
- Downloadable PDF and EPUB

### Running Locally

```bash
cd web
cargo run
# Server starts at http://127.0.0.1:8080
```

Or with the Oxidite CLI:

```bash
cd web
oxidite dev
```

---

## Path B: Read on GitHub

Read the raw manuscript directly in the `markdown-version/` folder. Each part is a standalone markdown file you can read right here on GitHub.

| # | Part | File |
|---|------|------|
| -- | Title | [00-title.md](markdown-version/00-title.md) |
| 01 | [The Fracture](markdown-version/01-the-fracture.md) | `markdown-version/01-the-fracture.md` |
| 02 | [The Kid in the Video](markdown-version/02-the-kid-in-the-video.md) | `markdown-version/02-the-kid-in-the-video.md` |
| 03 | [The Collapse and the Rise](markdown-version/03-the-collapse-and-the-rise.md) | `markdown-version/03-the-collapse-and-the-rise.md` |
| 04 | [The Script Kiddie Era](markdown-version/04-the-script-kiddie-era.md) | `markdown-version/04-the-script-kiddie-era.md` |
| 05 | [The Invisible Engineer](markdown-version/05-the-invisible-engineer.md) | `markdown-version/05-the-invisible-engineer.md` |
| 06 | [The Systems No One Saw Coming](markdown-version/06-the-systems-no-one-saw-coming.md) | `markdown-version/06-the-systems-no-one-saw-coming.md` |
| 07 | [Jarvis and the Silence](markdown-version/07-jarvis-and-the-silence.md) | `markdown-version/07-jarvis-and-the-silence.md` |
| 08 | [The Maturation](markdown-version/08-the-maturation.md) | `markdown-version/08-the-maturation.md` |
| 09 | [The Cost of Invisible Labor](markdown-version/09-the-cost-of-invisible-labor.md) | `markdown-version/09-the-cost-of-invisible-labor.md` |
| 10 | [The Rejections](markdown-version/10-the-rejections.md) | `markdown-version/10-the-rejections.md` |
| 11 | [The Break](markdown-version/11-the-break.md) | `markdown-version/11-the-break.md` |
| 12 | [The Architect in the Room](markdown-version/12-the-architect-in-the-room.md) | `markdown-version/12-the-architect-in-the-room.md` |
| 13 | [The Long Middle](markdown-version/13-the-long-middle.md) | `markdown-version/13-the-long-middle.md` |
| 14 | [The Open Question](markdown-version/14-the-open-question.md) | `markdown-version/14-the-open-question.md` |
| 15 | [A Note on What Comes Next](markdown-version/15-a-note-on-what-comes-next.md) | `markdown-version/15-a-note-on-what-comes-next.md` |

## Web Version

Read the web version online at [thelongmiddle.g24sec.com](https://thelongmiddle.g24sec.com)

---

## Downloads

- [Download PDF](web/public/downloads/the-long-middle.pdf)
- [Download EPUB](web/public/downloads/the-long-middle.epub)

---

## Projects by the Author

- **[Oxidite](https://oxidite.g24sec.com)** — Rust full-stack web framework · [crates.io](https://crates.io/crates/oxidite) · [GitHub](https://github.com/meshackbahati/rust-oxidite)
- **[Codebana](https://codebana.g24sec.com)**

## Repository Structure

```
the-long-middle/
  markdown-version/     Raw manuscript chapters (.md) for GitHub reading
  web/                  Oxidite full-stack web application
    src/                Rust source (routes, controllers, services)
    templates/          HTML templates (layout, chapters, index)
    public/             Static assets (CSS, JS, downloads)
  The Long Middle1.pdf  Source manuscript PDF
```

## Tech Stack

- **Framework**: [Oxidite](https://oxidite.g24sec.com) ([crates.io](https://crates.io/crates/oxidite)) — Rust full-stack web framework
- **Template Engine**: Oxidite native templates with `{% extends %}`, `{% include %}`, `{% for %}`, `{{ variable }}`
- **SEO**: Server-rendered HTML, JSON-LD structured data, Open Graph, Twitter Cards, semantic markup
- **Frontend**: Vanilla CSS with custom properties, vanilla JavaScript

---

*Written by Meshack Bahati Ouma. All rights reserved.*
