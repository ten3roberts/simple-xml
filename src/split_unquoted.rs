//! This is a module providing the functionality to split a string with a delimiter function unless the delimiter is surrounded in quotes
//! The quotes can also be escaped

pub struct SplitUnquoted<'a, F>
where
    F: Fn(char) -> bool,
{
    in_quotes: bool,
    data: &'a str,
    del: F,
}

impl<'a, F> SplitUnquoted<'a, F>
where
    F: Fn(char) -> bool,
{
    pub fn split(data: &'a str, delimiter_func: F) -> Self {
        SplitUnquoted {
            in_quotes: false,
            data,
            del: delimiter_func,
        }
    }
}

impl<'a, F> Iterator for SplitUnquoted<'a, F>
where
    F: Fn(char) -> bool,
{
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        let mut non_del = false;
        for (i, c) in self.data.char_indices() {
            if c == '"' {
                self.in_quotes = !self.in_quotes;
            }

            if (self.del)(c) {
                if non_del && !self.in_quotes {
                    let end = &self.data[..i];
                    // println!("Iter end: '{}', i: {}", end, i);
                    self.data = &self.data[i + 1..];
                    // println!("data: '{}'",&self.data[i..]);
                    return Some(end);
                }
            } else {
                non_del = true;
            }
        }
        match self.data.len() {
            0 => None,
            // No more quotes left, return what is over and end iterator
            _ => {
                let last = self.data;
                self.data = &self.data[..0];
                Some(last)
            }
        }
    }
}
