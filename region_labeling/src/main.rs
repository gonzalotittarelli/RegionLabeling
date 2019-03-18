
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::env;
use std::time::{Instant};
//use std::fmt::{Display, Formatter, Error};

const RADIX: u32 = 10;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filepath = &args[1];
    
    // Create a path to the desired file
    let path = Path::new(filepath);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(_) => panic!("couldn't open {}", display),
        Ok(file) => file,
    };    
    let buffered = BufReader::new(file);
    let mut matrix : Vec<Vec<u8>> = Vec::new();
    for (_, line) in buffered.lines().enumerate() {
        let l = line.unwrap();
        let mut row : Vec<u8> = Vec::new();
        for (_, c) in l.chars().map(|c| c.to_digit(RADIX).unwrap()).enumerate() {
            let value = c as u8;
            row.push(value);
        }
        matrix.push(row);
    }

    let m = matrix.len();
    let n = matrix[0].len();
    let grid = &mut vec![vec![0; n]; m];
    for (i, row) in grid.iter_mut().enumerate() {
        for (j, col) in row.iter_mut().enumerate() {
            if matrix[i][j] == 1 {
                let value = i as u64  * n as u64  + j as u64;
                *col = value;
            }
        }
    }
    let start = Instant::now();
    region_labeling(matrix, m, n, grid);
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);    
    
    //print_matrix(grid)
}

/*struct NumVec(Vec<Vec<u32>>);

impl Display for NumVec {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut comma_separated = String::new();

        for num in &self.0[0..self.0.len() - 1] {
            for num2 in &num[0..num.len() - 1] {
                comma_separated.push_str(&num2.to_string());
            }            
            comma_separated.push_str("\n")
        }        
        write!(f, "{}", comma_separated)
    }
}*/

fn print_matrix(matrix: &Vec<Vec<u64>>) {
    let mut comma_separated = String::new();

    for row in &matrix[0..matrix.len()] {
        for col in &row[0..row.len()] {
            if *col > 0 {
                comma_separated.push_str("*");
            }else{
                comma_separated.push_str(&col.to_string());
            }
        }
        comma_separated.push_str("\n")
    }        
    println!("{}", comma_separated);
}

fn max_neighbours(matrix: &Vec<Vec<u64>>, i: usize, j: usize) -> u64 {
    let mut neighbour : Vec<u64> = vec![];
    if i > 0 {
        neighbour.push(matrix[i - 1][j]);
    }
    if i < matrix.len() - 1 {
        neighbour.push(matrix[i + 1][j]);
    }
    if j > 0 {
        neighbour.push(matrix[i][j - 1]);
    }
    if j < matrix[i].len() - 1 {
        neighbour.push(matrix[i][j + 1]);
    }
    return *neighbour.iter().max().unwrap();
}

fn region_labeling(matrix : Vec<Vec<u8>>, m : usize, n : usize, g : &mut Vec<Vec<u64>>) {    
    loop {
        let mut change = false;
        for i in 0..m {
            for j in 0..n{
                let oldlabel = g[i][j];
                if matrix[i][j] == 1 {
                    g[i][j] = max_neighbours(g, i, j);                    
                }
                if g[i][j] != oldlabel {
                    change = true;
                }             
            }            
        }
        if !change {
            break;
        }
    }
}
