<p align="center">
  <a href="https://github.com/bobis33/verrs/blob/main/assets/icons/icon.png">
    <img src="assets/icons/icon.png" alt="Logo" width="200" height="200">
  </a>

  <h1 align="center">VERS</h1>

  <p align="center">
    Vulkan based engine.
  </p>

<p align="center">
  <a href="https://rust-lang.org/"><img src="https://img.shields.io/badge/Rust-1.93.0-orange" alt="Rust 1.93.0"/></a>
</p>

---

```text
crates/
├── engine-core/        → lib  (maths, types fondamentaux, ECS...)
├── engine-render/      → lib  (wgpu, pipelines, materials...)
├── engine-audio/       → lib
├── engine-platform/    → lib  (fenêtre, input, OS...)
├── engine-asset/       → lib  (chargement, hot-reload...)
├── asset-compiler/     → bin  (outil de build, exclu en release)
└── engine/             → lib  (crate facade, re-exporte tout)
```

## Prerequisites
- Rust 1.93+ ([install](https://rustup.rs/))

# Usage
Run the examples: <br>
`cargo run --example [ example_to_run ]`
> run `cargo run --example` to list available examples

Run the tests: <br>
`cargo test`

Run the docs: <br>
`cargo doc`

Run the benches: <br>
`cargo bench`

Lint project: <br>
`cargo clippy`

Format project: <br>
`cargo fmt`

## Security
See the [security policy](https://github.com/OxideSlicer/OxideSlicer/blob/main/SECURITY.md) for more information.

## Contributing
Want to contribute? See [contributing guidelines](https://github.com/OxideSlicer/OxideSlicer/blob/main/CONTRIBUTING.md).

Please read our [code of conduct](https://github.com/OxideSlicer/OxideSlicer/blob/main/CODE_OF_CONDUCT.md) before contributing to this project.

## License
This project is licensed under the **MIT license**. See the [license](https://github.com/OxideSlicer/OxideSlicer/blob/main/LICENSE) file for details.

