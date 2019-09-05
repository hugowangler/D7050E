extern crate nom;
use nom::IResult;
use nom::character::complete::digit1;
use nom::FindSubstring;


enum Node {
    Root {left_child: Box<Node>, right_child: Box<Node>, operation: String},
    Leaf(i32),
    Nil
}

/*
fn expr_parser(input: &str) {
    create_tree(input); // Build the tree
    // Calculate expression
}

fn create_tree(input: &str) {
    let parsed = find_digit(input).unwrap();
    let left_child = create_node(digit.0);
    let root = create_tree(digit.1);
}

fn create_node(input: &str) -> (Node) {
    if input.is_empty() { // Was at leaf so no more nodes
        println!("Empty leaf");
        return Node::Nil
    } else {    // New node
        if input.digit
    }
} */

fn parse_digit(input: &str) -> IResult<&str, &str> {
    digit1(input)
}

fn parse_char(input: &str) -> Option<usize> {
    input.find_substring("+")
}

fn main() {
    let input = "1 + 2 + 3";
    println!("input = {}", input);
    parse_char(input);
}

