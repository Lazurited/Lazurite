<h1 align="center">
  <img src="extra/images/logo.png" width=200 height=200/><br>
  Lazurite
</h1>

<h4 align="center">A pocket IDE. Lapce, rebuilt for Android.</h4>

<div align="center">
  <a href="https://github.com/Lazurited/lazurite/actions/workflows/ci.yml" target="_blank">
    <img src="https://github.com/Lazurited/lazurite/actions/workflows/ci.yml/badge.svg" />
  </a>
  <img src="https://img.shields.io/badge/status-pre--alpha-orange" />
  <img src="https://img.shields.io/badge/license-Apache--2.0-blue" />
  <img src="https://img.shields.io/badge/platform-Android%209%2B-green" />
</div>
<br/>

Lazurite is an Android port of [Lapce](https://github.com/lapce/lapce), the
lightning-fast code editor written in pure Rust. It keeps Lapce's Rust core
intact and pairs it with a native Android port of
[Floem](https://github.com/lapce/floem), Lapce's UI framework.

The name comes from *lazurite* — the mineral that gives lapis lazuli its
deep blue. A small nod to the project it grew out of.

---

## Why Lazurite

Android has no shortage of text editors. What it lacks is a **real IDE** —
one with LSP, a project tree, a terminal, and the same editing engine you'd
use on a desktop. Most Android "IDEs" are either web wrappers around a remote
server, or toy editors with no extension model.

Lazurite takes a different approach:

- **Native, not web.** No Electron, no WebView, no browser engine. The UI is
  Floem, rendering through `wgpu` directly to Vulkan or GLES 3.x.
- **The real Lapce core.** Tree-sitter highlighting, the same Rope Science
  text engine, the same LSP integration, the same plugin model. Not a
  reimplementation.
- **Built for hardware keyboards.** A tablet with a keyboard case is a
  workstation. Lazurite treats it like one — pointer capture, IME bridging,
  and a trackpad path that doesn't lose motion samples.
- **Permissive license.** Lapce is Apache-2.0. Unlike some editor ports,
  Lazurite can be forked, extended, and shipped without copyleft strings.

---

## Features

Features marked `[done]` are inherited from Lapce and work today on desktop.
Features marked `[planned]` are planned for the Android build.

- `[done]` **Built-in LSP support** — completion, diagnostics, code actions
- `[done]` **Modal editing** — Vim-like, toggleable, first-class
- `[done]` **Tree-sitter syntax highlighting** — for every language Lapce supports
- `[done]` **WASI plugin system** — write plugins in C, Rust, or AssemblyScript
- `[done]` **Remote development** — connect to a remote Lapce proxy over SSH
- `[planned]` **Native Floem UI on Android** — via the `floem-winit-android` backend
- `[planned]` **Vulkan + GLES 3.x rendering** — dual-backend through `wgpu`
- `[planned]` **Embedded terminal via libghostty-vt** — zero-dependency VT parsing, no full Linux userland required
- `[planned]` **Hardware keyboard + trackpad support**
- `[planned]` **Custom IME** for soft-keyboard input
- `[planned]` **SAF file picker integration** for external storage

---

## Download

> **Status: pre-alpha.** There is no installable APK yet. The
> `lazurite-proxy` backend compiles and runs on `aarch64-linux-android`,
> but the Floem Android UI is still in development.

When ready, releases will be published on the
[Releases page](https://github.com/Lazurited/lazurite/releases) as signed
APKs. Distribution will be **sideload-only**, since the backend runs
subprocesses that Google Play does not permit.

For build-from-source instructions, see
[`docs/building.md`](docs/building.md).

---

## Contributing

Lazurite is a fork of Lapce with a deliberate divergence: the Android build
path. Contributions are welcome in three areas:

1. **`floem-android`** — the UI backend for Android. This is the critical
   path and the least explored.
2. **Terminal integration** — binding `libghostty-vt` into Floem's widget
   tree and wiring the PTY layer.
3. **Android platform integration** — SAF, clipboard, IME, notifications,
   lifecycle.

Before opening a PR, read [`CONTRIBUTING.md`](CONTRIBUTING.md). For
significant work, open an issue first so we can discuss the approach.

---

## Special Thanks

- **[Lapce](https://github.com/lapce/lapce)** — for the editor, the Rope
  engine, the LSP integration, and the choice of an Apache-2.0 license.
  Lazurite would not exist without it.
- **[Floem](https://github.com/lapce/floem)** — for the reactive UI
  framework that makes porting Lapce feasible in the first place.
- **[Ghostty](https://github.com/ghostty-org/ghostty)** — for `libghostty-vt`,
  the zero-dependency terminal emulation library that powers Lazurite's
  embedded terminal. Mitchell Hashimoto's vision of an embeddable terminal
  core is exactly what a mobile IDE needs.
- **[Zdroid](https://github.com/Dylanmurzello/zed-android-port)** — for
  demonstrating that a GPU-accelerated Rust editor can run natively on
  Android. Several of Lazurite's platform decisions follow their lead.
- **[winit](https://github.com/rust-windowing/winit)** — for the
  `winit-android` backend that the Lapce fork carries forward.

---

## Credits

Lazurite is a derivative work of Lapce.

```

Lapce
Copyright 2022 Dongdong Zhou and other Lapce contributors
Licensed under the Apache License, Version 2.0

Lazurite
Copyright 2026 The Lazurite Contributors
Licensed under the Apache License, Version 2.0

```

Lazurite is **not affiliated with, endorsed by, or sponsored by** the Lapce
project or its maintainers. See [`NOTICE`](NOTICE) for full upstream
attribution and the list of significant changes.

---

## License

Lazurite is released under the Apache License, Version 2.0 — the same
license as Lapce. You may use, modify, and redistribute the code under the
terms of that license. A copy of the text is included in
[`LICENSE`](LICENSE).

---

<p align="center"><em>Code anywhere. Even here.</em></p>