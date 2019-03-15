
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::env;
use std::fmt::{Display, Formatter, Error};

struct NumVec(Vec<Vec<u32>>);

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
}

/*fn maxNeighbours<T0, T1, T2, RT>(M: T0, i: T1, j: T2) -> RT {
    let mut neighbour = vec![];
    if i > 0 {
        neighbour.push(M[i - 1][j]);
    }
    if i < M.len() - 1 {
        neighbour.push(M[i + 1][j]);
    }
    if j > 0 {
        neighbour.push(M[i][j - 1]);
    }
    if j < M[i].len() - 1 {
        neighbour.push(M[i][j + 1]);
    }
    return neighbour.iter().max().unwrap();
}*/

fn region_labeling(matrix : Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let m : usize = matrix.len();
    let n : usize = matrix[0].len();
    let mut grid : Vec<Vec<u32>> = Vec::with_capacity(n * m);
    for (i, row) in grid.iter_mut().enumerate() {
        for (j, col) in row.iter_mut().enumerate() {
            if matrix[i][j] == 1 {
                let value = i as u32  * n as u32  + j as u32;
                *col = value;
            }
        }
    }
    /*while true {
        let mut change = false;
        for i in 0..m {
            n = M[i].len();
            for j in 0..n {
                let oldlabel = R[i][j];
                if M[i][j] == 1 {
                    R[i][j] = maxNeighbours(R, i, j);
                }
                if R[i][j] != oldlabel {
                    change = true;
                }
            }
        }
        if !change {
            break;
        }
    }*/
    grid
}



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
    const RADIX: u32 = 10;
    let buffered = BufReader::new(file);
    let mut matrix : Vec<Vec<u32>> = Vec::new();
    for (_, line) in buffered.lines().enumerate() {
        let l = line.unwrap();
        let mut row : Vec<u32> = Vec::new();
        //let mut row = vec![];
        for (_, c) in l.chars().map(|c| c.to_digit(RADIX).unwrap()).skip(1).enumerate() {
            row.push(c);
        }
        matrix.push(row);
    }

    let v = region_labeling(matrix);
    

    println!("{}", v.len());

    /*for (_, row) in matrix.iter().enumerate() {
        for (_, col) in row.iter().enumerate() {
            print!("{:?}", col);
        }
        println!()
    }*/
    //print!("{:?}", matrix[0])

    /*let R = regionLabeling(M);
    */
}