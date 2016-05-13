FROM mongo:latest
MAINTAINER Brandon Powers "bpowers1215@gmail.com"

ADD db_config /db_config
ADD init_mongoDB.sh /init_mongoDB.sh
RUN chmod +x init_mongoDB.sh;

VOLUME /data/db

ENV AUTH yes
ENV STORAGE_ENGINE wiredTiger
ENV JOURNALING yes

EXPOSE 27017 28017

CMD ["/init_mongoDB.sh"]
