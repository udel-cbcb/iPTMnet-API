# use alpine linux as the base
FROM fedora:27

MAINTAINER Sachinx0e

# fix for slow apt-get
# RUN echo 'Acquire::ForceIPv4 "true";' | tee /etc/apt/apt.conf.d/99force-ipv4

ENV DB_DRIVER ""
ENV DB_HOST ""
ENV DB_PORT ""
ENV DB_NAME ""
ENV DB_USER ""
ENV DB_PASSWORD ""

# copy docker_run.sh script
COPY ./docker_run.sh /home/

# copy iptmnet api executable
COPY ./iptmnet_api /home/

# copy the config file
COPY ./config.ini /home/

# copy the muscle executable
COPY ./muscle /home/ 

# run the app
WORKDIR "/home/"

# update PATH env variable so that iptmnet_api can find muscle executable
ENV PATH="/home/:${PATH}"


EXPOSE 8088
CMD ["sh", "./docker_run.sh"]
