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
        let _root = match simple_xml::from_file("./examples/cube.dae") {
            Err(e) => {
                println!("Error: {:?}", e);
                panic!("")
            }
            Ok(v) => v,
        };
    }

    #[test]
    fn parse_comment() {
        let _comment = simple_xml::from_file("examples/comment.xml")
            .expect("Failed to parse comment.xml");
    }

    #[test]
    fn parse_graph() {
        let graph =
            simple_xml::from_file("./examples/graph.xml").expect("Failed to parse graph.xml");

        assert_eq!(graph["node"].len(), 4);
        assert_eq!(graph["node"][0].attributes["id"], "n1");
        assert_eq!(graph["node"][0]["label"][0].content, "Start");
        assert_eq!(graph["node"][1].attributes["id"], "n2");
        assert_eq!(graph["node"][2].attributes["id"], "n3");
        assert_eq!(graph["node"][3].attributes["id"], "n4");

        assert_eq!(graph["init"][0].attributes["ref"], "n1");

        assert_eq!(graph["edge"][0].attributes["id"], "e1");
        assert_eq!(graph["edge"][0].attributes["from"], "n1");
        assert_eq!(graph["edge"][0].attributes["to"], "n2");

        assert_eq!(graph["edge"][1].attributes["id"], "e2");
        assert_eq!(graph["edge"][1].attributes["from"], "n2");
        assert_eq!(graph["edge"][1].attributes["to"], "n3");

        assert_eq!(graph["edge"][2].attributes["id"], "e3");
        assert_eq!(graph["edge"][2].attributes["from"], "n3");
        assert_eq!(graph["edge"][2].attributes["to"], "n1");

        assert_eq!(graph["edge"][3].attributes["id"], "e4");
        assert_eq!(graph["edge"][3].attributes["from"], "n3");
        assert_eq!(graph["edge"][3].attributes["to"], "n4");

        assert_eq!(graph["edge"][4].attributes["id"], "e5");
        assert_eq!(graph["edge"][4].attributes["from"], "n4");
        assert_eq!(graph["edge"][4].attributes["to"], "n3");
    }
}
