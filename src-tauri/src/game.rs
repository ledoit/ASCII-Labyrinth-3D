use crate::maze::Maze;
use crate::raycast::{cast_ray, get_ascii_char};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameState {
    pub player_x: f64,
    pub player_y: f64,
    pub player_angle: f64,
    pub maze: MazeData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MazeData {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<bool>>,
}

impl From<&Maze> for MazeData {
    fn from(maze: &Maze) -> Self {
        MazeData {
            width: maze.width,
            height: maze.height,
            cells: maze.cells.clone(),
        }
    }
}

impl From<MazeData> for Maze {
    fn from(data: MazeData) -> Self {
        Maze {
            width: data.width,
            height: data.height,
            cells: data.cells,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerInput {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
    pub turn_left: bool,
    pub turn_right: bool,
}

impl GameState {
    pub fn new() -> Self {
        let maze = Maze::new(41, 41);
        GameState {
            player_x: 1.5,
            player_y: 1.5,
            player_angle: 0.0,
            maze: MazeData::from(&maze),
        }
    }

    pub fn update(&mut self, input: &PlayerInput) {
        let move_speed = 0.05;
        let turn_speed = 0.05;
        let maze: Maze = self.maze.clone().into();

        // Handle rotation
        if input.turn_left {
            self.player_angle -= turn_speed;
        }
        if input.turn_right {
            self.player_angle += turn_speed;
        }

        // Normalize angle
        self.player_angle = self.player_angle % (2.0 * std::f64::consts::PI);
        if self.player_angle < 0.0 {
            self.player_angle += 2.0 * std::f64::consts::PI;
        }

        // Handle movement
        let dx = self.player_angle.cos() * move_speed;
        let dy = self.player_angle.sin() * move_speed;

        if input.forward {
            let new_x = self.player_x + dx;
            let new_y = self.player_y + dy;
            if !maze.get_cell(new_x, new_y) {
                self.player_x = new_x;
                self.player_y = new_y;
            }
        }
        if input.backward {
            let new_x = self.player_x - dx;
            let new_y = self.player_y - dy;
            if !maze.get_cell(new_x, new_y) {
                self.player_x = new_x;
                self.player_y = new_y;
            }
        }
        if input.left {
            let left_angle = self.player_angle - std::f64::consts::PI / 2.0;
            let new_x = self.player_x + left_angle.cos() * move_speed;
            let new_y = self.player_y + left_angle.sin() * move_speed;
            if !maze.get_cell(new_x, new_y) {
                self.player_x = new_x;
                self.player_y = new_y;
            }
        }
        if input.right {
            let right_angle = self.player_angle + std::f64::consts::PI / 2.0;
            let new_x = self.player_x + right_angle.cos() * move_speed;
            let new_y = self.player_y + right_angle.sin() * move_speed;
            if !maze.get_cell(new_x, new_y) {
                self.player_x = new_x;
                self.player_y = new_y;
            }
        }
    }

    pub fn render_frame(&self, width: usize, height: usize) -> String {
        let maze: Maze = self.maze.clone().into();
        let fov = std::f64::consts::PI / 3.0; // 60 degrees
        let max_distance = 20.0;
        
        // Pre-calculate raycast results for each column
        let mut column_data: Vec<(f64, u8)> = Vec::with_capacity(width);
        for col in 0..width {
            let ray_angle = self.player_angle - fov / 2.0 + (col as f64 / width as f64) * fov;
            let result = cast_ray(self.player_x, self.player_y, ray_angle, &maze, max_distance);
            column_data.push((result.distance, result.wall_type));
        }
        
        let mut frame = String::new();
        
        // Render row by row
        for row in 0..height {
            for col in 0..width {
                let (distance, wall_type) = column_data[col];
                
                // Calculate wall height based on distance (perspective projection)
                let wall_height = if distance > 0.01 {
                    (height as f64 / distance).min(height as f64 * 2.0)
                } else {
                    height as f64 * 2.0
                };
                
                let wall_start = ((height as f64 - wall_height) / 2.0) as usize;
                let wall_end = (wall_start + wall_height as usize).min(height);
                
                if row < wall_start {
                    // Ceiling
                    frame.push(' ');
                } else if row < wall_end {
                    // Wall
                    let char = get_ascii_char(distance, wall_type, max_distance);
                    frame.push(char);
                } else {
                    // Floor
                    let floor_dist = calculate_floor_distance(
                        col,
                        row,
                        width,
                        height,
                        fov,
                        self.player_angle,
                    );
                    if floor_dist < max_distance {
                        frame.push(get_floor_char(floor_dist, max_distance));
                    } else {
                        frame.push(' ');
                    }
                }
            }
            
            if row < height - 1 {
                frame.push('\n');
            }
        }
        
        frame
    }
}

fn calculate_floor_distance(
    _col: usize,
    row: usize,
    _width: usize,
    height: usize,
    _fov: f64,
    _player_angle: f64,
) -> f64 {
    let p = (row as f64 - height as f64 / 2.0) / (height as f64 / 2.0);
    let distance = 1.0 / p.max(0.1);
    distance
}

fn get_floor_char(distance: f64, max_distance: f64) -> char {
    let normalized_dist = (distance / max_distance).min(1.0);
    if normalized_dist < 0.3 {
        '.'
    } else if normalized_dist < 0.6 {
        ','
    } else {
        ' '
    }
}

