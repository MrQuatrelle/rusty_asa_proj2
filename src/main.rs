use std::io;

struct Node {
    id: u32,
    parents: Vec<Node>,
}

fn parse_input() -> Vec<Vec<u32>> {
    let mut ret: Vec<Vec<u32>> = Vec::new();
    return ret;
}

fn build_tree() -> Vec<Node> {
    let mut ret: Vec<Node> = Vec::new();
    return ret;
}

fn main() -> io::Result<()> {
    let parsed_input: Vec<Vec<u32>> = parse_input();
    let _nodes: Vec<Node> = build_tree();
    Ok(())
}
