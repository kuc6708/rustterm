# rustterm

A clean-room, cross-platform terminal/session client written in Rust, with a
native desktop app and a browser (web) version.

> **Not affiliated with Mobatek or MobaXterm.** This project contains **no**
> MobaXterm code, assets, icons, strings, license scheme, or bundled binaries.
> See [CLEANROOM.md](CLEANROOM.md). "MobaXterm" is a trademark of its owner and
> is referenced only to describe the behavioral benchmark.

## Status

Early scaffold (M0/M1):

- [x] Native GUI shell (egui/eframe) with tabs
- [x] Local PTY terminal (VT parsing via `vt100`)
- [x] Web gateway (axum WebSocket + xterm.js) for a local PTY
- [ ] SSH transport (`russh`)
- [ ] SFTP browser
- [ ] Credentials vault
- [ ] Port forwarding / jump hosts
- [ ] X11 forwarding, VNC/RDP

## Layout

```
crates/core   shared session/config model
crates/term   PTY + VT engine (portable-pty, vt100)
crates/gui    native desktop app (eframe/egui) -> binary `rustterm`
crates/web    web gateway (axum) -> binary `rustterm-web`, serves web/
web/          xterm.js frontend (MIT)
```

## Build & run

```bash
cargo build --release --workspace

# Desktop
./target/release/rustterm

# Web (http://localhost:8080)
./target/release/rustterm-web
# override bind: RUSTTERM_BIND=127.0.0.1:9000 ./target/release/rustterm-web
```

The web frontend loads xterm.js from a CDN. For offline use, vendor
`xterm@5.3.0` and `xterm-addon-fit@0.8.0` under `web/` and update the `<script>`
tags in `web/index.html`.

## CI / cross-platform binaries

`.github/workflows/build.yml` builds and tests on **Linux, macOS, and Windows**
and uploads per-OS artifacts (`rustterm`, `rustterm-web`, web assets). Local
cross-compilation to macOS is not supported without the Apple SDK; use CI.

## Roadmap

1. SSH (password, key, agent, keyboard-interactive) + known_hosts
2. Session persistence and folders/presets
3. SFTP dual-pane browser
4. Encrypted credential vault (Argon2 + AEAD)
5. Port forwarding, jump hosts, X11 via system Xwayland
6. Macros/hotkeys/theming; optional VNC/RDP

## License

MIT. Third-party crates retain their own licenses; xterm.js is MIT.
