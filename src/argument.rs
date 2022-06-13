#[derive(Debug, Clone)]
pub struct Argument {
    identifier: ArgumentIdentifier,
}

#[derive(Debug, Clone)]
pub enum ArgumentIdentifier {
    Position(u32),
    Name(String),
}
