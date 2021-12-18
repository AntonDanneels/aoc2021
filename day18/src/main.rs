type NodeId = usize;

#[derive(Debug)]
enum Node {
    Pair {
        left: NodeId,
        right: NodeId,
        parent: Option<NodeId>,
    },
    Leaf {
        value: u32,
        parent: Option<NodeId>,
    },
}

impl Node {
    fn set_parent(&mut self, parent: NodeId) {
        match self {
            Node::Pair {
                left: _,
                right: _,
                parent: p,
            } => *p = Some(parent),
            Node::Leaf {
                value: _,
                parent: p,
            } => *p = Some(parent),
        };
    }

    fn get_parent(&mut self) -> Option<NodeId> {
        match self {
            Node::Pair {
                left: _,
                right: _,
                parent: p,
            } => *p,
            Node::Leaf {
                value: _,
                parent: p,
            } => *p,
        }
    }

    fn add_value(&mut self, val: u32) {
        match self {
            Node::Pair {
                left: _,
                right: _,
                parent: p,
            } => panic!(),
            Node::Leaf { value, parent: p } => *value += val,
        }
    }

    fn get_value(&self) -> u32 {
        match self {
            Node::Pair {
                left: _,
                right: _,
                parent: p,
            } => panic!(),
            Node::Leaf { value, parent: p } => *value,
        }
    }

    fn get_right(&self) -> NodeId {
        match self {
            Node::Pair {
                left: _,
                right,
                parent: p,
            } => *right,
            Node::Leaf { value, parent: p } => panic!(),
        }
    }

    fn get_left(&self) -> NodeId {
        match self {
            Node::Pair {
                left,
                right,
                parent: p,
            } => *left,
            Node::Leaf { value, parent: p } => panic!(),
        }
    }

    fn replace_node(&mut self, old: NodeId, new_node: NodeId) {
        match self {
            Node::Pair {
                left,
                right,
                parent: p,
            } => {
                if *left == old {
                    *left = new_node;
                }
                if *right == old {
                    *right = new_node;
                }
            }
            Node::Leaf { value, parent: p } => panic!(),
        }
    }

    fn is_leaf(&self) -> bool {
        match self {
            Node::Pair {
                left,
                right,
                parent: p,
            } => {
                return false;
            }
            Node::Leaf { value, parent: p } => {
                return true;
            }
        }
    }
}

struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    fn add_node_leaf(&mut self, value: u32) -> NodeId {
        let index = self.nodes.len();

        self.nodes.push(Node::Leaf {
            value,
            parent: None,
        });

        index
    }

    fn add_node_pair(&mut self, left: NodeId, right: NodeId) -> NodeId {
        let index = self.nodes.len();

        self.nodes.push(Node::Pair {
            left,
            right,
            parent: None,
        });

        index
    }

    fn print_node(&self, start: NodeId, depth: u32) {
        match self.nodes[start] {
            Node::Pair {
                left,
                right,
                parent,
            } => {
                for _ in 0..depth {
                    print!("\t");
                }
                println!("Pair {{ ");

                self.print_node(left, depth + 1);
                self.print_node(right, depth + 1);

                for _ in 0..depth {
                    print!("\t");
                }
                println!("\tparent: {:?}", parent);
                for _ in 0..depth {
                    print!("\t");
                }
                println!("}}");
            }
            Node::Leaf { value, parent } => {
                for _ in 0..depth {
                    print!("\t");
                }
                println!("Leaf {{ value: {}, parent: {:?} }}", value, parent);
            }
        };
    }

    fn print(&self, start: NodeId) {
        self.print_node(start, 0);
    }

    fn print_simple(&self, start: NodeId) {
        self.print_node_simple(start);
        println!();
    }

    fn print_node_simple(&self, start: NodeId) {
        match self.nodes[start] {
            Node::Pair {
                left,
                right,
                parent,
            } => {
                print!("[");
                self.print_node_simple(left);
                print!(",");
                self.print_node_simple(right);
                print!("]");
            }
            Node::Leaf { value, parent } => {
                print!("{}", value);
            }
        }
    }
}

fn parse_tree(input: &mut impl Iterator<Item = char>, tree: &mut Tree) -> NodeId {
    match input.next() {
        Some('[') => {
            let left = parse_tree(input, tree);
            if let Some(',') = input.next() {
                let right = parse_tree(input, tree);

                if let Some(']') = input.next() {
                    let id = tree.add_node_pair(left, right);
                    tree.nodes[left].set_parent(id);
                    tree.nodes[right].set_parent(id);

                    return id;
                }
            }
            panic!("Invalid input data");
        }
        Some(x) => {
            if let Some(value) = x.to_digit(10) {
                return tree.add_node_leaf(value);
            } else {
                panic!("Invalid input data");
            }
        }
        _ => panic!("Invalid input data"),
    }
}

fn add_trees(t1: NodeId, t2: NodeId, tree: &mut Tree) -> NodeId {
    let new_tree = tree.add_node_pair(t1, t2);
    tree.nodes[t1].set_parent(new_tree);
    tree.nodes[t2].set_parent(new_tree);

    new_tree
}

fn get_left_most_node(start: NodeId, tree: &Tree) -> NodeId {
    match tree.nodes[start] {
        Node::Pair {
            left,
            right: _,
            parent: _,
        } => get_left_most_node(left, tree),
        Node::Leaf {
            value: _,
            parent: _,
        } => start,
    }
}

fn get_right_most_node(start: NodeId, tree: &Tree) -> NodeId {
    match tree.nodes[start] {
        Node::Pair {
            left: _,
            right,
            parent: _,
        } => get_right_most_node(right, tree),
        Node::Leaf {
            value: _,
            parent: _,
        } => start,
    }
}

fn find_offending_node(start: NodeId, tree: &Tree, depth: u32) -> Option<NodeId> {
    match tree.nodes[start] {
        Node::Pair {
            left,
            right,
            parent: _,
        } => {
            if depth >= 4 && tree.nodes[left].is_leaf() && tree.nodes[right].is_leaf() {
                return Some(start);
            }
            if let Some(id) = find_offending_node(left, tree, depth + 1) {
                return Some(id);
            }
            if let Some(id) = find_offending_node(right, tree, depth + 1) {
                return Some(id);
            }
        }
        Node::Leaf {
            value: _,
            parent: _,
        } => {
            return None;
        }
    }

    None
}

fn find_right_sibling(start: NodeId, tree: &mut Tree) -> Option<NodeId> {
    let mut current = start;
    loop {
        if let Some(p) = tree.nodes[current].get_parent() {
            if tree.nodes[p].get_right() == current {
                current = p;
            } else {
                return Some(get_left_most_node(tree.nodes[p].get_right(), tree));
            }
        } else {
            return None;
        }
    }
}

fn find_left_sibling(start: NodeId, tree: &mut Tree) -> Option<NodeId> {
    let mut current = start;
    loop {
        if let Some(p) = tree.nodes[current].get_parent() {
            if tree.nodes[p].get_left() == current {
                current = p;
            } else {
                return Some(get_right_most_node(tree.nodes[p].get_left(), tree));
            }
        } else {
            return None;
        }
    }
}

fn explode_tree(start: NodeId, tree: &mut Tree) -> bool {
    let node = find_offending_node(start, tree, 0);

    if let Some(node) = node {
        let left = find_left_sibling(node, tree);
        let right = find_right_sibling(node, tree);

        let parent = tree.nodes[node].get_parent().unwrap();
        let new_node = tree.add_node_leaf(0);
        tree.nodes[new_node].set_parent(parent);

        let n = &tree.nodes[node];
        let vl = tree.nodes[n.get_left()].get_value();
        let vr = tree.nodes[n.get_right()].get_value();

        if let Some(left) = left {
            tree.nodes[left].add_value(vl);
        }
        if let Some(right) = right {
            tree.nodes[right].add_value(vr);
        }

        tree.nodes[parent].replace_node(node, new_node);

        return true;
    }

    false
}

fn split_node(node: NodeId, tree: &mut Tree) {
    let val = tree.nodes[node].get_value();
    let parent = tree.nodes[node].get_parent().unwrap();
    let x1 = val / 2;
    let x2 = (val as f32 / 2.0).ceil() as u32;

    let n1 = tree.add_node_leaf(x1);
    let n2 = tree.add_node_leaf(x2);
    let new_node = tree.add_node_pair(n1, n2);
    tree.nodes[n1].set_parent(new_node);
    tree.nodes[n2].set_parent(new_node);
    tree.nodes[new_node].set_parent(parent);

    tree.nodes[parent].replace_node(node, new_node);
}

fn find_split_needed(start: NodeId, tree: &mut Tree) -> Option<NodeId> {
    match tree.nodes[start] {
        Node::Pair {
            left,
            right,
            parent: _,
        } => {
            if let Some(id) = find_split_needed(left, tree) {
                return Some(id);
            }
            if let Some(id) = find_split_needed(right, tree) {
                return Some(id);
            }
        }
        Node::Leaf { value, parent: _ } => {
            if value >= 10 {
                return Some(start);
            }
        }
    }

    None
}

fn split_tree(start: NodeId, tree: &mut Tree) -> bool {
    let node = find_split_needed(start, tree);

    if let Some(node) = node {
        split_node(node, tree);
        return true;
    }

    false
}

fn reduce_tree(start: NodeId, tree: &mut Tree) {
    loop {
        while explode_tree(start, tree) {}
        if !split_tree(start, tree) {
            break;
        }
    }
}

fn get_tree_magnitude(start: NodeId, tree: &mut Tree) -> u32 {
    match tree.nodes[start] {
        Node::Pair {
            left,
            right,
            parent: _,
        } => 3 * get_tree_magnitude(left, tree) + 2 * get_tree_magnitude(right, tree),
        Node::Leaf { value, parent: _ } => value,
    }
}

fn main() {
    let data = include_str!("day18.txt").lines().collect::<Vec<&str>>();
    let mut tree = Tree { nodes: Vec::new() };

    let mut max_magnitude = 0;
    for i in 0..data.len() {
        for j in 0..data.len() {
            if i == j {
                continue;
            }
            let mut tree = Tree { nodes: Vec::new() };
            let t1 = parse_tree(&mut data[i].chars(), &mut tree);
            let t2 = parse_tree(&mut data[j].chars(), &mut tree);
            let t3 = add_trees(t1, t2, &mut tree);
            reduce_tree(t3, &mut tree);
            max_magnitude = std::cmp::max(get_tree_magnitude(t3, &mut tree), max_magnitude);
        }
    }
    println!("{}", max_magnitude);

    /*
    let mut current = parse_tree(&mut data.next().unwrap().chars(), &mut tree);
    while let Some(line) = data.next() {
        let extra = parse_tree(&mut line.chars(), &mut tree);
        current = add_trees(current, extra, &mut tree);
        reduce_tree(current, &mut tree);
    }
    tree.print_simple(current);
    */

    //println!("{}", get_tree_magnitude(current, &mut tree));
}
