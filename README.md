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

* HRI01 - Push code to a Git server
* HRI02 - Provide non-trivial README.md
* HRI03 - Mind copyright headers
* HRI04 - Strive for codestyle consistency
* HRI05 - Use state-of-art source-code analyzers
* HRI06 - Use CI/CD pipelines
* HRI07 - Enable security-enhancing CI/CD components
* HRI08 - License compliance
* HRI09 - Use a modern build / packaging system
* HRI10 - Obey HRI01-HRI-09 😊

## Building from source

1. You will need the Rust toolchain, which consists of the compiler, package
   manager, standard libraries, and so on. Install it with this command:
   ```bash
   $ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. Then compile and launch the package:
   ```bash
   $ cargo run --release
   ```
   ```
   Compiling proc-macro2 v1.0.92
   Compiling unicode-ident v1.0.14
   Compiling libc v0.2.167
   Compiling shlex v1.3.0
   [...]
   Compiling trust v0.1.0 (/home/mstein/code/trust)
   Finished `release` profile [optimized] target(s) in 0.04s
   ```
   The final executable is: `./target/release/trust`.

## Usage

```bash
$ /path/to/trust [-h|--help] [-V|--version]
```

Without arguments, `trust` will run all the checks and display their results.
For example:
```
user@host:~/code/GitLabAdmin$ ~/code/trust/target/release/trust
2025-01-25T21:15:18.660Z INFO  [trust]
2025-01-25T21:15:18.660Z INFO  [trust] ████████╗██████╗ ██╗   ██╗███████╗████████╗
2025-01-25T21:15:18.660Z INFO  [trust] ╚══██╔══╝██╔══██╗██║   ██║██╔════╝╚══██╔══╝
2025-01-25T21:15:18.660Z INFO  [trust]    ██║   ██████╔╝██║   ██║███████╗   ██║
2025-01-25T21:15:18.660Z INFO  [trust]    ██║   ██╔══██╗██║   ██║╚════██║   ██║
2025-01-25T21:15:18.660Z INFO  [trust]    ██║   ██║  ██║╚██████╔╝███████║   ██║
2025-01-25T21:15:18.660Z INFO  [trust]    ╚═╝   ╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝
2025-01-25T21:15:18.660Z INFO  [trust]    DevSecOps Best-Practices Scanner 0.2.0
2025-01-25T21:15:18.660Z INFO  [trust]
2025-01-25T21:15:18.660Z INFO  [trust]
2025-01-25T21:15:18.660Z INFO  [trust::hri01] checking HRI01 (Push code to a Git server)
2025-01-25T21:15:18.666Z INFO  [trust::hri01] /home/mstein/code/GitLabAdmin/.git/: Git working tree found
2025-01-25T21:15:18.666Z INFO  [trust::hri01] found remote named 'origin'
2025-01-25T21:15:18.666Z INFO  [trust::hri01] HRI01 passed ✅
2025-01-25T21:15:18.666Z INFO  [trust]
2025-01-25T21:15:18.666Z INFO  [trust::hri02] checking HRI02 (Provide non-trivial README.md)
2025-01-25T21:15:18.666Z INFO  [trust::hri02] README.md: 2664 Bytes
2025-01-25T21:15:18.666Z INFO  [trust::hri02] HRI02 passed ✅
2025-01-25T21:15:18.666Z INFO  [trust]
2025-01-25T21:15:18.666Z INFO  [trust::hri03] checking HRI03 (Mind copyright headers)
[...]
```

## Author(s)

* Marijke Stein <marijke.stein@honda-ri.de>

## License

* [BSD 3-Clause License](LICENSE)

