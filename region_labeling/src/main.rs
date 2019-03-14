use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::env;


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
}
fn regionLabeling<T0, RT>(M: T0) -> RT {
    let start_time = time::time();
    let m = M.len();
    let mut n = M[0].len();
    let R = 0..m
        .iter()
        .map(|_| 0..n.iter().map(|_| 0).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    for i in 0..m {
        for j in 0..n {
            if M[i][j] == 1 {
                R[i][j] = i * n + j;
            }
        }
    }
    while true {
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
    }
    println!("{:?} ", "--- %s seconds ---" % time::time() - start_time);
    return R;
}
fn printMatrix<T0>(M: T0) {
    let s = M
        .iter()
        .map(|row| row.iter().map(|e| String::from(e)).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let lens = zip(starred!(s) /*unsupported*/)
        .iter()
        .map(|col| col.iter().map(len).iter().max().unwrap())
        .collect::<Vec<_>>();
    let fmt = "".join(lens.iter().map(|x| "{{:{}}}".format(x)).collect::<Vec<_>>());
    let table = s
        .iter()
        .map(|row| fmt.format(starred!(row) /*unsupported*/))
        .collect::<Vec<_>>();
    println!("{:?} ","".join(table));
}*/


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
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let buffered = BufReader::new(file);
    let mut matrix = vec![vec![]];
    for (_, line) in buffered.lines().enumerate() {
        let l = line.unwrap();
        let mut row = vec![];
        for (_, c) in l.chars().skip(1).enumerate() {
            row.push(c)
        }
        matrix.push(row)
    }
    for (_, row) in matrix.iter().enumerate() {
        for (_, col) in row.iter().enumerate() {
            print!("{}", col);
        }
        println!()
    }
        
    /*let R = regionLabeling(M);
    let m = R.len();
    let n = R[0].len();
    for i in 0..m {
        for j in 0..n {
            if R[i][j] > 0 {
                R[i][j] = 5;
            }
        }
    }
    printMatrix(R);*/
}