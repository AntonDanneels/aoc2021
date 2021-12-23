use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Clone, Debug)]
enum FishType {
    Amber,
    Bronze,
    Copper,
    Dessert
}

#[derive(Clone, Debug)]
struct Fish {
    fish_type: FishType,
    pos: usize,
    moves: u32
}

#[derive(Debug, PartialEq)]
enum GraphNodeType {
    RoomAmber,
    RoomBronze,
    RoomCopper,
    RoomDessert,
    Temporary,
    MoveOnly
}

#[derive(Debug)]
struct GraphNode {
    graph_type: GraphNodeType,
    id: usize,
    neighbors: Vec<usize>
}

#[derive(Debug)]
struct Graph {
    nodes: Vec<GraphNode>
}

impl Graph {
    fn add_node(&mut self, graph_type: GraphNodeType) -> usize {
        let result = self.nodes.len();

        self.nodes.push(GraphNode {graph_type, id: result, neighbors: Vec::new()} );

        result
    }
}

fn add_connection(n1: usize, n2: usize, graph: &mut Graph) {
    graph.nodes[n1].neighbors.push(n2);
    graph.nodes[n2].neighbors.push(n1);
}

fn build_graph() -> (Graph, HashMap<usize, Fish>) {
    let mut graph = Graph { nodes: Vec::new() };
    let mut fishes = HashMap::new();

    let p1 = graph.add_node(GraphNodeType::Temporary);
    let p2 = graph.add_node(GraphNodeType::Temporary);
    let p3 = graph.add_node(GraphNodeType::MoveOnly);
    let p4 = graph.add_node(GraphNodeType::Temporary);
    let p5 = graph.add_node(GraphNodeType::MoveOnly);
    let p6 = graph.add_node(GraphNodeType::Temporary);
    let p7 = graph.add_node(GraphNodeType::MoveOnly);
    let p8 = graph.add_node(GraphNodeType::Temporary);
    let p9 = graph.add_node(GraphNodeType::MoveOnly);
    let p10 = graph.add_node(GraphNodeType::Temporary);
    let p11 = graph.add_node(GraphNodeType::Temporary);

    let r11 = graph.add_node(GraphNodeType::RoomAmber);
    let r12 = graph.add_node(GraphNodeType::RoomAmber);
    let r13 = graph.add_node(GraphNodeType::RoomAmber);
    let r14 = graph.add_node(GraphNodeType::RoomAmber);

    let r21 = graph.add_node(GraphNodeType::RoomBronze);
    let r22 = graph.add_node(GraphNodeType::RoomBronze);
    let r23 = graph.add_node(GraphNodeType::RoomBronze);
    let r24 = graph.add_node(GraphNodeType::RoomBronze);

    let r31 = graph.add_node(GraphNodeType::RoomCopper);
    let r32 = graph.add_node(GraphNodeType::RoomCopper);
    let r33 = graph.add_node(GraphNodeType::RoomCopper);
    let r34 = graph.add_node(GraphNodeType::RoomCopper);

    let r41 = graph.add_node(GraphNodeType::RoomDessert);
    let r42 = graph.add_node(GraphNodeType::RoomDessert);
    let r43 = graph.add_node(GraphNodeType::RoomDessert);
    let r44 = graph.add_node(GraphNodeType::RoomDessert);

    add_connection(p1, p2, &mut graph);
    add_connection(p2, p3, &mut graph);

    add_connection(p3, p4, &mut graph);
    add_connection(p3, r11, &mut graph);
    add_connection(r11, r12, &mut graph);
    add_connection(r12, r13, &mut graph);
    add_connection(r13, r14, &mut graph);

    add_connection(p4, p5, &mut graph);
    add_connection(p5, r21, &mut graph);
    add_connection(r21, r22, &mut graph);
    add_connection(r22, r23, &mut graph);
    add_connection(r23, r24, &mut graph);

    add_connection(p5, p6, &mut graph);
    add_connection(p6, p7, &mut graph);
    add_connection(p7, r31, &mut graph);
    add_connection(r31, r32, &mut graph);
    add_connection(r32, r33, &mut graph);
    add_connection(r33, r34, &mut graph);

    add_connection(p7, p8, &mut graph);
    add_connection(p8, p9, &mut graph);
    add_connection(p9, r41, &mut graph);
    add_connection(r41, r42, &mut graph);
    add_connection(r42, r43, &mut graph);
    add_connection(r43, r44, &mut graph);

    add_connection(p9, p10, &mut graph);
    add_connection(p10, p11, &mut graph);

    fishes.insert(r11, Fish {fish_type: FishType::Bronze, pos: r11, moves: 0});
    fishes.insert(r12, Fish {fish_type: FishType::Dessert, pos: r12, moves: 0});
    fishes.insert(r13, Fish {fish_type: FishType::Dessert, pos: r13, moves: 0});
    fishes.insert(r14, Fish {fish_type: FishType::Amber, pos: r14, moves: 0});

    fishes.insert(r21, Fish {fish_type: FishType::Copper, pos: r21, moves: 0});
    fishes.insert(r22, Fish {fish_type: FishType::Copper, pos: r22, moves: 0});
    fishes.insert(r23, Fish {fish_type: FishType::Bronze, pos: r23, moves: 0});
    fishes.insert(r24, Fish {fish_type: FishType::Dessert, pos: r24, moves: 0});

    fishes.insert(r31, Fish {fish_type: FishType::Bronze, pos: r31, moves: 0});
    fishes.insert(r32, Fish {fish_type: FishType::Bronze, pos: r32, moves: 0});
    fishes.insert(r33, Fish {fish_type: FishType::Amber, pos: r33, moves: 0});
    fishes.insert(r34, Fish {fish_type: FishType::Copper, pos: r34, moves: 0});

    fishes.insert(r41, Fish {fish_type: FishType::Dessert, pos: r41, moves: 0});
    fishes.insert(r42, Fish {fish_type: FishType::Amber, pos: r42, moves: 0});
    fishes.insert(r43, Fish {fish_type: FishType::Copper, pos: r43, moves: 0});
    fishes.insert(r44, Fish {fish_type: FishType::Amber, pos: r44, moves: 0});

    (graph, fishes)
}

fn is_valid_location(fish: &Fish, node: &GraphNodeType) -> bool {
    match fish.fish_type {
        FishType::Amber => {
            if *node == GraphNodeType::RoomAmber {
                return true;
            }
        },
        FishType::Bronze => {
            if *node == GraphNodeType::RoomBronze {
                return true;
            }
        },
        FishType::Copper => {
            if *node == GraphNodeType::RoomCopper {
                return true;
            }
        },
        FishType::Dessert => {
            if *node == GraphNodeType::RoomDessert {
                return true;
            }
        },
    }

    false
}

fn find_all_paths(graph: &Graph, fishes: &HashMap<usize, Fish>, fish: &Fish) -> Vec<(usize, usize)> {
    let mut result = Vec::new();

    if fish.moves >= 2 {
        return result;
    }

    let mut visited = HashSet::new();
    let mut possible_neighbors = Vec::new();
    for n in graph.nodes[fish.pos].neighbors.iter() {
        visited.insert(*n);
        possible_neighbors.push((*n, 1));
    }

    while !possible_neighbors.is_empty() {
        let (pos, cost) = possible_neighbors.pop().unwrap();

        if let Some(other) = fishes.get(&pos) {

            continue;
        }

        if fish.moves == 1 {
            if is_valid_location(fish, &graph.nodes[pos].graph_type) {
                result.push((pos, cost));
            }
        } else {
            match graph.nodes[pos].graph_type {
                GraphNodeType::Temporary => result.push((pos, cost)),
                GraphNodeType::MoveOnly => {},
                _ => {}
            }
        }

        for n in graph.nodes[pos].neighbors.iter() {
            if !visited.contains(&n) {
                visited.insert(*n);
                possible_neighbors.push((*n, cost + 1));
            }
        }
    }

    result
}

fn print_at_pos(fishes: &HashMap<usize, Fish>, n: usize) {
    if let Some(x) = fishes.get(&n) {
        match x.fish_type {
            FishType::Amber => print!("A"),
            FishType::Bronze => print!("B"),
            FishType::Copper => print!("C"),
            FishType::Dessert => print!("D"),
        }
    } else {
        print!(".");
    }
}

fn render_map(fishes: &HashMap<usize, Fish>) {
    for n in 0..11 {
        print_at_pos(fishes, n);
    }
    println!();

    for i in 0..4 {
        for j in 0..4 {
            print!("  ");
            print_at_pos(fishes, 11 + j * 4 + i);
        }
        println!();
    }

    println!();
}

fn has_won(graph: &Graph, fishes: &mut HashMap<usize, Fish>) -> bool {
    let mut reached_end = true;
    for n in 0..graph.nodes.len() {
        match graph.nodes[n].graph_type {
            GraphNodeType::RoomAmber => {
                if let Some(f) = fishes.get(&n) {
                    if f.fish_type == FishType::Amber {
                        continue;
                    }
                }
                reached_end = false;
                break;
            },
            GraphNodeType::RoomBronze => {
                if let Some(f) = fishes.get(&n) {
                    if f.fish_type == FishType::Bronze {
                        continue;
                    }
                }
                reached_end = false;
                break;
            },
            GraphNodeType::RoomCopper => {
                if let Some(f) = fishes.get(&n) {
                    if f.fish_type == FishType::Copper {
                        continue;
                    }
                }
                reached_end = false;
                break;
            },
            GraphNodeType::RoomDessert => {
                if let Some(f) = fishes.get(&n) {
                    if f.fish_type == FishType::Dessert {
                        continue;
                    }
                }
                reached_end = false;
                break;
            }
            _ => {}
        }
    }

    reached_end
}

fn find_solutions(graph: &Graph, fishes: &mut HashMap<usize, Fish>, iteration: u32, current_cost: usize, current_min: &mut usize) {

    //println!("--------------------------------");
    //println!("Fishes: {:?}", fishes);
    render_map(fishes);
    //std::io::stdin().read_line(&mut String::new());
    if has_won(graph, fishes) {
        println!("Found a solution!: {:#?}, cost: {}", fishes, current_cost);

        if current_cost < *current_min {
            *current_min = current_cost;
        }
        //std::io::stdin().read_line(&mut String::new());
    }

    for (pos, fish) in fishes.iter() {
        let mut dests = find_all_paths(graph, fishes, &fish);
        if dests.len() == 0 {
            //println!("Fish has no path to go to: {:?}", fish);
        }
        dests.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        for (dest, cost) in dests.iter().rev() {
            let mut new_fishes = fishes.clone();
            let mut f = new_fishes.remove(pos).unwrap();
            f.moves += 1;
            f.pos = *dest;
            let cost = match f.fish_type {
                FishType::Amber => cost * 1,
                FishType::Bronze  => cost * 10,
                FishType::Copper => cost * 100,
                FishType::Dessert => cost * 1000,
            };
            if cost + current_cost >= *current_min {
                continue;
            }
            new_fishes.insert(*dest, f);
            //println!("Moving {:?} to {}", fish, dest + 1);
            //println!("--------------------------------");
            find_solutions(graph, &mut new_fishes, iteration + 1, current_cost + cost, current_min);
        }
    }
}

fn main() {
    let (graph, mut fish) = build_graph();

    println!("Current graph: {:#?}", graph);
    println!("Current fish: {:?}", fish);

    let mut min = usize::MAX;
    find_solutions(&graph, &mut fish, 0, 0, &mut min);

    println!("Min: {}", min);
}
