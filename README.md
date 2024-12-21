# Welcome to Search :search:

This is a search application built using the Loco framework for Rust.

## Disclaimers

1. Rust is not my *day job* language, but I do enjoy using it. I've read a few books and did some fun projects enough to have a grasp on basics, but it might show in the codebase maturity.
2. The choice of technologies doesn't mean endorsement:
    - `Loco.rs`: chosen for the quick start it provides. It's the first time I use in a project though and I've found a few limitations, but overall I really enjoyed using it.
    - `Tantivy`: I've mostly used hosted services for search (ElasticSearch), so it was my first time handling an on disk database, with locking mechanims, etc. As far as I'm aware, the current project can only be run as a monolothic process, including any background job. I didn't spend enough time evaluating options though.
    - `Spider`: seems to be focused on their cloud offering, because there's very limited documentation. It works well enough that I didn't look for alternatives, but I haven't been able to improve the output of the webpage.


## Quick Start

I've used `devenv` to manage the local environment, although it's not required, any standard Rust installation should suffice.

1. Install `devenv`:
   If you haven't already installed `devenv`, you can do so by following the instructions on the [devenv.sh website](https://devenv.sh/getting-started/).

2. Initialize the development environment:
    ```sh
    devenv init
    ```

3. Start the webserver:
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

## Full Stack Serving

You can check your [configuration](config/development.yaml) to pick either frontend setup or server-side rendered template, and activate the relevant configuration sections.


## Getting help

Check out [a quick tour](https://loco.rs/docs/getting-started/tour/) or [the complete guide](https://loco.rs/docs/getting-started/guide/).
