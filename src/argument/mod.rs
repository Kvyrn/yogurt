pub mod parser;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Argument {
    validator: fn(String) -> bool,
    name: String,
    required: bool,
}

impl Argument {
    pub fn new(validator: fn(String) -> bool, name: String, required: bool) -> Self {
        Self {
            validator,
            name,
            required,
        }
    }
}
