#!/bin/bash
VERSION=1.1.9
IMAGE_NAME=udelcbcb/iptmnet_api

# copy the iptmnet api executable
cargo build --release
cp ./../target/release/iptmnet_api ./

# copy oracle instant client rpms
rm -r ./oracle
mkdir ./oracle
cp -r ./../oracle/* ./oracle/

# build the iptmnet api images
docker build --no-cache . -t $IMAGE_NAME:$VERSION
docker save -o iptmnet_api-$VERSION.tar.gz $IMAGE_NAME:$VERSION 

