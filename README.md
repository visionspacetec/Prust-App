# Prust-RTOS
Main RTOS task entry point for Prust. A static Rust crate. This is a sub-module of Prust-RTOS-Gen and it depends on the external functions defined in it.

## NOTE: Can't be built seperately run above lines after cloning Prust-RTOS-Gen
```
rustup default nightly   
# sudo apt install build-essential  
cargo install xargo  
rustup component add rust-src  
rustup target add thumbv7em-none-eabihf  
xargo build --target thumbv7em-none-eabihf 
```

