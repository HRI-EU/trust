#
#  Alpine 3.21 runtime environment for 'trust'
#


FROM dmz-gitlab.honda-ri.de:5050/tech_team/docker/alpine321-hri-minimal:1.0.0

USER root

RUN echo "nameserver 127.0.0.11"   >> /etc/resolv.conf && \
    echo "nameserver 192.168.65.1" >> /etc/resolv.conf && \
    echo "nameserver 172.26.0.47"  >> /etc/resolv.conf && \
    echo "nameserver 172.26.0.133" >> /etc/resolv.conf && \
    echo "nameserver 172.26.0.31"  >> /etc/resolv.conf && \
    echo "172.26.0.188 package-cache.honda-ri.de" >> /etc/hosts && \
    echo "172.26.0.188 package-cache"             >> /etc/hosts && \
    echo "https://package-cache.honda-ri.de/repository/alpine-proxy/v3.21/main" > /etc/apk/repositories && \
    echo "https://package-cache.honda-ri.de/repository/alpine-proxy/v3.21/community" >> /etc/apk/repositories

RUN apk add --no-cache --no-check-certificate --allow-untrusted --update \
        git libgcc openssl

RUN mkdir -p /opt/hri/trust/bin
COPY ./target/release/trust /opt/hri/trust/bin
RUN chmod 0755 /opt/hri/trust/bin && \
    chown -R hri /opt/hri && \
    chgrp -R hri /opt/hri

USER hri

# Newer Git versions complain if the current user (who invokes git on commandline)
# is not the owner of the working tree, leading to this error:
# "repository path ... is not owned by current user; class=Config (7); code=Owner (-36)"
RUN git config --global --add safe.directory '*'

