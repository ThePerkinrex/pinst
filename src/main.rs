const PINST_VERSION:&str = "0.1.0"/*big.small.patch*/;
const PINST_BRANCH :&str = "alpha"/*alpha, nightly or stable*/;

/*
 Pinst uses a sistem that is simply separated in ports & ships.
 In ports there are a list of ships saying their name, where to find them & where to find the makefile to install them.
 Ships are just things to install, like formulae in homebrew.
 */

use std::env;

mod io;
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
    //println!("PORT PAGE");
    if args.clone().len() > 0 {
        //println!("{:?}", args.clone());
        let command:String = String::from(args.clone()[0].as_ref());
        match command.as_ref(){
            "list" => port_list(),
            _ => println!("Not a valid command"),
        }
    }
}

fn port_list() {
    let ports_toml = toml::parse_file("ports.toml".to_string());
    let file_ports = ports_toml.get_property("files".to_string()).expect("Ports file error")
                                                     .get_array().expect("Port array error");
    for port in file_ports {
        let port_path = port.get_string().expect("Port path error");
        println!("File: {}", port_path);
        let port_toml = toml::parse_file(port_path);
        for ship in port_toml.get_objects(){
            println!("Ship: {}", ship.name);
        }
    }
}
