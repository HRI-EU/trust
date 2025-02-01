# HRI02 - Provide non-trivial README.md

## Objective

Software development collaboration platforms such as GitLab or GitHub use
the `README.md` file as main entry point for documentation. It is often
used to explain:

* Software setup
* Usage instructions
* Project status
* Authors
* Applicable license
* ...

## Recommendation

The `README.md` gives a first impression of what this package is all about.
Appreciate the user's interest and provide them with well-curated and
up-to-date information.

## Performed check

The `trust` utility assumes that it is being invoked from within the root
directory of a software project. In this location, platforms like GitLab or
GitHub would expect to find a `README.md` file.

We are looking for a file called `README.md`.
To be reasonable, it must be at least 500 bytes in size.

## Weblinks

  * [CommonMark syntax reference](https://commonmark.org)
  * [GitLab Flavored Markdown (GLFM) syntax reference](https://docs.gitlab.com/ee/user/markdown.html)
  * [GitHub Flavored Markdown (FM) syntax reference](https://github.github.com/gfm)
