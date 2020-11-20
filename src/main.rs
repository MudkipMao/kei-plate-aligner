use std::env;

// Usage: cargo run <path-to-JSON> <grid spacing (default: 19.05mm)>
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let len_arg = args.len();

    // Script only takes up to 2 extra command line argument, the JSON file name
    assert!((len_arg <= 3) && (len_arg > 1), "Kei plate aligner only takes up to 2 arguments");

    let json_path = &args[1];
    let mut grid_spacing = 19.05; // 19.05mm grid spacing if not passed in
    if (len_arg == 3) {
        grid_spacing = args[2].trim().parse().expect("2nd argument must be a float");
    }
    
}
