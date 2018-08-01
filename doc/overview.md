[Home](/README.md) 

# Overview
This project holds the code for iPTMnet API. The project uses [Rust](https://www.rust-lang.org/en-US/) and [Actix-Web](https://github.com/actix/actix-web) REST Api framework. 

The entry point for the project is `main.rs`. The routes are defined in the `router.rs` file. The `docker` folder contains the `Dockerfile` to build the docker image. Prebuilt images are available at the [udelcbcb](https://hub.docker.com/u/udelcbcb/) docker hub repository.   

The `iptmnet_api_test` folder contains the test for the api. The `responses` folder under `test` contains the model responses against which the actual results of the test are compared. The test are written in `python-3` and follow the black box approach to testing.  

