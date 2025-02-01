# HRI07 - Enable security-enhancing CI/CD components

## Objective

The "DevSecOps" approach attempts to identify and mitigate security
problems early in the development process.

## Recommendation

Make use of provided CI/CD components that enhance the security:

1. [secrets-detection](https://dmz-gitlab.honda-ri.de/explore/catalog/TECH_Team/ci/secrets-detection):
   Scans the package for accidentally committed credentials such as passwords,
   SSH keys, certificates or API tokens.

## Performed check

The `trust` utility assumes that it is being invoked from within the root
directory of a software project. In this location, GitLab looks for a file
called `.gitlab-ci.yml`.

We look for the existence of a file named `.gitlab-ci.yml` and that it
includes the "secrets-detection" CI/CD component.

## Weblinks

  * [Get started with GitLab CI/CD](https://docs.gitlab.com/ee/ci)
  * HRI-EU network: [CI/CD components overview](https://dmz-gitlab.honda-ri.de/explore/catalog)
