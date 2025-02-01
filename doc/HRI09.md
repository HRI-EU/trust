# HRI09 - Use a modern build / packaging system

## Objective

Automation is the key to modern software development. It occurs in:

* Programming: Linters and formatters perform an early code check
* Compiling: Building the code and including the dependencies need to be orchestrated
* Testing: Fully-automated testsuite execution
* Delivery and Deployment: Packaging the software and getting it into production

## Recommendation

To manage all of the above automation tasks, use a modern build / packaging system:

| Programming language | Tool  | Configfile                        | Lockfile     |
|----------------------|-------|-----------------------------------|--------------|
| C, C++               | Conan | `conanfile.py` or `conanfile.txt` | `conan.lock` |
| Python               | Uv    | `pyproject.toml` or `uv.toml`     | `uv.lock`    |
| Rust                 | Cargo | `Cargo.toml`                      | `Cargo.lock` |

## Performed check

The `trust` utility assumes that it is being invoked from within the root
directory of a software project.

In this location we look for a valid combination of configfile and lockfile of the above tools.
At least one of the file combinations must exist:

* `conanfile.py` and `conan.lock`
* `conanfile.txt` and `conan.lock`
* `pyproject.toml` and `uv.lock`
* `uv.toml` and `uv.lock`
* `Cargo.toml` and `Cargo.lock`

## Weblinks

* [Conan website](https://conan.io)
* [Cargo website](https://doc.rust-lang.org/cargo)
* [Uv website](https://docs.astral.sh/uv)
