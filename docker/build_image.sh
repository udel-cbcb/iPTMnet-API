# copy the iptmnet api executable
cargo build --release
cp ./../target/release/iptmnet_api ./

# copy the static assets
rm -r ./static
mkdir ./static
cp -r ./../static/* ./static

# copy oracle instant client rpms
rm -r ./oracle
mkdir ./oracle
cp -r ./../oracle/* ./oracle/

# build the iptmnet api images
docker build --no-cache . -t udelcbcb/iptmnet_api:1.1.8

