use clap::Parser;

use crate::args::Args;

pub fn get_path_and_ratio() -> (String, u32) {
    let args = Args::parse();

    if args.interactive {
        println!("Please enter the path to the image file:");
        let mut input_path = String::new();
        std::io::stdin()
            .read_line(&mut input_path)
            .expect("Failed to read line");
        let input_path = input_path.trim().to_string();

        println!("Please enter the ratio (default is 2):");
        let mut input_ratio = String::new();
        std::io::stdin()
            .read_line(&mut input_ratio)
            .expect("Failed to read line");
        let input_ratio: u32 = input_ratio.trim().parse().unwrap_or(2);

        return (input_path, input_ratio);
    } else {
        if args.path.is_none() {
            println!(
                "Please provide a path to the image file using the --path argument or run in interactive mode using the --interactive flag."
            );
            std::process::exit(0);
        }

        return (args.path.unwrap(), args.ratio);
    }
}
