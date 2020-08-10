#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        let note =
            simple_xml::from_file("./examples/note.xml").expect("Failed to parse simple_xml");

        let to = &note["to"][0];
        let from = &note["from"][0];
        let heading = &note.get_nodes("heading").expect("Missing heading")[0];
        let body = &note["body"][0];
        let lang = note
            .get_attribute("lang")
            .expect("Failed to get attribute lang");

        // Test try_get_nodes
        match note.try_get_nodes("missing_tag") {
            Err(simple_xml::Error::TagNotFound(a, b)) if a == "note" && b == "missing_tag" => {}
            Err(simple_xml::Error::TagNotFound(_, _)) => {
                panic!("Incorrect error for try_get_nodes()");
            }
            Err(_) => {
                panic!("Incorrect error kind");
            }

            Ok(_) => panic!("Did not expect ok variant"),
        }

        assert_eq!(to.content, "Tove");
        assert_eq!(from.content, "Jani");
        assert_eq!(heading.content, "Reminder");
        assert_eq!(body.content, "Don't forget me this weekend!");
        assert_eq!(lang, "en_US");
    }

    #[test]
    fn parse_collada() {
        let root = match simple_xml::from_file("./examples/cube.dae") {
            Err(e) => {
                println!("Error: {:?}", e);
                panic!("")
            }
            Ok(v) => v,
        };
    }
}
