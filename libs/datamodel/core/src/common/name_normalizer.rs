pub(crate) trait NameNormalizer {
    fn camel_case(&self) -> String;

    fn pascal_case(&self) -> String;
}

impl NameNormalizer for String {
    fn camel_case(&self) -> String {
        let mut c = self.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_lowercase().collect::<String>() + c.as_str(),
        }
    }

    fn pascal_case(&self) -> String {
        let mut c = self.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }
}
