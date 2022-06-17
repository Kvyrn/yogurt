pub mod parser;

pub struct Argument {
    validator: Box<dyn Fn(String) -> bool>,
    name: String,
    required: bool,
}

impl Argument {
    pub fn new(validator: Box<dyn Fn(String) -> bool>, name: String, required: bool) -> Self {
        Self {
            validator,
            name,
            required,
        }
    }
}
