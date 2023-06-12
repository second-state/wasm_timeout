# wasm_timeout
This project demonstrates how to execute a WebAssembly module with a timeout.
## Overview
WebAssembly modules can execute code without upper time limits by default. This project shows how to apply timeouts to WebAssembly functions to prevent excessive execution time. If the timeout expires before the WebAssembly function returns, an error is thrown.
## Usage
First, you need to [install wasmedge](https://wasmedge.org/book/en/quick_start/install.html). 


To run the project:
```bash
rustc --target=wasm32-wasi -O demo.rs
```
This will compile demo.rs into demo.wasm.


To run the example:
```bash 
cargo run
```
