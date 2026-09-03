# Embedded UI source

The web interface bundled in `daemon/web-content` is built from this repository:

- Source directory: `ui/`
- Build commands: `cd ui`, `npm ci`, then `npm run build`
- Generated output: `ui/dist/`
- Embedded output: `daemon/web-content/`

The Broadcast Control Surface specification lives in `ui/DESIGN.md`. The source was originally based on
`GoXLR-on-Linux/goxlr-ui`; its MIT license is retained in `ui/LICENSE`.
