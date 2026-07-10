use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long)]
    pub path: Option<String>,

    #[arg(short, long)]
    pub interactive: bool,

    #[arg(short, long, default_value_t = 2)]
    pub ratio: u32,
}
