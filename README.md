# Trust

A minimal checker for DevSecOps best-practices.

### Why this name?

* It starts with 'T' to refer to the HRI-EU TECH Team.
* It is written in Rust.
* This tool is very essential, you can easily fool it by providing
  it with boilerplate files it is looking for. But we trust that our colleagues
  are interested in improving their workflow, not fooling a tool.

## Building

1. You will need the Rust toolchain, which consists of the compiler, package
   manager, standard libraries, and so on. Install it with this command:
   ```bash
   $ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. Then compile the package:
   ```bash
   $ cargo run
   ```

## Execution

```bash
$ /path/to/trust

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

### License

* [BSD 3-Clause License](LICENSE)

