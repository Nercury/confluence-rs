# Confluence for Rust

[![Linux](https://travis-ci.org/Nercury/confluence-rs.svg)](https://travis-ci.org/Nercury/confluence-rs)

Access and modify [Atlassian Confluence](https://www.atlassian.com/software/confluence/) pages from Rust.

## Usage

### Add dependency

Add dependency in your `Cargo.toml`:

```toml
[dependencies]
confluence = "0.1"
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

Read __[full documentation](http://nercury.github.io/confluence-rs)__.

## License

MIT
