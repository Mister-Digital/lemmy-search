FROM rust:slim-bookworm AS crawler

WORKDIR /build
COPY crawler/ crawler/

RUN cargo build --manifest-path=crawler/Cargo.toml

FROM python:3-slim AS server

# Install build software
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        binutils git gcc zlib1g-dev

WORKDIR /build
COPY server/ server/

# Install user specific software
RUN pip3 install --verbose --no-warn-script-location --no-cache-dir --user \
        simplejson

# Install PyInstaller
RUN pip3 install pyinstaller --verbose --no-warn-script-location --no-cache-dir || \
    pip3 install git+https://github.com/pyinstaller/pyinstaller

RUN pyinstaller --onefile --python-option u server/server.py

FROM ubuntu:latest

WORKDIR /lemmy
COPY ui/ ui/
COPY --from=crawler /build/crawler/target/debug/lemmy-crawler bin/lemmy-crawler
COPY --from=server /build/dist/server bin/

EXPOSE 80
ENTRYPOINT [ "./bin/server" ]
