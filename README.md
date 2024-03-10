# Sitemaps

Sitemaps is:

- a personal, experimental Rust learning project
- a Rust library for reading [sitemaps](https://www.sitemaps.org/) that I hope will one day be useful for me and for others.

Sitemaps is currently experimental, a work in progress, &c. Even its name might change. It already has once.

## Goals

I want Sitemaps to be easy for a new Rust programmer to understand, because I am that new Rust programmer. The goal is that the code be idiomatic and intelligible, that it use best practices, and that it be easy to use the library and understand and extend the code. I'd like for it to be lightweight (is that possible when an HTTP library is a dependency?) and reasonably fast.

## TODO (in no particular order)

### Lib

- [ ] test sitemap index writing
- [ ] Sitemaps enum with two variants: 
  - Sitemap
  - Sitemap index
  - one method: read_from
    - reads root element event, calls read_from for appropriate variant based on text of that event
      - sitemapindex -> Sitemaps::SitemapIndex
      - urlset -> Sitemap
      - error on unrecognized root element
- [ ] use `thiserror`
- [ ] document!
- [ ] extract validation code

### CLI

- [ ] test with `assert_cmd`
- [ ] sitemap index handling
- [ ] adjust some flag defaults
