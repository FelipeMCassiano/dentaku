mod calculator;
mod tokens;

use std::io::stdin;

use calculator::calculate;

fn main() {
    loop {
        let mut input = String::new();

        if let Err(e) = stdin().read_line(&mut input) {
            println!("{}", e);
            return;
        }

        match calculate(&mut input) {
            Ok(result) => {
                println!("{:?}", result.parse::<i32>().unwrap())
            }
            Err(e) => {
                eprintln!("{}", e)
            }
        }
    }
}
