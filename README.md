# Data-Collection-Rust-server

A rust server for Data collection of any game for the purposes of AI.

## Setup

Make sure to have Rust installed, which you can do in the [rust developer handbook](<https://doc.rust-lang.org/stable/book/ch01-01-installation.html>)
(Use the linux version in WSL if you are using windows!)

Then run `cargo install --path .` to install all necessary packages

From here you can run the project using `cargo run`.

## Include hot-reloading

In order to use hotreloading we need to use bacon. Bacon is a hot-reloading tool.

First do `cargo install bacon`

Then you can run `bacon run` to run the project with hot-reloading.

(The default behaviour in `bacon run` is just to run the program, but i have changed it to hot-reload)
