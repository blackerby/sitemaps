# Sitemaps

Sitemaps is:

- a personal, experimental Rust learning project
- a Rust library for reading [sitemaps](https://www.sitemaps.org/) that I hope will one day be useful for me and for others.

Sitemaps is currently experimental, a work in progress, &c. Even its name might change. It already has once.

## Goals

I want Sitemaps to be easy for a new Rust programmer to understand, because I am that new Rust programmer. The goal is that the code be idiomatic and intelligible, that it use best practices, and that it be easy to use the library and understand and extend the code. I'd like for it to be lightweight (is that possible when an HTTP library is a dependency?) and reasonably fast.

## TODO (in no particular order)

- [x] custom error type
- [x] verify encoding
- [x] URL validation for `<loc>`
- [x] Priority validation
- [x] validate `<loc>` length
- [x] validate URL count
- [x] read from the web
- [x] proper datetime handling for `<lastmod>`
- [x] add CLI
- [x] remove `new` and instantiate SitemapReader from `read`
- [x] tweak datetime display format
- [x] reimplement CLI output with good formatting
- [ ] make showing header optional
- [ ] show header for plain output
- [ ] read from stdin
- [ ] sitemap index handling
- [ ] add CLI subcommand to filter on `<lastmod>`
- [ ] use `thiserror`
- [ ] add doc comments
- [ ] validate `urlset` namespace attribute?
- [ ] serialization
  - json
  - csv
  - xml
    - add ability to edit?
