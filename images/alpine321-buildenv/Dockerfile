#
#  Alpine 3.21 build environment for 'trust'
#


FROM dmz-gitlab.honda-ri.de:5050/tech_team/docker/alpine321-hri-rustc:1.0.0

USER root

RUN echo "nameserver 127.0.0.11"   >> /etc/resolv.conf && \
    echo "nameserver 192.168.65.1" >> /etc/resolv.conf && \
    echo "nameserver 172.26.0.47"  >> /etc/resolv.conf && \
    echo "nameserver 172.26.0.133" >> /etc/resolv.conf && \
    echo "nameserver 172.26.0.31"  >> /etc/resolv.conf && \
    echo "172.26.0.188 package-cache.honda-ri.de" >> /etc/hosts && \
    echo "172.26.0.188 package-cache"             >> /etc/hosts && \
    echo "https://package-cache.honda-ri.de/repository/alpine-proxy/v3.21/main" > /etc/apk/repositories

RUN apk add --no-cache --no-check-certificate --allow-untrusted \
        openssl openssl-dev pkgconfig

USER hri

