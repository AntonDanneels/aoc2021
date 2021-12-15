use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Edge {
    node: usize,
    cost: usize,
}

fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();
    let mut parent: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();

    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if position == goal {
            let mut current = position;
            while current != start {
                let x = current % 100;
                let y = current / 100;
                //println!("{:?}: {} {}", current, x, y);
                current = parent[current];
            }
            return Some(cost);
        }

        if cost > dist[position] {
            continue;
        }

        for edge in &adj_list[position] {
            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
            };

            if next.cost < dist[next.position] {
                heap.push(next);
                dist[next.position] = next.cost;
                parent[next.position] = position;
            }
        }
    }

    None
}

fn construct_adj_list(data: &Vec<Vec<u32>>) -> Vec<Vec<Edge>> {
    let mut edges = Vec::new();
    let height = data.len();
    let width = data[0].len();

    println!("{} {}", width, height);

    for y in 0..height {
        for x in 0..width {
            let mut neighbours = Vec::new();

            const OFFSETS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
            for (xx, yy) in OFFSETS {
                let index = (y as i32 + yy as i32) * width as i32 + x as i32 + xx as i32;
                let n_x = x as i32 + xx;
                let n_y = y as i32 + yy;
                if xx == yy || n_x < 0 || n_x >= width as i32 || n_y < 0 || n_y >= height as i32 {
                    continue;
                }
                neighbours.push(Edge {
                    node: index as usize,
                    cost: data[n_y as usize][n_x as usize] as usize,
                });
            }
            edges.push(neighbours);
        }
    }

    edges
}

fn main() {
    let data: Vec<Vec<u32>> = include_str!("day15.txt")
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let width = data[0].len();
    let height = data.len();
    let edges = construct_adj_list(&data);
    println!("{:?}", shortest_path(&edges, 0, width * height - 1));

    let width = width * 5;
    let height = height * 5;
    let mut data2 = vec![vec![0; width]; height];

    for y in 0..height {
        for x in 0..width {
            let extra = (x / (width / 5) + y / (height / 5)) as u32;
            let mut new_num = data[y % (height / 5)][x % (width / 5)] as u32 + extra;
            if new_num > 9 {
                new_num = new_num % 9;
            }
            data2[y][x] = new_num as u32;
        }
    }

    let edges = construct_adj_list(&data2);
    println!("{:?}", shortest_path(&edges, 0, width * height - 1));
}
