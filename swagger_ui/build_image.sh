# build the image
VERSION=1.5
IMAGE_NAME=iptmnet_api_doc
sudo docker build --no-cache . -t $IMAGE_NAME:$VERSION
docker save -o $IMAGE_NAME-$VERSION.tar.gz $IMAGE_NAME:$VERSION 
