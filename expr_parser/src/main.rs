extern crate nom;
use nom::IResult;
use nom::character::complete::digit1;
use nom::FindSubstring;

#[derive(Debug)]
enum Node {
    Root {
        operation: char,
        left_child: Box<Node>,
        right_child: Box<Node>
    },
    Leaf(i32),
    Nil
}

fn expr_parser(input: &str) -> (Node) {
    create_tree(&input)
}

fn create_tree(input: &str) -> (Node) {
    if input.is_empty() {
        Node::Nil
    } else if input.len() <= 1 {
        create_leaf(&input)
    } else {
        let char_pos = parse_char(&input);
        let op = char_pos.1;
        let op_pos = char_pos.0;

        let left_child = create_tree(&input[..op_pos]);
        let right_child = create_tree(&input[op_pos+1..]);

        create_root(left_child, right_child, op)
    }
}

fn create_leaf(input: &str) -> (Node) {
    let num = parse_digit(&input).unwrap().1.parse::<i32>().unwrap();
    Node::Leaf(num)
} 

fn create_root(l_child: Node, r_child: Node, op: char) -> (Node) {
    Node::Root{left_child: Box::new(l_child), right_child: Box::new(r_child), operation: op}
}

fn parse_digit(input: &str) -> IResult<&str, &str> {
    digit1(input)
}

fn parse_char(input: &str) -> (usize, char) {
    let plus_pos = input.find_substring("+");
    (plus_pos.unwrap(), '+')
}

fn main() {
    let input = "1+2+3";
    println!("input = {}", input);
    let result = expr_parser(&input);
    println!("result ={:#?}", result);
}

