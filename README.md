# Minesweeper • Yew

Inspired by Dave Comptons [Comparison](https://dc25.github.io/myBlog/2017/11/26/minesweepers-written-using-elm-reflex-miso.html) between Elm, Miso and Reflex is this a Minesweeper version written in [Rust](https://www.rust-lang.org/en-US/) using [Yew](https://github.com/DenisKolodin/yew).

## Build Instructions

1. Install [Rustup](https://github.com/rust-lang-nursery/rustup.rs#installation):

    ```$ curl https://sh.rustup.rs -sSf | sh```
1. Install Rust nightly:

    ```$ rustup install nightly```
1. Set Rust nightly as default:

    ```$ rustup default nightly```
1. Install [cargo web](https://github.com/koute/cargo-web)

    ```$ cargo install cargo-web```
1. Run this project:

    ```$ cargo web start```