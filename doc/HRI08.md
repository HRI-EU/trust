# HRI08 - License compliance

## Objective

Software projects must not infringe the copyright or violate the license
terms and conditions of third parties. Failure to ensure legal correctness
may result in lawsuits and/or negative publicity for the organization with
unpredictable material or immaterial damages.

## Recommendation

Make use of provided CI/CD components that perform legal scans:

1. [AutoCompliance](https://dmz-gitlab.honda-ri.de/explore/catalog/TECH_Team/ci/autocompliance):
   Software Composition Analysis (SCA) and basic legal clearance

## Performed check

The `trust` utility assumes that it is being invoked from within the root
directory of a software project. In this location, GitLab looks for a file
called `.gitlab-ci.yml`.

We look for the existence of a file named `.gitlab-ci.yml` and that it
includes the "AutoCompliance" CI/CD component.

## Weblinks

  * [Get started with GitLab CI/CD](https://docs.gitlab.com/ee/ci)
  * HRI-EU network: [CI/CD components overview](https://dmz-gitlab.honda-ri.de/explore/catalog)
