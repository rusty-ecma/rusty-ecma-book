# StringWriter
When building `resw` it became clear that the only way to validate the output would be to write a bunch of files to disk and then read them back which didn't seem like the correct option. Because of this `resw` includes an public module called `write_str`. In it you will find two structs `WriteString` and `ChildWriter`. The basic idea here is that you can use this to simply write the values to a buffer that the `resw::Writer` hasn't taken ownership over and then read them back after the `Writer` is done. Below is an example of how you might use that.

```rust
fn test_round_trip() {
    let original = "let x = 0";
    let dest = WriteString::new();
    let parser = ressa::Parser::new(original).expect("Failed to create parser");
    let writer = resw::Writer::new(dest.generate_child());
    for part in parser {
        let part = part.expect("failed to parse part");
        writer.write_part(part).expect("failed to write part");
    }
    assert_eq!(dest.get_string_lossy(), original.to_string());
}
```