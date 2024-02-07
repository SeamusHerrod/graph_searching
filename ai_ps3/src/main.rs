
use std::collections::VecDeque;

fn main() {

    let grid = ExampleGrid {
        grid: [
                [0,0,0,0,0,0,0,0],
                [0,0,1,1,1,1,0,0],
                [0,0,0,0,0,1,0,0],
                [0,0,0,0,0,1,0,0],
              ]
    };
    // dbg!(grid);
    let source: Node = Node {x: 3, y: 2, cost: 0};
    let dest: Node = Node {x: 3, y: 6, cost: 0};
    println!("source: {:?}", source);
    println!("dest: {:?}", dest);

    let solution = breadth_first_search(&grid, source, dest);
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
    cost: i32,

}

#[derive(Debug, Clone, PartialEq)]
struct Node {
    x: isize,
    y: isize,
    cost: i32,
}

fn breadth_first_search(grid: &ExampleGrid, source: Node, dest: Node) -> Option<Solution> {
    let moves = [(1,0), (0,-1), (-1,0), (0,1)]; // up, down, left, right 
    let mut visited = vec![vec![false; grid.grid[0].len()]; grid.grid.len()];
    let mut queue = VecDeque::new();

    visited[source.x as usize][source.y as usize] = true;
    queue.push_back(source.clone());
    while !queue.is_empty() {
        dbg!(&queue);
        for &(i, j) in &moves {
            let new_node = Node {x: source.x + i, y: source.y + j, cost: source.cost + 1};
            if grid.valid_move(new_node.clone()) && !visited[new_node.x as usize][new_node.y as usize] {
                visited[new_node.x as usize][new_node.y as usize] = true;
                queue.push_back(new_node.clone());
            
                if new_node == dest {
                    return Some(Solution {cost: new_node.cost});
                }
            }
        }
    }
    None
}


