# Welcome to Search :search:

This is a search application built using the Loco framework for Rust.

# Open the demo [deployed on Fly.io here](https://search-sahc6w.fly.dev/)

## Disclaimers

1. Rust is not my *day job* language, but I do enjoy using it. I've read a few books and did some fun projects enough to have a grasp on basics, but it might show in the codebase maturity.
2. The choice of technologies doesn't mean endorsement:
    - `Loco.rs`: chosen for the quick start it provides and how it's supposed to look like Ruby on Rails. It's the first time I use in a project though and I've found a few limitations, but overall I really enjoyed using it.
    - `Tantivy`: I've mostly used hosted services for search (ElasticSearch), so it was my first time handling an on disk database, with locking mechanims, etc. As far as I'm aware, the current project can only be run as a monolothic process, including any background job. I didn't spend enough time evaluating options though.
    - `Spider`: seems to be focused on their cloud offering, because there's very limited documentation. It works well enough that I didn't look for alternatives, but I haven't 
    been able to improve the output of the webpage scraping.

## How it works?

`Loco.rs` is used as the project structure, it was used to scaffold two models called `Websites` and `Pages`. The seed data initializes the `Websites` with a list of domains which are supposed to be scraped. 

When the app boots, a `Tantivy` index is created in the `data/tantivy` directory. The index contains url, title and body. Currently title is not populated for lack of time extracing it properly. This index is persistent in the web server, so any readers and writers are created from it.

There is a background worker called `crawler` that takes a website domain and uses `Spider` to scrape it, following any links it may find. The output of the scrapping is currently converted to text using `Html2Text`, which I found not to be fully satisfactory, I believe a better process to extract text from websites would yield better results. Once this data is scraped, it's stored in both the `Tantivy` index and the database as a `Page` model.

There is a scheduler that crawls all websites at 10am, but I don't think it currently works in Fly.io. There's a button instead in the websites page that has the same effect, it queues all websites to be scraped in the background.

The search happens directly in the `Tantivy` index, which seems to be very fast and somewhat accurate. I believe a lot of techniques can be applied for a more accurate search, like treating the search term (removing common words for instance). The ranking given by `Tantivy` is used for the search results order. The body of the page is then used to create a snippet of about 400 characters using a very naive approach, but gives the user the idea of where the term is in the website.

## Quick Start

I've used `devenv` to manage the local environment, although it's not required, any standard Rust installation should suffice.

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
