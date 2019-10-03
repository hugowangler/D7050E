#[derive(Clone, Debug, PartialEq)]
pub enum Value {
	Number(i32),
	Bool(bool),
	String(String),
	None,
}