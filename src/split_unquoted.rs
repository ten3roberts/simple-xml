//! This is a module providing the functionality to split a string with a delimiter unless the delimiter is surrounded in quotes
//! The quotes can also be escaped

pub struct SplitUnquoted<'a> {
    in_quotes: bool,
    data: &'a str,
    del: char,
}

impl<'a> SplitUnquoted<'a> {
    pub fn split(data: &'a str, delimiter: char) -> Self {
        SplitUnquoted {
            in_quotes: false,
            data: data,
            del: delimiter,
        }
    }
}

impl<'a> Iterator for SplitUnquoted<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        for (i, c) in self.data.char_indices() {
            if c == '"' {
                self.in_quotes = !self.in_quotes;
            }

            if !self.in_quotes && c == self.del {
                let end = &self.data[..i];
                // println!("Iter end: '{}', i: {}", end, i);
                self.data = &self.data[i + 1..];
                // println!("data: '{}'",&self.data[i..]);
                return Some(end);
            }
        }
        match self.data.len() {
            0 => None,
            // No more quotes left, return what is over and end iterator
            _ => {
                let last = &self.data[..];
                self.data = &self.data[..0];
                Some(last)
            }
        }
    }
}
