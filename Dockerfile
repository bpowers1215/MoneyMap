FROM debian:wheezy
MAINTAINER Brandon Powers "bpowers1215@gmail.com"

# needed by cargo
ENV USER root

ADD install.sh install.sh
RUN chmod +x install.sh; sync; ./install.sh && rm install.sh

VOLUME ["/source"]
WORKDIR /source

EXPOSE 6767 27017

CMD ["cargo", "run"]
#CMD ["bash"]
