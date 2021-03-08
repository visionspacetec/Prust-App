# Prust-App
Main RTOS task entry point for [Prust-FreeRTOS](https://github.com/visionspacetec/Prust-FreeRTOS). This is a static Rust crate. This is a sub-module of Prust-RTOS-Gen and it depends on the external functions defined in it.  Can't be built seperately...
# Usage
This is a seperate repo because it has **only rust code** except the c headers. This crate can be potentially used in other projects too. **Only the Drivers glue code is dependent on the device**. 

So you can use this repo as a template and by properly written glue code you should be good to go.

# Requirements
```
rustup default nightly   
# sudo apt install build-essential  
# cargo install xargo  
rustup component add rust-src  
rustup target add thumbv7em-none-eabihf
```
# Building

```
cargo build
```
## NOTE: Can't be linked seperately, see [Prust-FreeRTOS](https://github.com/visionspacetec/Prust-FreeRTOS)
