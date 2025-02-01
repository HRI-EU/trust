# HRI03 - Put copyright headers and license file(s)

## Objective

1. Although not legally binding, it is very common to place a boilerplate
   header at the top of every source file. This makes it clear to the reader
   whether the file is copyrighted, and briefly states what license applies
   to the file.

2. The "End-User License Agreement" ("EULA", or simply "License") describes
   the terms and conditions in detail.

3. For tool-assisted license assessment, it is highly recommended to include
   the corresponding SPDX license identifier in each source file.

## Recommendation

You need to make an informed decision about what the recipient should (or
should not) do with this software, and what you want to protect. You can
provide it under a very strict license, or you can protect the freedom to
share and distribute knowledge (copyleft or permissive open source licenses).

While it is technically possible, it is not recommended to have source files
with multiple sections under different copyrights and/or licences, to avoid
errors.

Source code files and related artifacts, such as scripts or config files, must
begin with a copyright header.

A range of years (such as 2024-2025) is used to indicate the years in which the
active development took place and therefore when copyright was claimed.

The standard corporate copyright header includes the copyright notice, the
company address, and the SPDX-License-ID. It is the default for all work
created for the company. Typical copyright headers:

  * [HRI-EU corporate copyright header](Copyright-HRI.md)
  * [BSD 3-Clause copyright header](Copyright-BSD3Clause.md)

The license file should be named `LICENSE` in uppercase letters with the
appropriate extension, e.g. `LICENSE.txt`.
  * [HRI-EU Software License 1.0 (*.txt)](HRI-EndUserLicenseAgreement-1.0.txt)
  * [BSD 3-Clause License (*.txt)](LICENSE-BSD3Clause.txt)

Other headers and licenses may be used in the process of publishing the work
results as open source (after written approval).

Copyright headers can be omitted in automatically generated files, or in
files that follow a syntax that does not support commented-out lines.

## Performed check

The `trust` utility assumes that it is being invoked from within the root
directory of a software project. It expects to find a `src` subdirectory in
that location.

We recursively search in `./src` for files with the following extensions:
`bash` `c` `cpp` `go` `h` `hpp` `hxx` `inc` `js` `m` `py` `rs` `sh` `ts` `tsx`.

Each of these files is checked for at least one of the following strings
(case-insensitive): `copyright`, `(c)`, `©`. In addition, each of these
files must contain the string `SPDX-License-Identifier:`.

## Weblinks

  * HRI-EU network: [HRI-EU Software License 1.0 (various file formats)](https://dmz-gitlab.honda-ri.de/TECH_Team/EULA)
  * [SPDX Licenses overview](https://spdx.org/licenses)
