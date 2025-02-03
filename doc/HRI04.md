# HRI04 - Strive for codestyle consistency

## Objective

By incorporating a formatter into their workflow, software development teams
can improve code quality, enhance collaboration, and boost overall
productivity.

* Ensures all code within a project adheres to the same formatting rules,
  regardless of who wrote it. This improves readability and reduces cognitive
  overhead when navigating different parts of the codebase.

* Consistent formatting minimizes distractions and allows developers to focus
  on the code's functionality rather than its appearance.

* It also minimizes unnecessary changes in version control systems, making it
  easier to merge code from different developers.

## Recommendation

Use a formatter suitable for your programming language(s):

| Programming language | Tool           | Configfile                        |
|----------------------|----------------|-----------------------------------|
| C, C++, JavaScript   | `clang-format` | `.clang-format`                   |
| Python               | `ruff`         | `pyproject.toml`                  |
| Rust                 | `rustfmt`      | `rustfmt.toml` or `.rustfmt.toml` |

## Performed check

The `trust` utility assumes that it is being invoked from within the root
directory of a software project.

In this location we look for the configuration of the above tools.
At least one of the configurations must exist:

* `.clang-format` exists
* `pyproject.toml` exists and contains a table `[tool.ruff.format]`
* `rustfmt.toml` or `.rustfmt.toml` exists

## Weblinks

* [Clang-format website](https://clang.llvm.org/docs/ClangFormat.html)
* [Ruff website](https://docs.astral.sh/ruff)
* [Rustfmt website](https://github.com/rust-lang/rustfmt)

