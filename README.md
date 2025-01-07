# Welcome to Search 

This is a search application built using the [Loco](https://crates.io/crates/loco-rs) framework for Rust, and the crates [Tantivy](https://crates.io/crates/tantivy) and [Spider](https://crates.io/crates/spider).

[![Open the Demo on Fly.io](https://img.shields.io/badge/Open%20Demo-Fly.io-7b3be2?style=for-the-badge&logo=fly.io&logoColor=white)](https://search-sahc6w.fly.dev/)

## Quick Start

This project `devenv` to manage the local environment, although it's not required, any standard Rust installation should suffice.

1. Install `devenv`:
   If you haven't already installed `devenv`, you can do so by following the instructions on the [devenv.sh website](https://devenv.sh/getting-started/).

2. Initialize the development environment:
    ```sh
    devenv init
    ```

3. Seed data to make sure you have a list of websites:
    ````sh
    cargo loco task seed_data
    ```

4. Start the webserver:
    ```sh
    cargo loco start -s
    ```

```sh
$ cargo loco start
Finished dev [unoptimized + debuginfo] target(s) in 21.63s
    Running `target/debug/myapp start`

    :
    :
    :

controller/app_routes.rs:203: [Middleware] Adding log trace id

                      ▄     ▀
                                 ▀  ▄
                  ▄       ▀     ▄  ▄ ▄▀
                                    ▄ ▀▄▄
                        ▄     ▀    ▀  ▀▄▀█▄
                                          ▀█▄
▄▄▄▄▄▄▄  ▄▄▄▄▄▄▄▄▄   ▄▄▄▄▄▄▄▄▄▄▄ ▄▄▄▄▄▄▄▄▄ ▀▀█
 ██████  █████   ███ █████   ███ █████   ███ ▀█
 ██████  █████   ███ █████   ▀▀▀ █████   ███ ▄█▄
 ██████  █████   ███ █████       █████   ███ ████▄
 ██████  █████   ███ █████   ▄▄▄ █████   ███ █████
 ██████  █████   ███  ████   ███ █████   ███ ████▀
   ▀▀▀██▄ ▀▀▀▀▀▀▀▀▀▀  ▀▀▀▀▀▀▀▀▀▀  ▀▀▀▀▀▀▀▀▀▀ ██▀
       ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀
                https://loco.rs

environment: development
   database: automigrate
     logger: debug
compilation: debug
      modes: server

listening on http://localhost:5150
```
