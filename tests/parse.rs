#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        let note = xml::from_file("./examples/note.xml").expect("Failed to parse xml");

        let to = &note["to"][0];
        let from = &note["from"][0];
        let heading = &note.get_nodes("heading").expect("Missing heading")[0];
        let body = &note["body"][0];
        let lang = note
            .get_attribute("lang")
            .expect("Failed to get attribute lang");

        assert_eq!(to.content, "Tove");
        assert_eq!(from.content, "Jani");
        assert_eq!(heading.content, "Reminder");
        assert_eq!(body.content, "Don't forget me this weekend!");
        assert_eq!(lang, "en_US");
    }
}
