use std::collections::HashSet;

pub struct Maze {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<bool>>, // true = wall, false = empty
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Self {
        let mut maze = Maze {
            width,
            height,
            cells: vec![vec![true; width]; height],
        };
        maze.generate();
        maze
    }

    fn generate(&mut self) {
        // Recursive backtracking algorithm
        let mut stack: Vec<(usize, usize)> = Vec::new();
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        
        // Start at (1, 1) - ensure it's a path
        let start = (1, 1);
        self.cells[start.1][start.0] = false;
        visited.insert(start);
        stack.push(start);

        let mut rng_seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        
        while let Some(current) = stack.pop() {
            let neighbors = self.get_unvisited_neighbors(current, &visited);
            
            if !neighbors.is_empty() {
                stack.push(current);
                // Simple LCG random selection
                rng_seed = rng_seed.wrapping_mul(1103515245).wrapping_add(12345);
                let next = neighbors[rng_seed as usize % neighbors.len()];
                self.remove_wall_between(current, next);
                self.cells[next.1][next.0] = false;
                visited.insert(next);
                stack.push(next);
            }
        }

        // Ensure exit point
        self.cells[self.height - 2][self.width - 2] = false;
    }

    fn get_unvisited_neighbors(&self, pos: (usize, usize), visited: &HashSet<(usize, usize)>) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        let (x, y) = pos;

        if x > 2 && !visited.contains(&(x - 2, y)) {
            neighbors.push((x - 2, y));
        }
        if x < self.width - 2 && !visited.contains(&(x + 2, y)) {
            neighbors.push((x + 2, y));
        }
        if y > 2 && !visited.contains(&(x, y - 2)) {
            neighbors.push((x, y - 2));
        }
        if y < self.height - 2 && !visited.contains(&(x, y + 2)) {
            neighbors.push((x, y + 2));
        }

        neighbors
    }

    fn remove_wall_between(&mut self, a: (usize, usize), b: (usize, usize)) {
        let (ax, ay) = a;
        let (bx, by) = b;
        let mid_x = (ax + bx) / 2;
        let mid_y = (ay + by) / 2;
        self.cells[mid_y][mid_x] = false;
    }

    pub fn is_wall(&self, x: usize, y: usize) -> bool {
        if x >= self.width || y >= self.height {
            return true;
        }
        self.cells[y][x]
    }

    pub fn get_cell(&self, x: f64, y: f64) -> bool {
        let ix = x as usize;
        let iy = y as usize;
        self.is_wall(ix, iy)
    }
}


