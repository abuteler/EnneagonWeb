# EnneagonStudios.com

Company web application.
- [Live version](https://www.enneagonstudios.com/) - Currently Typescript + NextJS 14 + TailwindCSS
- [Dev version](https://enneagon-web.fly.dev/) - Rust + Leptos + Actix + TailwindCSS


## Project information

This web app was
- written in Rust,
- created with [Leptos](https://github.com/leptos-rs/leptos) web framework,
- stylized with [TailwindCSS](https://tailwindcss.com/),
- and bootstrapped with [Leptos Starter Template](https://github.com/leptos-rs/start).

It uses [cargo-leptos](https://github.com/akesson/cargo-leptos), and [Actix](https://actix.rs/) for the back-end.

<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img width="150" src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

### Running it local

Running `cargo leptos watch`, by default, you can access your local project at `http://localhost:3000`.

#### Executing a Server on a Remote Machine Without the Toolchain

After running a `cargo leptos build --release` the minimum files needed are:

1. The server binary located in `target/server/release`
2. The `site` directory and all files within located in `target/site`

Copy these files to your remote server. The directory structure should be:
```text
leptos_start
site/
```
Set the following environment variables (updating for your project as needed):
```sh
export LEPTOS_OUTPUT_NAME="leptos_start"
export LEPTOS_SITE_ROOT="site"
export LEPTOS_SITE_PKG_DIR="pkg"
export LEPTOS_SITE_ADDR="127.0.0.1:3000"
export LEPTOS_RELOAD_PORT="3001"
```
Finally, run the server binary.

##### Notes about CSR and Trunk:

Although it is not recommended, you can also run your project without server integration using the feature `csr` and `trunk serve`:

`trunk serve --open --features csr`

This may be useful for integrating external tools which require a static site, e.g. `tauri`.
