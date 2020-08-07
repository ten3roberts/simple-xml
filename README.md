# Simple XML
Simple xml is a small crate for reading, parsing and storing xml data

## Usage
Example parsing:

``` rust

let note =
    simple_xml::from_file("./examples/note.xml").expect("Failed to parse simple_xml");

let to = &note["to"][0];
let from = &note["from"][0];
let heading = &note.get_nodes("heading").expect("Missing heading")[0];
let body = &note["body"][0];
let lang = note
    .get_attribute("lang")
    .expect("Failed to get attribute lang");
```

More examples can be found in the docs and tests
