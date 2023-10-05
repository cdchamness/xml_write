# scribe
An implementation to simplify writing for specific file types.

This currently is only implemented for xmls using the XMLWriter<T: Write>

```rust
let file = std::fs::File::create("foo.xml")?;
writer = XMLWriter::new(file)
  .open_tag("example")?               // opens a tag that must later be closed
  .add_elem("inner_tag", "val1")?     // opens a tag and closes it with "val1" as the element
  .open_tag("new_tag")?
  .add_elem("inside_new_tag", "val2")?
  .add_elem("inside_new_tag_again", "val3")?
  .close_tag()?                       // closes "new_tag"
  .add_elem("inside_example", "val4")?
  .close_tag()?                       // closes "example"
  .flush()?
```