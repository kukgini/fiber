FROM ghcr.io/hyperledger/aries-vcx/libvcx:0.24.1

USER root

ARG UID=1001
ARG GID=1001

RUN addgroup -g $GID agent && adduser -u $UID -D -G agent agent

USER agent

ENV X86_64_UNKNOWN_LINUX_MUSL_OPENSSL_NO_VENDOR true
RUN rustup default stable
