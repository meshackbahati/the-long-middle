# the-long-middle

Web application for "The Long Middle: Built First, Seen Last" by Meshack Bahati Ouma.

Built with [Oxidite](https://oxidite.g24sec.com) ([crates.io](https://crates.io/crates/oxidite)) as a fullstack application.

## Development

```bash
oxidite dev
```

Or:

```bash
cargo run
```

Server runs at `http://127.0.0.1:8080`.

## Routes

| Path | Description |
|------|-------------|
| `/` | Landing page with chapter index |
| `/chapter/:slug` | Individual chapter reading view |
| `/downloads/*` | Static file downloads (PDF, EPUB) |
| `/css/*`, `/js/*` | Static assets |
