use std::io;

struct Node {
    state: u8,
    i_parents: Vec<usize>,
}

fn parse_input(edges: u32) -> Vec<Vec<u32>> {
    let mut ret: Vec<Vec<u32>> = Vec::new();
    let mut buffer: String;
    for _ in 0..edges {
        buffer = String::from("");
        _ = io::stdin().read_line(&mut buffer);
        let buffer_s: &str = &*buffer;
        let curr: Vec<u32> = buffer_s.split_whitespace()
                                        .map(|elm| elm.parse().unwrap())
                                        .collect();
        ret.push(curr);
    };
    return ret;
}

fn build_tree(parsed_input: Vec<Vec<u32>>, vertexes: u32) -> Vec<Node> {
    let mut ret: Vec<Node> = Vec::new();
    for _ in 0..vertexes {
        ret.push(Node{state: 0, i_parents: Vec::new()});
    }
    for elm in parsed_input {
        ret[(elm[1] - 1) as usize].i_parents.push(elm[0] as usize);
        ret[(elm[1] - 1) as usize].state = 0;
        if ret[(elm[1] - 1) as usize].i_parents.len() > 2 {
            panic!("node {} has more than two parents", elm[1]);
        }
    }
    return ret;
}

fn print_tree(inverted_tree: &Vec<Node>, size: u32) {
    for i in 0..size {
        println!("node #: {}; parents: {:#?}", i + 1, inverted_tree[i as usize].i_parents);
    }
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    _ = io::stdin().read_line(&mut buffer);
    let mut buffer_slice: &str = &*buffer;
    let pair: Vec<u32> = buffer_slice.split_whitespace()
                                 .map(|elm| elm.parse().unwrap())
                                 .collect();
    buffer = String::from("");
    _ = io::stdin().read_line(&mut buffer);
    buffer_slice = &*buffer;
    let values: Vec<u32> = buffer_slice.split_whitespace()
                                 .map(|elm| elm.parse().unwrap())
                                 .collect();
    let vertexes = values[0];
    let edges = values[1];
    let parsed_input: Vec<Vec<u32>> = parse_input(edges);
    // println!("{:#?}", parsed_input);
    let nodes: Vec<Node> = build_tree(parsed_input, vertexes);
    //print_tree(&nodes, vertexes);
    Ok(())
}
