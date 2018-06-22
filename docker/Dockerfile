# use alpine linux as the base
FROM fedora:28

MAINTAINER Sachinx0e

# fix for slow apt-get
# RUN echo 'Acquire::ForceIPv4 "true";' | tee /etc/apt/apt.conf.d/99force-ipv4

ENV DB_DRIVER ""
ENV DB_HOST ""
ENV DB_PORT ""
ENV DB_NAME ""
ENV DB_USER ""
ENV DB_PASSWORD ""

# install oracle drivers
COPY ./oracle/* oracle/
#RUN dnf update -y
RUN dnf install libaio -y
RUN rpm -i oracle/oracle-instantclient11.2-basic-11.2.0.4.0-1.x86_64.rpm
RUN rpm -i oracle/oracle-instantclient11.2-devel-11.2.0.4.0-1.x86_64.rpm
RUN rpm -i oracle/oracle-instantclient11.2-sqlplus-11.2.0.4.0-1.x86_64.rpm
RUN sh -c "echo /usr/lib/oracle/11.2/client64/lib > /etc/ld.so.conf.d/oracle-instantclient.conf"
RUN ldconfig

# copy docker_run.sh script
COPY ./docker_run.sh /home/

# copy iptmnet api executable
COPY ./iptmnet_api /home/

# copy the config file
COPY ./config.ini /home/

# run the app
WORKDIR "/home/"

EXPOSE 8088
CMD ["sh", "./docker_run.sh"]
