#
#  Ubuntu 22.04 build environment for 'trust'
#


FROM dmz-gitlab.honda-ri.de:5050/tech_team/docker/ubuntu2204-hri-rustc:latest

USER root

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
            libssl-dev pkg-config && \
     rm -rf /var/lib/apt/lists/*

USER hri


# EOF
