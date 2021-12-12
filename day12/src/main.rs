use std::collections::HashMap;

fn get_max_visits(node: &str, visited: &mut HashMap<&str, u32>) -> u32 {
    match node {
        "start" => 1,
        "end" => 1,
        _ => {
            if node.chars().all(|c| c.is_lowercase()) {
                for key in visited.keys() {
                    if !key.chars().all(|c| c.is_lowercase()) {
                        continue;
                    }
                    if *visited.get(key).unwrap() >= 2 {
                        return 1;
                    }
                }
                return 2;
            } else {
                return u32::MAX;
            }
        }
    }
}

fn visit_node<'a>(
    node: &str,
    connections: &HashMap<&str, Vec<&'a str>>,
    visited: &mut HashMap<&'a str, u32>,
    path: &mut Vec<&'a str>,
) -> u32 {
    if node == "end" {
        //println!("{:?}", path);
        return 1;
    }

    let mut score = 0;
    if connections.contains_key(node) {
        for dest in connections[node].iter() {
            let max_visits = get_max_visits(dest, visited);
            if let Some(x) = visited.get_mut(dest) {
                if *x >= max_visits {
                    continue;
                } else {
                    *x += 1;
                }
            } else {
                visited.insert(dest, 1);
            }
            path.push(dest);
            score += visit_node(dest, connections, visited, path);
            path.pop();
            if let Some(x) = visited.get_mut(dest) {
                *x -= 1;
            }
        }
    }

    score
}

fn add_connection<'a>(
    start: &'a str,
    end: &'a str,
    connections: &mut HashMap<&'a str, Vec<&'a str>>,
) {
    if let Some(x) = connections.get_mut(start) {
        x.push(end);
    } else {
        let mut conn = Vec::new();
        conn.push(end);
        connections.insert(start, conn);
    }
}

fn main() {
    let data = include_str!("day12.txt");

    let mut connections: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in data.lines() {
        let parts: Vec<&str> = line.split("-").collect();
        let start = parts.get(0).unwrap();
        let end = parts.get(1).unwrap();

        add_connection(start, end, &mut connections);
        add_connection(end, start, &mut connections);
    }
    println!("{:?}", connections);

    let mut visited = HashMap::new();
    visited.insert("start", 1);
    let mut path = Vec::new();
    let s = visit_node("start", &connections, &mut visited, &mut path);

    println!("{}", s);
}
