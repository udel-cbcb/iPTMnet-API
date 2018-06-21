[Home](/README.md) 

# Configuring the dev environment

## Requirements ##
 1. [Rust](https://www.rust-lang.org/en-US/install.html)
 2. [Oracle Instant Client](http://www.oracle.com/technetwork/database/database-technologies/instant-client/downloads/index.html)
 3. [Visual Studio Code](https://code.visualstudio.com/Download)
 4. [vscode-rust](https://marketplace.visualstudio.com/items?itemName=kalitaalexey.vscode-rust)

    
### building the server
```
cargo build            //development version
cargo build --release  //release version
```

### running the server
```
cargo run               //development version
cargo run --release     //release version
```
In order to run the server you will have to create a `config.ini` file. The structure of config file is described [here](/doc/config.md).

## Authors
* **Sachin Gavali** 

## License
This project is the sole copyright of **University of Delaware**. The project can not be copied and/or distributed without the express permission of **University of Delaware**
