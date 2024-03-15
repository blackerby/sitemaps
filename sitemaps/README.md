# sitemaps

Read and write files in the [Sitemaps XML format](https://sitemaps.org/protocol.html)

```rust
use std::fs::File;
use std::io::BufReader;
use sitemaps::SitemapsFile;

let file = File::open("tests/data/example_1_url.xml").unwrap();
let reader = BufReader::new(file);
let sitemap = SitemapsFile::read(reader).unwrap();
```
