# A-Rust-Full-Stack-Web-App-with-Yew-and-Actix-Web
we’ll build a simple full-stack Rust web app using Actix and Yew. 🦀

Setting Up Your Rust Full-Stack Project 🏗️

1. Install Rust and Cargo

First, ensure you have Rust installed. If not, install it using:

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

Then, install WebAssembly target for Rust:

rustup target add wasm32-unknown-unknown

2. Create a New Actix Backend

cargo new rust-fullstack --bin
cd rust-fullstack

Add dependencies in Cargo.toml:


3. Creating the Yew Frontend
Install Trunk and Set Up Project

cargo install trunk
cargo new yew-frontend --lib
cd yew-frontend

4. Serve Front end
