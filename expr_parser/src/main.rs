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
    } else {
        let op_parsed = parse_char(&input);
        let op = op_parsed.1;
        let op_pos = op_parsed.0;
        match op_pos {
            Some(op_pos) => {
                let left_child = create_tree(&input[..op_pos]);
                let right_child = create_tree(&input[op_pos+1..]);
                create_root(left_child, right_child, op)
            }
            None => {
                create_leaf(&input)
            }
        }

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


fn parse_char(input: &str) -> (Option<usize>, char) {
    (input.find_substring("+"), '+')
}


fn main() {
    let input = "1";
    println!("input = {}", input.len());
    

    let result = expr_parser(&input);
    println!("result ={:#?}", result);


    /*
    let test = parse_char(&input);
    println!("char ={:?}", test);
    
    
    let plus = input.find_substring("+");

    match plus {
        Some(usize) => println!("nice"),
        None => println!("not nice")
    };
    */
    
}

