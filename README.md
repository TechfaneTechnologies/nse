# NSE

[![rust badge](https://img.shields.io/static/v1?label=Made%20with&message=Rust&style=for-the-badge&logo=rust&labelColor=e82833&color=b11522)](https://www.rust-lang.org/)
[![license badge](https://img.shields.io/github/license/TechfaneTechnologies/nse?style=for-the-badge)](https://github.com/TechfaneTechnologies/nse/blob/main/LICENSE)
[![copyleft badge](https://img.shields.io/static/v1?label=&message=Copyleft&style=for-the-badge&labelColor=silver&color=silver&logo=data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADIAAAAyCAQAAAC0NkA6AAAABGdBTUEAALGPC/xhBQAAACBjSFJNAAB6JgAAgIQAAPoAAACA6AAAdTAAAOpgAAA6mAAAF3CculE8AAAAAmJLR0QA/4ePzL8AAAAHdElNRQfjAxYBNgYPa+9oAAAEM0lEQVRYw6WYb0zVVRjHP9wQW7umA0xoKNSC+6bSNkzetKZbaVu19aLpfOGcbcw/S+uNbikuNwMsVyE3XVsro7VEXjS3ylmLxkRtC9crHGjCAv9AATK4CoZye8Hl/J7n/M7v8rvX57w55/lznt/583yf5/xyCEOlrKaSCp6ggCiQYJheLvMHv9HHA1MZ++kmmaZ1UUNZ9g6eo4X7aR3Mtvs0syJzB0U0MR3KgddOsiQTFxsZzdDBTLvFetd0OT5OHo1U+7j9tNJBN4MkgChFVLCS1Sz1aR7jHf5Lv4Yov1hfN8YRKgP1V9LIuGVxhmg6Fwv4XalPcJD8OTe3gA+YVHYXgt3kWato46nQp1jOWWs1eW7Fz5VaLbkZ3cdc6pX9UfeNkvd+a1aRtV3Fle+mLeGWEO/0mT/EWo7SxhBjjNDPKfbxtMPNVjHLKMVa+I0Q1lmG89nDTWdctPGqz80hIT+uAWRaGOqzeJEraQOw2YrzXNqNbJrlnqDFsCeJKZO3uDtnnN+wNq6cCSM74SGtd1wHlfrOkHAyyDPKrk5codIZ1n7DSlAoVF9iKjRq/cVCYZnPmJHsnWF1GcYRobiQf3yA3sr7VPM2cXp9br5Va2k0/EsAy4SixKh6a5LT6rQibGBAyaeV9SohWQabzeBvhUcTaoqPHHhdTKfSOaWk1wx/E8TN4CuhssW6pjnOCF/KiNrOxULWZPgNEbEJF4VKFT2mdbGLpNNJPzVqC9eKkTdbDK4ajy9ngVaPiHuU5AshWWe4VyIsMuwbWTi5Q7sYlYj+TdNbFBHpJZEV8vao8sOjMS8VRh64MkumrRhSh5UQ+T278s+jQdF/1PTGI4yaweNZuHiYF1RsyCiapdFcengyNajgZyP4RBhP8RpDAU42KcxqE30vNK7KYJQpploFY1NgnfmvApYiZxpskLAi6/PFVh454HBRyJ9K5yclvS5hJQggP7YA8vvZzJCi1+m3NKoUYnj8Eg31jSonDFuTTPEju9nIZuq55IP6FvUJ3iF0zjBqApLWOu6FTlp9FCgM90rX9/zpt1Z9z56QLkasatnLRfe8TT5pmHetQqI6RAoesB5A5aIy/s5jrxAl0VmrJHqFvrQuflCwCPM4Jy71s1L0tTA75IPzAyo5ea3D8eg5LORf2mWqnGaXz3Q+b3CcDm6nCtBfqeV5R+xsUyf1mC3eoBLp9qzAcocquN90qRxTW/Fhxk+Hw8o+HvQIOqPU2qkI7SLGeauAmhf8YrygVCepU0HmpkLqLaQ7nz43Ra3VJBknzqpA/SrivofpaduF64n9Kdt83OupJ/YA48ACiolRyRpHovuMd5kKs8PrA+JirjbsvlFBlE9DyP8qXnQ3+eNiblpOc+gfOCc0gGRGpeyzymq7dbLXSmch/q24qIQ1VBKjjMLUT7UheunmIq2qQgmg/wHquM6d9tIV7AAAACV0RVh0ZGF0ZTpjcmVhdGUAMjAxOS0wMy0yMlQwMTo1NDowNiswMDowMOIizoUAAAAldEVYdGRhdGU6bW9kaWZ5ADIwMTktMDMtMjJUMDE6NTQ6MDYrMDA6MDCTf3Y5AAAAAElFTkSuQmCC)](https://en.wikipedia.org/wiki/Copyleft)

> Check out the sister projects [NsePython](https://github.com/TechfaneTechnologies/nsepython) and [SaveKiteEnctoken](https://github.com/TechfaneTechnologies/SaveKiteEnctoken) which are Python & Javascript libraries to use the NSE and Zerodha APIs respectively

NSE is an easy to use rust CLI program and library for extracting real-time data from [National Stock Exchange (India)](https://www.nseindia.com/)

## Features

- Download EOD 1-Minute Data for NFO-FUT, NSE-EQ, NSE-Index, NSE-CDS from NSE Tame Charts.
- Download Intraday Futures & Options Data Snapshots with all LTP & OI Data.
- Multi-threading for network intensive subcommands
  - You can configure the maximum number of additional threads using the `--threads` options

## Installation

nse is a compiled, statically linked program that does not require any external dependencies.
> nse is coming to more package managers soon!

### Cargo Install

If you have the Rust toolchain, you can also compile and install nse by running `cargo install nse`.

Remember to use an add-on like [cargo-update](https://crates.io/crates/cargo-update) to keep nse updated to the latest version!

### GitHub Releases

1. Download the asset suitable for your operating system from [the latest release](https://github.com/TechfaneTechnologies/nse/releases/latest)
2. Unzip the file and move it to a folder in your path such as `~/bin`
3. Remember to check the releases page for any updates!

### First Startup

When you first start up, the program will create a new data download directory automatically named as `NSE_Downloads_Data` in which the download files will be further categorised, subcatogorised and saved into dated folders.

```bash
NSE_DOWNLOADS_DATA
├───1-Minute-OHLCV-Data
│   ├───Commodity
│   │   └───11-May-2022
│   ├───Currency
│   │   └───11-May-2022
│   ├───Equity
│   │   └───11-May-2022
│   ├───Futures
│   │   └───11-May-2022
│   └───Index
│   │   └───11-May-2022
├───BankNifty
│   ├───Futures_Intraday_Snapshots
│   │   └───11-May-2022
│   ├───GraphsData
│   │   └───11-May-2022
│   └───Options_Intraday_Snapshots
│       └───11-May-2022
├───FinNifty
│   ├───Futures_Intraday_Snapshots
│   │   └───11-May-2022
│   ├───GraphsData
│   │   └───11-May-2022
│   └───Options_Intraday_Snapshots
│       └───11-May-2022
├───Misc
│   ├───Futures_Intraday_Snapshots
│   │   └───11-May-2022
│   ├───GraphsData
│   │   └───11-May-2022
│   └───Options_Intraday_Snapshots
│       └───11-May-2022
└───Nifty
    ├───Futures_Intraday_Snapshots
    │   └───11-May-2022
    ├───GraphsData
    │   └───11-May-2022
    └───Options_Intraday_Snapshots
        └───11-May-2022
```

**WARNING:** _When upgrading, any files not downloaded by nse will be moved to the `.old` folder in the output directory_

#### Configure

_To Be Updated_

#### Manage

_To Be Updated_

#### Delete

_To Be Updated_

## Feature Requests

If you would like to make a feature request, check the [issues](https://github.com/TechfaneTechnologies/nse/issues?q=is%3Aissue) to see if the feature has already been added or is planned. If not, [create a new issue](https://github.com/TechfaneTechnologies/nse/issues/new).

## Building from Source or Working with nse

Firstly, you need the Rust toolchain which includes `cargo`, `rustup`, etc. You can install these from [the Rust website](https://www.rust-lang.org/tools/install).
You'll also need the [Just](https://github.com/casey/just#installation) command runner, its basically a much better version of `make`.

If you want to build nse without cloning the repo then run `cargo install nse`.

To build the project and install it to your Cargo binary directory, clone the project and run `just install`. If you want to install it for testing a developement version, run `just` (alias for `just install-dev`).

If you want to obtain executables for a specific OS, you can run `just build-<OS>` and replace `<OS>` with `mac`, `win`, or `linux`. The produced binaries will be zipped and moved to `out/`.

You can run clippy linters using `just lint`, and integration tests using `cargo test`. Finally you can delete all the build and test artefacts by using `just clean`.
