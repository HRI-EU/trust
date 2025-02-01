# HRI01 - Push code to a Git server

## Objective

A version control system serves several purposes:

* Preserve individual source code states over time
* Track or roll back changes
* Agreed "main" location
* Avoid the chaos of multiple copies + patch files
* Centralized backup

## Recommendation

1. Put the source code, documentation, and any config files relevant to
   development and execution under Git version control.
2. Do not commit build artifacts (binaries).
3. Agree on a main server location ("blessed repository").

## Performed check

The `trust` utility assumes that it is being invoked from within a Git
working tree of a software project. This working tree should have been
cloned from a main server ("blessed repository"), which by convention is
called "origin".

We use `libgit2` to determine if the current working directory belongs to
a Git working tree, and if a remote repository named "origin" has been
configured.

## Weblinks

  * [Official Git website](https://git-scm.com)
