# Trust

A minimal checker for DevSecOps best-practices.

## Why this name?

* It starts with 'T' to refer to the HRI-EU TECH Team.
* It is written in Rust.
* This tool is very essential, you can easily fool it by providing
  it with boilerplate files it is looking for. But we trust that our colleagues
  are interested in improving their workflow, not fooling a tool.

## Building from source

1. You will need the Rust toolchain, which consists of the compiler, package
   manager, standard libraries, and so on. Install it with this command:
   ```bash
   $ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. Then compile and launch the package:
   ```bash
   $ cargo run
   ```
   ```
   Compiling proc-macro2 v1.0.92
   Compiling unicode-ident v1.0.14
   Compiling libc v0.2.167
   Compiling shlex v1.3.0
   [...]
   Compiling trust v0.1.0 (/home/mstein/code/trust)
   Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.69s
   ```
   The final executable is: `./target/debug/trust`.

## Usage

```bash
$ /path/to/trust [-h|--help] [-V|--version]
```

Without arguments, `trust` will run all the checks and display their results.
For example:
```
user@host:~/code/ToolBOSLib $ cd ~/code/ToolBOSLib

user@host:~/code/ToolBOSLib $ trust
2024-12-07T17:18:14.823Z INFO  [trust]
2024-12-07T17:18:14.824Z INFO  [trust] ████████╗██████╗ ██╗   ██╗███████╗████████╗
2024-12-07T17:18:14.824Z INFO  [trust] ╚══██╔══╝██╔══██╗██║   ██║██╔════╝╚══██╔══╝
2024-12-07T17:18:14.824Z INFO  [trust]    ██║   ██████╔╝██║   ██║███████╗   ██║
2024-12-07T17:18:14.824Z INFO  [trust]    ██║   ██╔══██╗██║   ██║╚════██║   ██║
2024-12-07T17:18:14.824Z INFO  [trust]    ██║   ██║  ██║╚██████╔╝███████║   ██║
2024-12-07T17:18:14.824Z INFO  [trust]    ╚═╝   ╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝
2024-12-07T17:18:14.824Z INFO  [trust]
2024-12-07T17:18:14.824Z INFO  [trust]
2024-12-07T17:18:14.824Z INFO  [trust::hri01] checking HRI01 (Push code to a Git server)
2024-12-07T17:18:14.831Z INFO  [trust::hri01] /home/mstein/code/trust/.git/: Git working tree found
2024-12-07T17:18:14.831Z INFO  [trust::hri01] HRI01 passed ✅ (found remote named 'origin')
2024-12-07T17:18:14.831Z INFO  [trust]
2024-12-07T17:18:14.831Z INFO  [trust::hri02] checking HRI02 (Provide non-trivial README.md)
2024-12-07T17:18:14.831Z INFO  [trust::hri02] HRI02 passed ✅ (README.md: 1942 Bytes)
```

## Author(s)

* Marijke Stein <marijke.stein@honda-ri.de>

## License

* [BSD 3-Clause License](LICENSE)

