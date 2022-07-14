use clang::*;

pub trait Name {
    fn get_name(&self) -> String;
}

impl<T: Name> Name for Option<T> {
    fn get_name(&self) -> String {
        match self {
            Some(t) => t.get_name(),
            None => String::from(""),
        }
    }
}

impl Name for String {
    fn get_name(&self) -> String {
        self.to_string()
    }
}

impl Name for Usr {
    fn get_name(&self) -> String {
        self.0.to_string()
    }
}

impl Name for source::SourceLocation<'_> {
    fn get_name(&self) -> String {
        let location = self.get_presumed_location();
        [location.0, location.1.to_string(), location.2.to_string()].join(":")
    }
}

pub fn return_empty_if_null(name: Option<String>) -> String {
    match name {
        Some(name) => name,
        None => String::from(""),
    }
}