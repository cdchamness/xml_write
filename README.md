# xml_write
An implementation to simplify writing xml file types.

```rust
let file = std::fs::File::create("foo.xml")?;
writer = XMLWriter::new(file)?        // This automatically writes the header therefore returns a result
  .open_tag("example")?               // opens a tag that must later be closed
  .add_elem("inner_tag", "val1")?     // opens a tag and closes it with "val1" as the element
  .open_tag("new_tag")?
  .add_elem("inside_new_tag", "val2")?
  .add_elem("inside_new_tag_again", "val3")?
  .close_tag()?                       // closes "new_tag"
  .add_elem("inside_example", "val4")?
  .close_tag()?                       // closes "example"
  .flush()?;
```

This produces a new file, `foo.xml` which has the contents:

```xml
<?xml version="1.0" encoding="UTF-8" ?>
<example>
  <inner_tag>val1</inner_tag>
  <new_tag>
    <inside_new_tag>val2</inside_new_tag>
    <inside_new_tag_again>val3</inside_new_tag_again>
  </new_tag>
  <inside_example>val4</inside_example>
</example>
```
