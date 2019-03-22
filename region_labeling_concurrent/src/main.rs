#![allow(dead_code)]
#![warn(unused_must_use)]
extern crate crossbeam_channel;

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::env;
use std::time::{Instant};
use std::thread;
use crossbeam_channel::{unbounded, Receiver, Sender};

static P: usize = 2;
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
    
    let (result_tx, result_rx): (Sender<bool>, Receiver<bool>) = unbounded();
    let mut answer = Vec::<(Sender<bool>, Receiver<bool>)>::new();
	let mut first_image = Vec::<(Sender<Vec<Vec<u8>>>, Receiver<Vec<Vec<u8>>>)>::new();
    let mut first_label = Vec::<(Sender<Vec<Vec<u64>>>, Receiver<Vec<Vec<u64>>>)>::new();
    let mut second_image = Vec::<(Sender<Vec<Vec<u8>>>, Receiver<Vec<Vec<u8>>>)>::new();
	let mut second_label = Vec::<(Sender<Vec<Vec<u64>>>, Receiver<Vec<Vec<u64>>>)>::new();

    for _ in 0..P {
        answer.push(unbounded());
        first_image.push(unbounded());
		first_label.push(unbounded());
        second_image.push(unbounded());
		second_label.push(unbounded());
	}
    let mut childrens = Vec::new();
	let start = Instant::now();
    for w in 0..P {
		let grid = grid.to_vec();
		let matrix = matrix.to_vec();
		let first_image = first_image.clone();
		let first_label = first_label.clone();
		let second_image = second_image.clone();
		let second_label = second_label.clone();
		let result_tx = result_tx.clone();
		let answer = answer.clone();
		let m = matrix.len();
    	let n = matrix[0].len();
        // Each thread will send its id via the channel
        let child = thread::spawn(move || {
            let strip_size = m/P;
            let local_image = &mut vec![vec![0; n]; 1];
			let local_label = &mut vec![vec![0; n]; 1];
    		let last_element_image = &mut vec![vec![0; n]; 1];
			let last_element_label = &mut vec![vec![0; n]; 1];
			local_image.extend_from_slice(&matrix[(w*strip_size)..((w*strip_size)+strip_size)].to_vec());
			local_image.append(last_element_image);
			local_label.extend_from_slice(&grid[(w*strip_size)..((w*strip_size)+strip_size)].to_vec());
			local_label.append(last_element_label);
			let mut change : bool = true;
			if w != 0 {
				let f = local_image[1..2].to_vec();
				match first_image[w-1].0.send(f) {
					Err(e) => println!("{:?}", e),
					_ => ()
				}
			}
			if w != P - 1 {				
				let s = local_image[strip_size..strip_size+1].to_vec();
				match second_image[w+1].0.send(s) {
					Err(e) => println!("{:?}", e),
					_ => ()
				}
			}
			if w != P - 1 {
				let mut belowimage = first_image[w].1.recv().unwrap().to_vec();
				local_image.append(&mut belowimage);
				local_image.swap_remove(strip_size+1);
			}
			if w != 0 {
				let mut aboveimage = second_image[w].1.recv().unwrap().to_vec();
				local_image.append(&mut aboveimage);
				local_image.swap_remove(0);
			}
			while change {
				change = false;
				if w != 0 {
					let f = local_label[1..2].to_vec();
					match first_label[w-1].0.send(f) {
						Err(e) => println!("{:?}", e),
						_ => ()
					}
				}
				if w != P - 1 {
					let s = local_label[strip_size..strip_size+1].to_vec();
					match second_label[w+1].0.send(s) {
						Err(e) => println!("{:?}", e),
						_ => ()
					}
				}
				if w != P - 1 {
					let mut belowimage = first_label[w].1.recv().unwrap().to_vec();
					local_label.append(&mut belowimage);
					local_label.swap_remove(strip_size+1);
				}
				if w != 0 {
					let mut aboveimage = second_label[w].1.recv().unwrap().to_vec();
					local_label.append(&mut aboveimage);
					local_label.swap_remove(0);
				}
				for i in 1..strip_size+1 {
					for j in 0..n {
						let oldlabel = local_label[i][j];
						if local_image[i][j] == 1 {
							local_label[i][j] = max_neighbours(local_label, i, j);               
						}
						if local_label[i][j] != oldlabel {
							change = true;
						}
					}
				}
				match result_tx.send(change) {
					Err(e) => println!("{:?}", e),
					_ => ()
				}
				change = answer[w].1.recv().unwrap();
			}
        });

        childrens.push(child);
    }

    coordinator(result_rx, &mut answer);
    
    // Here, all the messages are collected
    /*let mut images = Vec::with_capacity(P as usize);
	let mut labels = Vec::with_capacity(P as usize);
    for _ in 0..P {
        // The `recv` method picks a message from the channel
        // `recv` will block the current thread if there are no messages available
        images.push(rx_image.recv());
		labels.push(rx_label.recv());
    }*/
    
    // Wait for the threads to complete any remaining work
   	for child in childrens {
        child.join().expect("oops! the child thread panicked");
    }

    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);

	/*for i in 0..P {
		println!("image -> {:?}", images[i]);
		println!("label -> {:?}", labels[i]);
    }*/  
    
}

fn coordinator(result: Receiver<bool>, answer: &mut Vec<(Sender<bool>, Receiver<bool>)>) {
    let mut chg : bool;
    let mut change : bool = true;
	while change {
		change = false;
		for _ in 0..P {
			chg = result.recv().unwrap();
			change = change || chg;
		}
        for i in 0..P {
			match answer[i as usize].0.send(change) {
				Err(e) => println!("{:?}", e),
				_ => ()
			}
        }
	}
	/*for i := 0; i < P; i++ {
		<-cresult[i]
	}*/
}
/*
fn worker(result: Sender<bool>, answer: &mut Vec<(Sender<bool>, Receiver<bool>)>, first: &mut Vec<(Sender<Vec<Vec<u64>>>, Receiver<Vec<Vec<u64>>>)>,
second: &mut Vec<(Sender<Vec<Vec<u64>>>, Receiver<Vec<Vec<u64>>>)>, w: i32, length: usize, n: usize) {

    
}*/
/*
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
*/
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
