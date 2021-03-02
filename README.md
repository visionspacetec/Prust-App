# Prust-RTOS
Main RTOS task entry point for [Prust-RTOS-Gen](https://github.com/visionspacetec/Prust-RTOS-Gen). This is a static Rust crate. This is a sub-module of Prust-RTOS-Gen and it depends on the external functions defined in it.  Can't be built seperately...
# Usage
This is a seperate repo because it has **only rust code** except the c headers. This crate can be potentially used in other projects too. **Only the Drivers glue code is dependent on the device**. 

So you can use this repo as a template and by properly written glue code you should be good to go.

## NOTE: Can't be built seperately run above lines after cloning [Prust-RTOS-Gen](https://github.com/visionspacetec/Prust-RTOS-Gen)
```
rustup default nightly   
# sudo apt install build-essential  
cargo install xargo  
rustup component add rust-src  
rustup target add thumbv7em-none-eabihf  
xargo build --target thumbv7em-none-eabihf  # this is the compile, command run this when the prust code changes
```

