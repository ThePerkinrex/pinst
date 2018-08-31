const PINST_VERSION:&str = "0.1.0"/*big.small.patch*/;
const PINST_BRANCH :&str = "alpha"/*alpha, nightly or stable*/;

/*
 Pinst uses a sistem that is simply separated in ports & containers.
 In ports there are a list of containers saying their name, where to find them & where to find the makefile to install them.
 Containers are just things to install, like formulae in homebrew.
 */

use std::env;
mod toml;

fn main() {
    println!("Pinst {}-{}", PINST_VERSION, PINST_BRANCH);
    let mut args: Vec<String> = env::args().collect();
    if env::args().count() > 0 {
        args.remove(0);
        //println!("{:?}", args);
        let commandtmp:String = args[0].clone();
        let command:&str = commandtmp.as_ref();
        args.remove(0);
        match command {
            "help" => help(args),
            "port" => port(args),
            _ => println!("Not a command"),
        }
    }else{
        println!("USAGE: pinst command [options]\n\n");
        println!("To see the list of available commands use 'pinst help'");
    }
}

fn help(args: Vec<String>){
    println!("HELP PAGE");
    if args.len() > 0 {
        println!("{:?}", args);
    }
}

fn port(args: Vec<String>){
    println!("PORT PAGE");
    if args.clone().len() > 0 {
        println!("{:?}", args.clone());
        let command:String = String::from(args.clone()[0].as_ref());
        match command.as_ref(){
            "list" => parse_test(),
            _ => println!("Not a valid command"),
        }
    }
}

fn parse_test() {
    /*
    hi = 7
    arr = [10,1,10]
    [obj]
    hello = "h"
    */

    println!("{}", toml::parse("hi = 7\narr = [10,90.9879,10]\n[obj]\nhello = \"h\"\n\n[obj inside obj]\nhello = \"h\""/*"[obj]\nhello = \"h\"\n\nhi=7\narr=[10,01,10]"*/.to_string()).to_string());
}
