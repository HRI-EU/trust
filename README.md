![image](doc/trust-banner.jpg)

# Trust

A minimal checker for DevSecOps best-practices.

## Why this name?

* It starts with 'T' to refer to the TECH Team at [HRI-EU](https://www.honda-ri.de).
* It is written in Rust.
* This tool is very essential, you can easily fool it by providing
  it with boilerplate files it is looking for. But we trust that our colleagues
  are interested in improving their workflow, not fooling a tool.

## Checked objectives

* [HRI01](doc/HRI01.md) - Push code to a Git server
* [HRI02](doc/HRI02.md) - Provide non-trivial README.md
* [HRI03](doc/HRI03.md) - Put copyright headers and license file(s)
* [HRI04](doc/HRI04.md) - Strive for codestyle consistency
* [HRI05](doc/HRI05.md) - Use state-of-art source-code analyzers
* [HRI06](doc/HRI06.md) - Use CI/CD pipelines
* [HRI07](doc/HRI07.md) - Enable security-enhancing CI/CD components
* [HRI08](doc/HRI08.md) - License compliance
* [HRI09](doc/HRI09.md) - Use a modern build / packaging system
* [HRI10](doc/HRI10.md) - Obey HRI01-HRI-09 😊

## Building from source

1. You will need the Rust toolchain, which consists of the compiler, package
   manager, standard libraries, and so on. Install it with this command:
   ```bash
   $ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. Then compile and launch the package:
   ```bash
   $ cargo run --release
   [...]
   Compiling trust v0.9.0 (/home/mstein/code/trust)
   Finished `release` profile [optimized] target(s) in 0.21s
   ```
   The final executable is: `./target/release/trust`.

## Usage

```bash
$ /path/to/trust [-h|--help] [-V|--version]
```

Without arguments, `trust` will run all the checks and display their results.
For example:
```
2025-02-18T08:12:31.854Z INFO  [trust]
2025-02-18T08:12:31.854Z INFO  [trust] ████████╗██████╗ ██╗   ██╗███████╗████████╗
2025-02-18T08:12:31.854Z INFO  [trust] ╚══██╔══╝██╔══██╗██║   ██║██╔════╝╚══██╔══╝
2025-02-18T08:12:31.854Z INFO  [trust]    ██║   ██████╔╝██║   ██║███████╗   ██║
2025-02-18T08:12:31.854Z INFO  [trust]    ██║   ██╔══██╗██║   ██║╚════██║   ██║
2025-02-18T08:12:31.854Z INFO  [trust]    ██║   ██║  ██║╚██████╔╝███████║   ██║
2025-02-18T08:12:31.854Z INFO  [trust]    ╚═╝   ╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝
2025-02-18T08:12:31.854Z INFO  [trust]    DevSecOps Best-Practices Scanner 1.0.0
2025-02-18T08:12:31.854Z INFO  [trust]
2025-02-18T08:12:31.854Z INFO  [trust]
2025-02-18T08:12:31.854Z INFO  [trust::hri01] checking HRI01 (Push code to a Git server)
2025-02-18T08:12:31.860Z INFO  [trust::hri01] /home/mstein/code/trust/.git/: Git working tree found
2025-02-18T08:12:31.860Z INFO  [trust::hri01] found remote named 'origin'
2025-02-18T08:12:31.860Z INFO  [trust::hri01] HRI01 passed ✅
2025-02-18T08:12:31.860Z INFO  [trust]
2025-02-18T08:12:31.860Z INFO  [trust::hri02] checking HRI02 (Provide non-trivial README.md)
2025-02-18T08:12:31.860Z INFO  [trust::hri02] README.md: 3713 Bytes
2025-02-18T08:12:31.860Z INFO  [trust::hri02] HRI02 passed ✅
2025-02-18T08:12:31.860Z INFO  [trust]
2025-02-18T08:12:31.860Z INFO  [trust::hri03] checking HRI03 (Put copyright headers and license files)
2025-02-18T08:12:31.861Z INFO  [trust::hri03] copyright and SPDX info found: src/hri01.rs
2025-02-18T08:12:31.861Z INFO  [trust::hri03] copyright and SPDX info found: src/hri02.rs
2025-02-18T08:12:31.861Z INFO  [trust::hri03] copyright and SPDX info found: src/hri03.rs
2025-02-18T08:12:31.861Z INFO  [trust::hri03] copyright and SPDX info found: src/hri04.rs
2025-02-18T08:12:31.861Z INFO  [trust::hri03] copyright and SPDX info found: src/hri05.rs
2025-02-18T08:12:31.861Z INFO  [trust::hri03] copyright and SPDX info found: src/hri06.rs
2025-02-18T08:12:31.861Z INFO  [trust::hri03] copyright and SPDX info found: src/hri07.rs
2025-02-18T08:12:31.861Z INFO  [trust::hri03] copyright and SPDX info found: src/hri08.rs
2025-02-18T08:12:31.861Z INFO  [trust::hri03] copyright and SPDX info found: src/hri09.rs
2025-02-18T08:12:31.861Z INFO  [trust::hri03] copyright and SPDX info found: src/hri10.rs
2025-02-18T08:12:31.861Z INFO  [trust::hri03] copyright and SPDX info found: src/main.rs
2025-02-18T08:12:31.861Z INFO  [trust::hri03] HRI03 passed ✅
2025-02-18T08:12:31.861Z INFO  [trust]
2025-02-18T08:12:31.861Z INFO  [trust::hri04] checking HRI04 (Strive for codestyle-consistency)
2025-02-18T08:12:31.861Z INFO  [trust::hri04] found rustfmt.toml
2025-02-18T08:12:31.861Z INFO  [trust::hri04] HRI04 passed ✅
2025-02-18T08:12:31.861Z INFO  [trust]
2025-02-18T08:12:31.861Z INFO  [trust::hri05] checking HRI05 (Use static source-code analyzers)
2025-02-18T08:12:31.861Z INFO  [trust::hri05] found clippy.toml
2025-02-18T08:12:31.861Z INFO  [trust::hri05] HRI05 passed ✅
2025-02-18T08:12:31.861Z INFO  [trust]
2025-02-18T08:12:31.861Z INFO  [trust::hri06] checking HRI06 (Use CI/CD pipelines)
2025-02-18T08:12:31.861Z INFO  [trust::hri06] GitLab CI/CD pipeline found: .gitlab-ci.yml
2025-02-18T08:12:31.861Z INFO  [trust::hri06] HRI06 passed ✅
2025-02-18T08:12:31.861Z INFO  [trust]
2025-02-18T08:12:31.861Z INFO  [trust::hri07] checking HRI07 (Use security-components)
2025-02-18T08:12:31.862Z INFO  [trust::hri07] found: secrets-detection@~latest
2025-02-18T08:12:31.862Z INFO  [trust::hri07] HRI07 passed ✅
2025-02-18T08:12:31.862Z INFO  [trust]
2025-02-18T08:12:31.862Z INFO  [trust::hri08] checking HRI08 (License compliance)
2025-02-18T08:12:31.862Z INFO  [trust::hri08] found: autocompliance@~latest
2025-02-18T08:12:31.862Z INFO  [trust::hri08] HRI08 passed ✅
2025-02-18T08:12:31.862Z INFO  [trust]
2025-02-18T08:12:31.862Z INFO  [trust::hri09] checking HRI09 (Use modern build/packaging system)
2025-02-18T08:12:31.862Z INFO  [trust::hri09] found Cargo.toml
2025-02-18T08:12:31.862Z INFO  [trust::hri09] found Cargo.lock
2025-02-18T08:12:31.862Z INFO  [trust::hri09] HRI09 passed ✅
2025-02-18T08:12:31.862Z INFO  [trust]
2025-02-18T08:12:31.862Z INFO  [trust::hri10] checking HRI10 (Obey HRI01-HRI-09 😊)
2025-02-18T08:12:31.862Z INFO  [trust::hri10] summary: HRI01 passed ✅
2025-02-18T08:12:31.862Z INFO  [trust::hri10] summary: HRI02 passed ✅
2025-02-18T08:12:31.862Z INFO  [trust::hri10] summary: HRI03 passed ✅
2025-02-18T08:12:31.862Z INFO  [trust::hri10] summary: HRI04 passed ✅
2025-02-18T08:12:31.862Z INFO  [trust::hri10] summary: HRI05 passed ✅
2025-02-18T08:12:31.862Z INFO  [trust::hri10] summary: HRI06 passed ✅
2025-02-18T08:12:31.862Z INFO  [trust::hri10] summary: HRI07 passed ✅
2025-02-18T08:12:31.862Z INFO  [trust::hri10] summary: HRI08 passed ✅
2025-02-18T08:12:31.862Z INFO  [trust::hri10] summary: HRI09 passed ✅
2025-02-18T08:12:31.862Z INFO  [trust::hri10] summary: HRI10 passed ✅
```

## Author(s)

* Marijke Stein <marijke.stein@honda-ri.de>

## License

* [BSD 3-Clause License](LICENSE)

