FROM ubuntu@sha256:ca5534a51dd04bbcebe9b23ba05f389466cf0c190f1f8f182d7eea92a9671d00

RUN apt update && apt upgrade -y && apt autoremove -y&& apt clean;

RUN useradd no_root --create-home;
WORKDIR /home/no_root

#USER no_root
#CMD ["/home/no_root/compiled/debug/ftp"]