<div align="center">

![Logo](docs/src/images/icon.jpg)

<h1>Wiki Graph</h1>
<h3>A Wikipedia search visualization tool</h3>

![Top Lang](https://img.shields.io/github/languages/top/francis-du/wiki-graph?color=%23E5531A&style=flat-square)
[![Api docs](https://img.shields.io/badge/Api-Doc-a94064?style=flat-square&color=%23E5531A)](https://wiki-graph.francis.run)
[![Rust Build](https://img.shields.io/github/workflow/status/francis-du/wiki-graph/cargo-test?label=rust%20build&style=flat-square)](https://github.com/francis-du/wiki-graph/actions?query=workflow:cargo-test)
[![Docs Build](https://img.shields.io/github/workflow/status/francis-du/wiki-graph/mdbook-deploy?label=docs%20build&style=flat-square)](https://github.com/francis-du/wiki-graph/actions?query=workflow:mdbook-deploy)
</div>

-----------------------------------------------------------------------------------------------

## [Live demo](https://wiki-graphs.herokuapp.com/)

## How to use

Enter keywords and click search, it will generate a network graph.
And keywords support multiple languages.[Checkout supported languages](https://github.com/francis-du/wiki-graph/blob/5bddd4e67229f1e8375a1dee0207a964ec7adb52/src/common/semantics.rs#L8-L24)

[![](docs/src/images/index.png)](https://wiki-graphs.herokuapp.com)

## Search result

[![](docs/src/images/graph.png)](https://wiki-graphs.herokuapp.com)

## How to deploy

### CLI Commands

```shell
░██╗░░░░░░░██╗██╗██╗░░██╗██╗  ░██████╗░██████╗░░█████╗░██████╗░██╗░░██╗
░██║░░██╗░░██║██║██║░██╔╝██║  ██╔════╝░██╔══██╗██╔══██╗██╔══██╗██║░░██║
░╚██╗████╗██╔╝██║█████═╝░██║  ██║░░██╗░██████╔╝███████║██████╔╝███████║
░░████╔═████║░██║██╔═██╗░██║  ██║░░╚██╗██╔══██╗██╔══██║██╔═══╝░██╔══██║
░░╚██╔╝░╚██╔╝░██║██║░╚██╗██║  ╚██████╔╝██║░░██║██║░░██║██║░░░░░██║░░██║
░░░╚═╝░░░╚═╝░░╚═╝╚═╝░░╚═╝╚═╝  ░╚═════╝░╚═╝░░╚═╝╚═╝░░╚═╝╚═╝░░░░░╚═╝░░╚═╝

Version: 0.1.0
A Wikipedia search visualization tool

USAGE:
    wiki-graph [FLAGS] [OPTIONS] <SUBCOMMAND>

FLAGS:
    -d, --debug      Activate debug mode
        --help       Prints help information
        --proxy      Using proxy network
    -V, --version    Prints version information

OPTIONS:
    -h, --host <host>    Set host [default: 0.0.0.0]
    -p, --port <port>    Set port [default: 3690]

SUBCOMMANDS:
    api     start a api service
    app     start a app service
    help    Prints this message or the help of the given subcommand(s)

```

### Local

- installation rust

- run `cargo run app` to start a service in ` http://0.0.0.0:3690`

- or use binary in `target/debug/` or `target/build/` to run `wiki-graph app` to start a service

### Heroku

[![Deploy](https://www.herokucdn.com/deploy/button.png)](https://heroku.com/deploy)

- installation hero CLI

- run commands

```shell
heroku login

heroku git:clone -a wiki-graphs 

cd wiki-graphs

git add .
git commit -am "hello heroku"
git push heroku master
```

## Docker

```shell
docker run -d -p 3690:3690 francisdu/wiki-graph
```

## License

[MIT LICENSE](LICENSE)
