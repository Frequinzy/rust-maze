use rand::prelude::*;

pub fn generate (current_x: usize, current_y: usize, grid: &mut Vec<Vec<u8>>) {
    let mut rng = thread_rng();
    /*
    north = 1;
    south = 2;
    east = 4;
    let west = 8;
    */
    let mut directions = [1, 2, 4, 8];
    directions.shuffle(&mut rng);
    for direction in directions {
        let new_x: usize = match direction {
            1 => current_x,
            2 => current_x,
            4 => current_x - 2,
            8 => current_x + 2,
            _ => panic!("No such direction! 1")
        };
        let new_y: usize = match direction {
            1 => current_y - 2,
            2 => current_y + 2,
            4 => current_y,
            8 => current_y,
            _ => panic!("No such direction! 2")
        };
        if 1 < new_x && new_x < grid.len()-2
            && 1 < new_y && new_y < grid[new_x].len()-2
            && grid[new_x][new_y] == 0 {
                grid[current_x][current_y] = 1;
                grid[(current_x+new_x)/2][(current_y+new_y)/2] = 1;
                grid[new_x][new_y] = 1;
                generate(new_x, new_y, grid);
        }
    }
    grid[2][1] = 2;
    let exit_x = grid.len()-4;
    let exit_y = grid.len()-3;
    grid[exit_x][exit_y] = 4;
}