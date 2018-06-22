#!/bin/sh

# replace the vraiables in config file
sed -i "s|db_driver|${DB_DRIVER}|g" /home/config.ini
sed -i "s|db_host|${DB_HOST}|g" /home/config.ini
sed -i "s|db_port|${DB_PORT}|g" /home/config.ini
sed -i "s|db_name|${DB_NAME}|g" /home/config.ini
sed -i "s|db_user|${DB_USER}|g" /home/config.ini
sed -i "s|db_password|${DB_PASSWORD}|g" /home/config.ini

#start the iptmnet server
./iptmnet_api
