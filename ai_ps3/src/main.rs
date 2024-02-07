
use std::collections::VecDeque;
use std::cmp::{Ord, Ordering, PartialOrd, Eq, PartialEq};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
fn main() {

    let grid = ExampleGrid {
        grid: [
                [0,0,0,0,0,0,0,0],
                [0,0,1,1,1,1,0,0],
                [0,0,0,0,0,1,0,0],
                [0,0,0,0,0,1,0,0],
              ]
    };

    // solving the example grid
    let source: Node = Node {x: 3, y: 2, cost: 0};
    let dest: Node = Node {x: 3, y: 6, cost: 0};
    let solution = breadth_first_search(&grid, source, dest);
    
    dbg!(solution);

    // solving the example grid with Greedy Best First Search
    let source: GreedyNode = GreedyNode {x: 3, y: 2, dist: 0, cost: 0};
    let dest: GreedyNode = GreedyNode {x: 3, y: 6, dist: 0, cost: 0};
    let solution = greedy_best_first_search(&grid, source, dest);

    dbg!(solution);
}

#[derive(Debug)]
struct ExampleGrid {
    grid: [[i32; 8];4],
}
impl  ExampleGrid {
    fn valid_move (&self, node: Node) -> bool {
        node.x < self.grid.len().try_into().unwrap() 
        && node.y < self.grid[0].len().try_into().unwrap()  
        && self.grid[node.x as usize][node.y as usize] == 0
    }
}

#[derive(Debug)]
struct Solution {
    node: Node,
    cost: i32,
    closed: usize,
    fringe: i32,

}
#[derive(Debug)]
struct GreedySolution {
    node: GreedyNode,
    cost: i32,
    closed: usize,
    fringe: i32,

}

#[derive(Debug, Clone, PartialEq)]
struct Node {
    x: isize,
    y: isize,
    cost: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GreedyNode {
    x: isize,
    y: isize,
    dist: i32,
    cost: i32,
}

impl Ord for GreedyNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist.cmp(&other.dist)
    }
}

impl PartialOrd for GreedyNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn breadth_first_search(grid: &ExampleGrid, source: Node, dest: Node) -> Option<Solution> {
    let moves = [(1,0), (0,-1), (-1,0), (0,1)]; // up, down, left, right 
    let mut visited = vec![vec![false; grid.grid[0].len()]; grid.grid.len()];
    let mut queue = VecDeque::new();

    visited[source.x as usize][source.y as usize] = true;
    queue.push_back(source.clone());
    while !queue.is_empty() {
        let cur_node = queue.pop_front().unwrap();
        //dbg!(&cur_node);
        let fringe = queue.len() as i32; // calculate fringe nodes
        if cur_node.x == dest.x && cur_node.y == dest.y{
            return Some(Solution {cost: cur_node.cost, node: cur_node.clone(), 
                                  closed: closed_set(&visited),
                                  fringe: fringe});
        }
        for &(i, j) in &moves {
            if cur_node.x + i >= 0 && cur_node.y + j >= 0
            && cur_node.x + i < grid.grid.len() as isize && cur_node.y + j < grid.grid[0].len() as isize {   
                let new_node = Node {x: cur_node.x + i, y: cur_node.y + j, cost: cur_node.cost + 1};
                if grid.valid_move(new_node.clone()) && !visited[new_node.x as usize][new_node.y as usize] {
                    visited[new_node.x as usize][new_node.y as usize] = true;
                    queue.push_back(new_node.clone());
                    
                }
            }
        }
        //dbg!(&queue);
    }
    None
}

fn closed_set(visited: &Vec<Vec<bool>>) ->  usize {
    //dbg!(&visited);
    let mut closed: usize = 0;
    for i in 0..visited.len() {
        for j in 0..visited[0].len() {
            if visited[i][j] {
                closed += 1;
            }
        }
    }
    closed
}

fn bfs_fringe_set(queue: VecDeque<Node>, visited: &Vec<Vec<bool>>, grid: &ExampleGrid) -> i32 {
    let mut fringe: i32 = 0;
    for node in queue {
        fringe += num_reachable_nodes(grid, &node, visited);
    }
    fringe
}

fn greedy_fringe_set(queue: BinaryHeap<Reverse<GreedyNode>>, visited: &Vec<Vec<bool>>, grid: &ExampleGrid) -> i32 {
    let mut fringe: i32 = 0;
    for node in queue {
        fringe += num_reachable_nodes(grid, &Node {x: node.0.x, y: node.0.y, cost: 0}, visited);
    }
    fringe
}

fn num_reachable_nodes(grid: &ExampleGrid, node: &Node, visited: &Vec<Vec<bool>>) -> i32 {
    let moves = [(1,0), (0,-1), (-1,0), (0,1)];
    let mut reachable = 0;
    for &(i,j) in &moves {
        if node.x + i >= 0 && node.y + j >= 0
        && node.x + i < grid.grid.len() as isize 
        && node.y + j < grid.grid[0].len() as isize {
            let new_node = Node {x: node.x + i, y: node.y + j, cost: 0};
            if grid.valid_move(new_node.clone()) && !visited[new_node.x as usize][new_node.y as usize] {
                reachable += 1;
            }
        }
    }
    reachable
}

fn greedy_best_first_search(grid: &ExampleGrid, source: GreedyNode, dest: GreedyNode) -> Option<GreedySolution>{
    let moves = [(1,0), (0,-1), (-1,0), (0,1)]; // up, down, left, right
    let mut visited = vec![vec![false; grid.grid[0].len()]; grid.grid.len()];
    let mut queue = BinaryHeap::new();

    visited[source.x as usize][source.y as usize] = true;
    queue.push(Reverse(GreedyNode {x: source.x, y: source.y, dist: manhatten_dist(&source, &dest), cost: 0}));

    while !queue.is_empty() {
        let cur_node_reverse = queue.pop().unwrap();
        let cur_node = cur_node_reverse.0;
        let fringe = queue.len() as i32;

        if cur_node.x == dest.x && cur_node.y == dest.y {
            return Some(GreedySolution {cost: cur_node.cost, 
                                  node: cur_node.clone(), 
                                  closed: closed_set(&visited),
                                  fringe: fringe});
        }

        for &(i, j) in &moves {
            if cur_node.x + i >= 0 && cur_node.y + j >= 0
            && cur_node.x + i < grid.grid.len() as isize 
            && cur_node.y + j < grid.grid[0].len() as isize {   
                let new_node = GreedyNode {x: cur_node.x + i,
                    y: cur_node.y + j,
                    dist: manhatten_dist(&GreedyNode {x: cur_node.x + i, y: cur_node.y + j, dist: 0, cost: 0}, &dest),
                    cost: cur_node.cost + 1};
                if grid.valid_move(Node {x: new_node.x, y: new_node.y, cost: 0}) && !visited[new_node.x as usize][new_node.y as usize] {
                    visited[new_node.x as usize][new_node.y as usize] = true;
                    queue.push(Reverse(new_node.clone()));
                }
            }
        }
    }
    None
}

fn manhatten_dist(node: &GreedyNode, dest: &GreedyNode) -> i32 {
    ((dest.x - node.x).abs() + (dest.y - node.y).abs()).try_into().unwrap()
}

