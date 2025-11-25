pub struct RaycastResult {
    pub distance: f64,
    pub wall_type: u8, // 0-3 for N, S, E, W walls
    #[allow(dead_code)]
    pub hit_x: f64,
    #[allow(dead_code)]
    pub hit_y: f64,
}

pub fn cast_ray(
    start_x: f64,
    start_y: f64,
    angle: f64,
    maze: &crate::maze::Maze,
    max_distance: f64,
) -> RaycastResult {
    let dx = angle.cos();
    let dy = angle.sin();
    
    // DDA (Digital Differential Analyzer) algorithm for efficient grid traversal
    let mut map_x = start_x as i32;
    let mut map_y = start_y as i32;
    
    let delta_dist_x = if dx == 0.0 { 1e30 } else { (1.0 / dx).abs() };
    let delta_dist_y = if dy == 0.0 { 1e30 } else { (1.0 / dy).abs() };
    
    let mut side_dist_x: f64;
    let mut side_dist_y: f64;
    let step_x: i32;
    let step_y: i32;
    
    if dx < 0.0 {
        step_x = -1;
        side_dist_x = (start_x - map_x as f64) * delta_dist_x;
    } else {
        step_x = 1;
        side_dist_x = (map_x as f64 + 1.0 - start_x) * delta_dist_x;
    }
    
    if dy < 0.0 {
        step_y = -1;
        side_dist_y = (start_y - map_y as f64) * delta_dist_y;
    } else {
        step_y = 1;
        side_dist_y = (map_y as f64 + 1.0 - start_y) * delta_dist_y;
    }
    
    let mut hit = false;
    let mut side = 0; // 0 = x-side, 1 = y-side
    
    while !hit {
        if side_dist_x < side_dist_y {
            side_dist_x += delta_dist_x;
            map_x += step_x;
            side = 0;
        } else {
            side_dist_y += delta_dist_y;
            map_y += step_y;
            side = 1;
        }
        
        if map_x < 0 || map_y < 0 || map_x as usize >= maze.width || map_y as usize >= maze.height {
            break;
        }
        
        if maze.is_wall(map_x as usize, map_y as usize) {
            hit = true;
        }
    }
    
    let perp_wall_dist = if side == 0 {
        side_dist_x - delta_dist_x
    } else {
        side_dist_y - delta_dist_y
    };
    
    let distance = perp_wall_dist.min(max_distance);
    let hit_x = start_x + dx * distance;
    let hit_y = start_y + dy * distance;
    
    // Determine wall type based on which side was hit
    let wall_type = if side == 0 {
        if step_x > 0 { 3 } else { 2 } // East or West
    } else {
        if step_y > 0 { 1 } else { 0 } // South or North
    };
    
    RaycastResult {
        distance,
        wall_type,
        hit_x,
        hit_y,
    }
}


pub fn get_ascii_char(distance: f64, wall_type: u8, max_distance: f64) -> char {
    let normalized_dist = (distance / max_distance).min(1.0);
    
    // Choose character based on distance and wall type
    if normalized_dist < 0.1 {
        match wall_type {
            0 => '█', // North
            1 => '█', // South
            2 => '█', // West
            3 => '█', // East
            _ => '█',
        }
    } else if normalized_dist < 0.3 {
        match wall_type {
            0 => '▓',
            1 => '▓',
            2 => '▓',
            3 => '▓',
            _ => '▓',
        }
    } else if normalized_dist < 0.5 {
        '▒'
    } else if normalized_dist < 0.7 {
        '░'
    } else {
        '·'
    }
}

#[allow(dead_code)]
pub fn get_color(distance: f64, max_distance: f64) -> u8 {
    let normalized_dist = (distance / max_distance).min(1.0);
    // Return brightness level (0-255, but we'll use 0-5 for terminal colors)
    if normalized_dist < 0.2 {
        5 // Very bright
    } else if normalized_dist < 0.4 {
        4
    } else if normalized_dist < 0.6 {
        3
    } else if normalized_dist < 0.8 {
        2
    } else {
        1 // Dark
    }
}

