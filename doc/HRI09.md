# HRI09 - Use a modern build / packaging system

## Objective

Automation is the key to modern software development. It occurs in:

* Programming: Linters and formatters perform an early code check
* Compiling: Building the code and including the dependencies need to be orchestrated
* Testing: Fully-automated testsuite execution
* Delivery and Deployment: Packaging the software and getting it into production

## Recommendation

To manage all of the above automation tasks, use a modern build / packaging system:

| Programming language | Tool            | Configfile                                                               | Lockfile                  |
|----------------------|-----------------|--------------------------------------------------------------------------|---------------------------|
| C, C++               | `conan`         | `conanfile.py` or `conanfile.txt`                                        | `conan.lock`              |
| Python               | `conda` or `uv` | `environment.yml` or `environment.yaml` or `pyproject.toml` or `uv.toml` | `conda.lock` or `uv.lock` |
| Rust                 | `cargo`         | `Cargo.toml`                                                             | `Cargo.lock`              |

## Performed check

The `trust` utility assumes that it is being invoked from within the root
directory of a software project.

In this location we look for a valid combination of configfile and lockfile of the above tools.
At least one of the file combinations must exist:

* `Cargo.toml` and `Cargo.lock`
* `conanfile.py` and `conan.lock`
* `conanfile.txt` and `conan.lock`
* `environment.yml` and `conda.lock`
* `environment.yaml` and `conda.lock`
* `pyproject.toml` and `uv.lock`
* `uv.toml` and `uv.lock`

## Weblinks

* [Conan website](https://conan.io)
* [Conda website](https://docs.conda.io)
* [Cargo website](https://doc.rust-lang.org/cargo)
* [Uv website](https://docs.astral.sh/uv)
