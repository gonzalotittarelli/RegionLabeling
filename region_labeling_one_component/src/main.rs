#![warn(dead_code)]
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::env;
use std::time::{Instant};
use std::collections::VecDeque;
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
    let mut matrix : Vec<Vec<u64>> = Vec::new();
    for (_, line) in buffered.lines().enumerate() {
        let l = line.unwrap();
        let mut row : Vec<u64> = Vec::new();
        for (_, c) in l.chars().map(|c| c.to_digit(RADIX).unwrap()).enumerate() {
            let value = c as u64;
            row.push(value);
        }
        matrix.push(row);
    }

    let m = matrix.len();
    let n = matrix[0].len();
    let grid = &mut vec![vec![0; n]; m];    
    let start = Instant::now();
    let components = one_component_at_time(matrix, m, n, grid);
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);    
    println!("Components: {}", components);    
    //print_matrix(grid)
}

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

fn find_neighbours(matrix : &Vec<Vec<u64>>, g : &mut Vec<Vec<u64>>, i: usize, j: usize, q : &mut VecDeque<[usize; 2]>, label : &u64) {
    if i > 0 {
        if matrix[i - 1][j] == 1 && g[i - 1][j] == 0 {
            g[i - 1][j] = *label;
            q.push_back([i - 1 , j]);
        }
    }
    if i < matrix.len() - 1 {
        if matrix[i + 1][j] == 1  && g[i + 1][j] == 0 {
            g[i + 1][j] = *label;
            q.push_back([i + 1, j]);
        }
    }
    if j > 0 {
        if matrix[i][j - 1] == 1 && g[i][j - 1] == 0 {
            g[i][j - 1] = *label;
            q.push_back([i, j - 1]);
        }
    }
    if j < matrix[i].len() - 1 {
        if matrix[i][j + 1] == 1 && g[i][j + 1] == 0 {
            g[i][j + 1] = *label;
            q.push_back([i, j + 1]);
        }
    }
}

/**
 *  1)Start from the first pixel in the image. Set current label to 1. Go to (2).
    2)If this pixel is a foreground pixel and it is not already labelled, give it the current label and add it as the first element in a queue, 
    then go to (3). If it is a background pixel or it was already labelled, then repeat (2) for the next pixel in the image.
    3)Pop out an element from the queue, and look at its neighbours (based on any type of connectivity). If a neighbour is a foreground pixel and 
    is not already labelled, give it the current label and add it to the queue. Repeat (3) until there are no more elements in the queue.
    4)Go to (2) for the next pixel in the image and increment current label by 1.
 */
fn one_component_at_time(matrix : Vec<Vec<u64>>, m : usize, n : usize, g : &mut Vec<Vec<u64>>) -> u32{
    let mut current_label : u64 = 1;
    let mut objects = 0;
    let queue: &mut VecDeque<[usize; 2]> = &mut VecDeque::new();
    for i in 0..m {
        for j in 0..n{
            if matrix[i][j] == 1 && g[i][j] == 0 {
                objects += 1;
                g[i][j] = current_label;
                queue.push_back([i, j]);
                while !queue.is_empty() {
                    let element : [usize; 2] = queue.pop_front().unwrap();
                    find_neighbours(&matrix, g, element[0], element[1], queue, &current_label)
                }
                current_label +=1;
            }
        }            
    }
    return objects;
}