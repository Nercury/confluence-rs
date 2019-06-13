# Confluence for Rust

[![Linux](https://travis-ci.org/Nercury/confluence-rs.svg)](https://travis-ci.org/Nercury/confluence-rs)

Access and modify [Atlassian Confluence](https://www.atlassian.com/software/confluence/) pages from Rust.

## Usage

### Add dependency

Add dependency in your `Cargo.toml`:

```toml
[dependencies]
confluence = "0.4"
```

### Example to update page title

```rust
extern crate confluence;

fn main() {
    // Get `Page` struct.
    let mut page = session
        .get_page_by_title("SomeSpaceKey", "Page Title")
        .expect("failed to fetch the page");

    // Change the title.
    page.title = "New Page Title".into();

    // Convert `Page` struct to `UpdatePage` and store it.
    session.store_page(page.into())
        .expect("failed to update the page");
}
```

## Reference

Read __[full documentation](https://docs.rs/crate/confluence/0.3.0)__.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
