/*

var cresult [P]chan [][]int


// Return the maximum value in an integer array.
func max(arr []int) int {
	max_num := arr[0]
	for _, elem := range arr {
		if max_num < elem {
			max_num = elem
		}
	}
	return max_num
}

// Return the maximun neighbour in an matrix at the position i, j
func maxNeighbours(M [][]int, i int, j int) int {
	neighbour := make([]int, 0)
	if i > 0 {
		neighbour = append(neighbour, M[i-1][j])
	}
	if i < (len(M) - 1) {
		neighbour = append(neighbour, M[i+1][j])
	}
	if j > 0 {
		neighbour = append(neighbour, M[i][j-1])
	}
	if j < len(M[i])-1 {
		neighbour = append(neighbour, M[i][j+1])
	}
	return max(neighbour)
}

func replace(i int, matrix [][]int, value [][]int) [][]int {
	//matrix = append(matrix[:i], matrix[i+1:]...)
	matrix = matrix[:i+copy(matrix[i:], matrix[i+1:])]
	matrix = append(matrix[:i], append(value, matrix[i:]...)...)
	return matrix
}

func worker(w int, length int, n int, wg *sync.WaitGroup) {
	stripSize := length / P
	localimage := make([][]int, 2) // local values plus edges
	locallabel := make([][]int, 2) // from neighbors
	for i := range localimage {
		localimage[i] = make([]int, n)
		locallabel[i] = make([]int, n)
	}
	change := true
	limage := make([][]int, len(image[(w*stripSize):((w*stripSize)+stripSize)]))
	copy(limage, image[(w*stripSize):((w*stripSize)+stripSize)])
	llabel := make([][]int, len(label[(w*stripSize):((w*stripSize)+stripSize)]))
	copy(llabel, label[(w*stripSize):((w*stripSize)+stripSize)])
	localimage = append(localimage[:1], append(limage, localimage[1:]...)...)
	locallabel = append(locallabel[:1], append(llabel, locallabel[1:]...)...)
	// exchange edges of image with neighbors
	if w != 0 {
		f := make([][]int, len(localimage[1:2]))
		copy(f, localimage[1:2])
		first[w-1] <- f
	}
	if w != P-1 {
		s := make([][]int, len(localimage[stripSize : stripSize+1]))
		copy(s, localimage[stripSize : stripSize+1])
		second[w+1] <- s
	}
	if w != P-1 {
		var belowimage [][]int
		belowimage = <-first[w]		
		localimage = replace(stripSize+1, localimage, belowimage)
	}
	if w != 0 {
		var aboveimage [][]int
		aboveimage = <-second[w]
		localimage = replace(0, localimage, aboveimage)
	}
	for change {
		change = false
		if w != 0 {
			f := make([][]int, len(locallabel[1:2]))
			copy(f, locallabel[1:2])
			first[w-1] <- f
		}
		if w != P-1 {
			s := make([][]int, len(locallabel[stripSize : stripSize+1]))
			copy(s, locallabel[stripSize : stripSize+1])
			second[w+1] <- s
		}
		if w != P-1 {
			var belowimage [][]int
			belowimage = <-first[w]
			locallabel = replace(stripSize+1, locallabel, belowimage)
		}
		if w != 0 {
			var aboveimage [][]int
			aboveimage = <-second[w]
			locallabel = replace(0, locallabel, aboveimage)
		}
		for i := 1; i <= stripSize; i++ {
			for j := 0; j < n; j++ {
				oldlabel := locallabel[i][j]
				if localimage[i][j] == 1 {
					locallabel[i][j] = maxNeighbours(locallabel, i, j)
				}
				if locallabel[i][j] != oldlabel {
					change = true
				}
			}
		}
		result <- change
		change = <-answer[w]
	}
	cresult[w] <- locallabel[1:len(locallabel)-1]
	wg.Done()
}
*/

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::env;
//use std::time::{Instant};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

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
    
    let (result_tx, result_rx): (Sender<bool>, Receiver<bool>) = mpsc::channel();
    let mut answer = Vec::<(Sender<bool>, Receiver<bool>)>::new();
	let first_image = Vec::<(Sender<Vec<Vec<u8>>>, Receiver<Vec<Vec<u8>>>)>::new();
    let first_label = Vec::<(Sender<Vec<Vec<u64>>>, Receiver<Vec<Vec<u64>>>)>::new();
    let mut second_image = Vec::<(Sender<Vec<Vec<u8>>>, Receiver<Vec<Vec<u8>>>)>::new();
	let mut second_label = Vec::<(Sender<Vec<Vec<u64>>>, Receiver<Vec<Vec<u64>>>)>::new();

    for _ in 0..P {
        answer.push(mpsc::channel());   
        first_image.push(mpsc::channel());
		first_label.push(mpsc::channel());
        second_image.push(mpsc::channel());
		second_label.push(mpsc::channel());
	}

    let (tx_image, rx_image): (Sender<Vec<Vec<u8>>>, Receiver<Vec<Vec<u8>>>) = mpsc::channel();
	let (tx_label, rx_label): (Sender<Vec<Vec<u64>>>, Receiver<Vec<Vec<u64>>>) = mpsc::channel();
    let mut childrens = Vec::new();
    for w in 0..P {
		let grid = grid.to_vec();
		let matrix = matrix.to_vec();
		let first_sender_image : Sender<Vec<Vec<u8>>>;
		if w != P - 1 {
			first_sender_image = first_image[w-1].0.clone();
		}else{
			first_sender_image = first_image[w].0.clone();
		}
		let first_sender_label : Sender<Vec<Vec<u64>>>;
		if w != P - 1 {
			first_sender_label = first_label[w-1].0.clone();
		}else{
			first_sender_label = first_label[w].0.clone();
		}
		let second_sender_image : Sender<Vec<Vec<u8>>>;		
		if w != P - 1 {
			second_sender_image = second_image[w+1].0.clone();
		}else{
			second_sender_image = second_image[w].0.clone();
		}
		let second_sender_label : Sender<Vec<Vec<u64>>>;		
		if w != P - 1 {
			second_sender_label = second_label[w+1].0.clone();
		}else{
			second_sender_label = second_label[w].0.clone();
		}
		/*let first_receive_image : Receiver<Vec<Vec<u8>>>;
		if w != P - 1 {
			first_receive_image = first_image[w-1].1.clone();
		}else{
			first_receive_image = first_image[w].1.clone();
		}
		let first_receiver_label : Receiver<Vec<Vec<u64>>>;
		if w != P - 1 {
			first_receiver_label = first_label[w-1].1.clone();
		}else{
			first_receiver_label = first_label[w].1.clone();
		}
		let second_receiver_image : Receiver<Vec<Vec<u8>>>;		
		if w != P - 1 {
			second_receiver_image = second_image[w+1].1.clone();
		}else{
			second_receiver_image = second_image[w].1.clone();
		}
		let second_receiver_label : Receiver<Vec<Vec<u64>>>;		
		if w != P - 1 {
			second_receiver_label = second_label[w+1].1.clone();
		}else{
			second_receiver_label = second_label[w].1.clone();
		}*/

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
			if w != 0 {
				let f = local_image[1..2].to_vec();
				//first_sender_image.send(f)
				first_image[w-1].0.send(f);
			}
			if w != P - 1 {
				let s = local_image[strip_size..strip_size+1].to_vec();
				second_sender_image.send(s);
			}
			if w != P - 1 {
				
				//let belowimage = first_receive_image.recv();
			}			
        });

        childrens.push(child);
    }    

    //coordinator(result_rx, answer);
    
    // Here, all the messages are collected
    let mut images = Vec::with_capacity(P as usize);
	let mut labels = Vec::with_capacity(P as usize);
    for _ in 0..P {
        // The `recv` method picks a message from the channel
        // `recv` will block the current thread if there are no messages available
        images.push(rx_image.recv());
		labels.push(rx_label.recv());
    }
    
    // Wait for the threads to complete any remaining work
    for child in childrens {
        child.join().expect("oops! the child thread panicked");
    }

	for i in 0..P {
		println!("image -> {:?}", images[i]);
		println!("label -> {:?}", labels[i]);
    }
    // Show the order in which the messages were sent
    

    /*let start = Instant::now();
    region_labeling(matrix, m, n, grid);
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);*/
    
    //print_matrix(grid)
}
/*
fn coordinator(result: Receiver<bool>, answer: &mut Vec<(Sender<bool>, Receiver<bool>)>) {
    let mut chg : bool = true;
    let mut change : bool = true;
	while change {
		change = false;
		for _ in 0..P {
			chg = result.recv().unwrap();
			change = change || chg;
		}
        for i in 0..P {
            answer[i as usize].0.send(change);
        }		
	}
	/*for i := 0; i < P; i++ {
		<-cresult[i]
	}*/
}*/
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
}*/

/*
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
}*/
