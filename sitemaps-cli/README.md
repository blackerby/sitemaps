# sitemaps-cli


A command line tool for working with Sitemaps as data.

## Installation

With Rust and Cargo installed, you can run

```sh
cargo install sitemaps-cli
```

## Usage

### Reading a file

```sh
sitemaps path/to/sitemap.xml
```

### Options

```sh
sitemaps --help
```

### Reading standard input through a pipe
```sh
curl https://www.govinfo.gov/sitemap/bulkdata/PLAW/117pvtl/sitemap.xml | sitemaps
```
