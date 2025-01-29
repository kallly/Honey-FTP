FROM rust@sha256:0ec205a9abb049604cb085f2fdf7630f1a31dad1f7ad4986154a56501fb7ca77

RUN useradd no_root
USER no_root

WORKDIR /usr/src/ftp
COPY src .

RUN cargo install --path .

CMD ["ftp"]
