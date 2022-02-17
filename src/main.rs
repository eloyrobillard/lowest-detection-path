mod test;
mod a_star;
mod graph;
mod input;
mod utils;

use a_star::{a_star};
use graph::{setup_graph};
use input::{get_input, format_input, Input};

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let input = get_input(args[1].as_str());
    let input_fmt: Input = format_input(input);
    let mut graph = setup_graph(&input_fmt);

    println!("{:.3}", a_star(&mut graph, (26/2, 0), (26/2, 25)))
}

