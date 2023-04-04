use std::io;
fn main() {
    println!("Fabonacci Sequence");
    loop {
        println!("Enter the number");
        let mut n = String::new();
        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read the line");
        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        println!("-------------");
        break fabonacci_seq(n);
    }
}

fn fabonacci_seq(n: u32) {
    let mut count = 0;
    let mut n_1: u32 = 0;
    let mut n_2: u32 = 1;
    println!("{n_1}");
    println!("{n_2}");
    let mut n_3: u32 ;
    while count < n - 2 {
        n_3 = n_2 + n_1;
        println!("{n_3}");
        n_1 = n_2;
        n_2 = n_3;
        count += 1;
    }
}

