#!/bin/bash
VERSION=2.1.3
IMAGE_NAME=iptmnet_api

# copy the iptmnet api executable
cargo build --release
cp ./../target/release/iptmnet_api ./

# copy oracle instant client rpms
rm -r ./oracle
mkdir ./oracle
cp -r ./../oracle/* ./oracle/

# build the iptmnet api images
docker build --no-cache . -t udelcbcb/$IMAGE_NAME:$VERSION
docker push udelcbcb/$IMAGE_NAME:$VERSION

