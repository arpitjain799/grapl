FROM debian:bullseye-slim as build

ENV HWLOC_VERSION="2.7.1"
# At least one of the install-recommends is necessary in order for hwloc to compile.
# hadolint ignore=DL3015
RUN apt-get update \
    && apt-get -y install curl=7.74.0-1.3+deb11u3 \
    libssl-dev=1.1.1n-0+deb11u3 \
    ca-certificates=20210119  \
    gcc=4:10.2.1-1 \
    cmake=3.18.4-2+deb11u1 \
    pkg-config=0.29.2-1 && \
curl --proto "=https" \
    --tlsv1.2 \
    --location \
    --output /tmp/hwloc-${HWLOC_VERSION}.tar.gz \
"https://download.open-mpi.org/release/hwloc/v2.7/hwloc-${HWLOC_VERSION}.tar.gz" && \
tar --extract \
    --verbose \
    --ungzip \
    --directory="/tmp" \
    --file="/tmp/hwloc-${HWLOC_VERSION}.tar.gz"
WORKDIR /tmp/hwloc-${HWLOC_VERSION}
RUN ./configure && \
make && \
make install

FROM scylladb/scylla:4.6.3

COPY --from=build /usr/local/lib/libhwloc.so.15.5.3 /usr/local/lib/libhwloc.so.15.5.3
RUN rm /opt/scylladb/libreloc/libhwloc.so.15 && \
    ln -sf /usr/local/lib/libhwloc.so.15.5.3 /opt/scylladb/libreloc/libhwloc.so.15
