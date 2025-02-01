# HRI06 - Use CI/CD pipelines

## Objective

Automation is the key to modern software development. It supports:

* Ensuring that all relevant source code and configuration files are properly committed
* Repeated execution of test and quality assurance programs (Continuous Integration)
* Creation of release packages (Continuous Delivery)
* Automated deployment to production (Continuous Deployment)

## Recommendation

1. Use CI/CD pipelines
2. Make informed decisions about what to execute within the pipeline
3. Be mindful of IT resources: Do not configure pipelines if no one will use their results

## Performed check

The `trust` utility assumes that it is being invoked from within the root
directory of a software project. In this location, GitLab looks for a file
called `.gitlab-ci.yml`.

For now, we are only looking for the existence of a file named `.gitlab-ci.yml`.

## Weblinks

  * [Get started with GitLab CI/CD](https://docs.gitlab.com/ee/ci)
  * HRI-EU network: [CI/CD components overview](https://dmz-gitlab.honda-ri.de/explore/catalog)
