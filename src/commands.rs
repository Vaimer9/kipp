use structopt::StructOpt;
use super::args;


pub fn math(args: args::Math) {


    match &*args.sus {
        "+" => { println!("{}", args.one + args.two) },
        "-" => { println!("{}", args.one - args.two) },
        "*" => { println!("{}", args.one * args.two) },
        "/" => { println!("{}", args.one / args.two) },
        &_ => { println!("Please enter the correct data")  }
    }

}