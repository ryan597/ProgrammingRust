use std::env;
use text_colorizer::*;


#[derive(Debug)]  // tells compiler to generate code such that Arguments can be formatted with {:?} in println!
struct Arguments
{
    target: String,
    replacement: String,
    filename: String,
    output: String,
}


fn print_usage()
{
    eprintln!("{} - change occurances of one string into another", "quickreplace".green());
    eprintln!("Usage: quickreplace <target> <replacement> <INPUT> <OUTPUT>");
}


fn parse_args() -> Arguments
{
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 4
    {
        print_usage();
        eprintln!("{} wrong number of arguments: expected 4 but got {}.", "Error".red().bold(), args.len());
        std::process::exit(1);
    }

    Arguments
    {
        target: args[0].clone(),
        replacement: args[1].clone(),
        filename: args[2].clone(),
        output: args[3].clone()
    }
}


fn main() {
    let args = parse_args();
    println!("{:?}", args);
}
