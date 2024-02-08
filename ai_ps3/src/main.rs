
use std::collections::VecDeque;
use std::cmp::{Ord, Ordering, PartialOrd, Eq, PartialEq};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
fn main() {

    //let grid = ExampleGrid {
    //    grid: [
    //            [0,0,0,0,0,0,0,0],
    //            [0,0,1,1,1,1,0,0],
    //            [0,0,0,0,0,1,0,0],
    //            [0,0,0,0,0,1,0,0],
    //          ]
    //};
    // initialize the bug trap grid to all zeros (no obstacle)
    let mut bug_trap = ExampleGrid {
        grid: [[0; 100]; 100],
    };
    for i in 0..100 {
        for j in 0..100 {
            if i < 50 {
                let d = (i as i32 - 51).abs() + (j as i32 - 50).abs();
                if d == 50 {
                    bug_trap.grid[i][j] = 1;
                }
            }
            else {
                if j > 50 {
                    let d = (i as i32 - 50).abs() + (j as i32 - 75).abs();
                    if d == 25 {
                        bug_trap.grid[i][j] = 1;
                    }
                }
                if j < 50 {
                    let d = (i as i32 - 50).abs() + (j as i32 - 25).abs();
                    if d == 25 {
                        bug_trap.grid[i][j] = 1;
                    }
                }
            }
        }
    }

    // solving the example grid
    let source: Node = Node {x: 50, y: 55, cost: 0};
    let dest: Node = Node {x: 75, y: 70, cost: 0};
    let solution = breadth_first_search(&bug_trap, source, dest);
    
    println!("BFS cost: {}\tBFS Closed: {}\tBFS Fringe: {}\n", solution.unwrap().cost, solution.unwrap().closed, solution.unwrap().fringe);

    // solving the example grid with Greedy Best First Search
    let source: GreedyNode = GreedyNode {x: 50, y: 55, dist: 0, cost: 0};
    let dest: GreedyNode = GreedyNode {x: 75, y: 70, dist: 0, cost: 0};
    let solution = greedy_best_first_search(&bug_trap, source, dest);

    println!("GBFS cost: {}\tGBFS Closed: {}\tGBFS Fringe: {}\n", solution.unwrap().cost, solution.unwrap().closed, solution.unwrap().fringe);

    // solving the example grid with A* Search
    let source: GreedyNode = GreedyNode {x: 50, y: 55, dist: 0, cost: 0};
    let dest: GreedyNode = GreedyNode {x: 75, y: 70, dist: 0, cost: 0};
    let solution = a_star_search(&bug_trap, source, dest);
    println!("A* cost: {}\tA* Closed: {}\tA* Fringe: {}\n", solution.unwrap().cost, solution.unwrap().closed, solution.unwrap().fringe);

    // swap the source and destination
    println!("Swapping source and destination\n");
    // solving the example grid
    let source: Node = Node {x: 75, y: 70, cost: 0};
    let dest: Node = Node {x: 50, y: 55, cost: 0};
    let solution = breadth_first_search(&bug_trap, source, dest);

    println!("BFS cost: {}\tBFS Closed: {}\tBFS Fringe: {}\n", solution.unwrap().cost, solution.unwrap().closed, solution.unwrap().fringe);

    // solving the example grid with Greedy Best First Search
    let source: GreedyNode = GreedyNode {x: 75, y: 70, dist: 0, cost: 0};
    let dest: GreedyNode = GreedyNode {x: 50, y: 55, dist: 0, cost: 0};
    let solution = greedy_best_first_search(&bug_trap, source, dest);
    println!("GBFS cost: {}\tGBFS Closed: {}\tGBFS Fringe: {}\n", solution.unwrap().cost, solution.unwrap().closed, solution.unwrap().fringe);

    // solving the example grid with A* Search
    let source: GreedyNode = GreedyNode {x: 75, y: 70, dist: 0, cost: 0};
    let dest: GreedyNode = GreedyNode {x: 50, y: 55, dist: 0, cost: 0};
    let solution = a_star_search(&bug_trap, source, dest);
    println!("A* cost: {}\tA* Closed: {}\tA* Fringe: {}\n", solution.unwrap().cost, solution.unwrap().closed, solution.unwrap().fringe);

}

#[derive(Debug)]
//struct ExampleGrid {
//    grid: [[i32; 8];4],
//}
struct ExampleGrid {
    grid: [[i32; 100]; 100],
}
impl  ExampleGrid {
    fn valid_move (&self, node: Node) -> bool {
        node.x < self.grid.len().try_into().unwrap() 
        && node.y < self.grid[0].len().try_into().unwrap()  
        && self.grid[node.x as usize][node.y as usize] == 0
    }
}

#[derive(Debug)]
struct BugTrapGrid {
    grid: [[i32; 100]; 100],
}

#[derive(Debug, Copy, Clone)]
struct Solution {
    node: Node,
    cost: i32,
    closed: usize,
    fringe: i32,
}
#[derive(Debug, Copy, Clone)]
struct GreedySolution {
    node: GreedyNode,
    cost: i32,
    closed: usize,
    fringe: i32,

}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Node {
    x: isize,
    y: isize,
    cost: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

fn a_star_heuristic(node: &GreedyNode, dest: &GreedyNode) -> i32 {
    manhatten_dist(node, dest) + node.cost
}

fn a_star_search(grid: &ExampleGrid, source: GreedyNode, dest: GreedyNode) -> Option<GreedySolution>{
    let moves = [(1,0), (0,-1), (-1,0), (0,1)]; // up, down, left, right
    let mut visited = vec![vec![false; grid.grid[0].len()]; grid.grid.len()];
    let mut queue = BinaryHeap::new();

    visited[source.x as usize][source.y as usize] = true;
    queue.push(Reverse(GreedyNode {x: source.x, y: source.y, dist: a_star_heuristic(&source, &dest), cost: 0}));

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
                    dist: a_star_heuristic(&GreedyNode {x: cur_node.x + i, y: cur_node.y + j, dist: cur_node.dist + 1, cost: cur_node.cost+1}, &dest),
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
