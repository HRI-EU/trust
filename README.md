# Trust

A minimal checker for DevSecOps best-practices.

### Why this name?

* It starts with 'T' to refer to the HRI-EU TECH Team.
* It is written in Rust.
* This tool is very essential, you can easily fool it by providing
  boilerplate files it looks for. But we trust that our colleagues
  are interested in improving their workflow rather than in fooling
  a tool.

## Building

```bash
$ cargo run
```

## Execution

```bash
$ /path/to/trust

2024-12-06T16:03:16.558Z INFO  [trust] 
2024-12-06T16:03:16.558Z INFO  [trust] ████████╗██████╗ ██╗   ██╗███████╗████████╗
2024-12-06T16:03:16.558Z INFO  [trust] ╚══██╔══╝██╔══██╗██║   ██║██╔════╝╚══██╔══╝
2024-12-06T16:03:16.558Z INFO  [trust]    ██║   ██████╔╝██║   ██║███████╗   ██║
2024-12-06T16:03:16.558Z INFO  [trust]    ██║   ██╔══██╗██║   ██║╚════██║   ██║
2024-12-06T16:03:16.558Z INFO  [trust]    ██║   ██║  ██║╚██████╔╝███████║   ██║
2024-12-06T16:03:16.558Z INFO  [trust]    ╚═╝   ╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝
2024-12-06T16:03:16.558Z INFO  [trust] 
2024-12-06T16:03:16.558Z INFO  [trust] 
2024-12-06T16:03:16.558Z INFO  [trust::hri01_git_server] checking HRI01 (Push code to a Git server)
2024-12-06T16:03:16.567Z INFO  [trust::hri01_git_server] /home/mstein/code/ToolBOSLib/.git/: Git working tree found
2024-12-06T16:03:16.567Z INFO  [trust] 
2024-12-06T16:03:16.567Z INFO  [trust::hri02_readme] checking HRI02 (Provide non-trivial README.md)
2024-12-06T16:03:16.567Z INFO  [trust::hri02_readme] size of README.md: 2628 Bytes (seems good)
```

### License

* [BSD 3-Clause License](LICENSE)
