use std::io;



fn main() {
    let grid: Vec<Vec<u8>> = vec![vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0]];

    display_grid(&grid);
}



fn display_grid(grid: &Vec<Vec<u8>>) {
    let mut display_string: String = String::new();

    for vec in grid.iter() {

        for char in vec.iter() {
            match char {
                0 => display_string.push_str("0"),
                1 => display_string.push_str("1"),
                2 => display_string.push_str("2"),
                _ => panic!("Unkown character") 
                // Assigned characters aren't assigned by user input, so if it isn't valid, then it is a problem in the program
            }
        }

        display_string.push_str("\n");
    }


    println!("{}", display_string)
}

fn create_character(row: u8, column: u8, grid: &mut Vec<Vec<u8>>) {
    //grid[row][column] = 1;
}

