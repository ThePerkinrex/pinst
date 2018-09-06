const PINST_VERSION:&str = "0.1.0"/*big.small.patch*/;
const PINST_BRANCH :&str = "alpha"/*alpha, nightly or stable*/;

/*
 Pinst uses a system that is simply separated in ports & ships.
 In ports there are a list of ships saying their name, where to find them & where to find the makefile to install them.
 Ships are just things to install, like formulae in homebrew.
 */

extern crate reqwest;

use std::env;

mod io;
mod toml;
mod ports;
mod ships;


fn main() {
    println!("Pinst {}-{}", PINST_VERSION, PINST_BRANCH);
    let mut args: Vec<String> = env::args().collect();
    if env::args().count() > 1 {
        args.remove(0);
        //println!("{:?}", args);
        let commandtmp:String = args[0].clone();
        let command:&str = commandtmp.as_ref();
        args.remove(0);
        match command {
            "help" => help(args),
            "port" => port(args),
            "ship" => ship(args),
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

    let github_ports = ports_toml.clone().get_property("github".to_string()).expect("Ports file error")
                                                               .get_array().expect("Port array error");
    if github_ports.len() > 0 {
        println!("Github ports: ");
    }
    for port in github_ports {
        let port_path = port.get_string().expect("Port path error");
        println!("{}", port_path);
    }

    let gitlab_ports = ports_toml.clone().get_property("gitlab".to_string()).expect("Ports file error")
                                                               .get_array().expect("Port array error");
    if gitlab_ports.len() > 0 {
        println!("Gitlab ports: ");
    }
    for port in gitlab_ports {
        let port_path = port.get_string().expect("Port path error");
        println!("{}", port_path);
    }

    let other_ports = ports_toml.clone().get_property("other".to_string()).expect("Ports file error")
                                                               .get_array().expect("Port array error");
    if other_ports.len() > 0 {
        println!("Other ports: ");
    }
    for port in other_ports {
        let port_path = port.get_string().expect("Port path error");
        println!("{}", port_path);
    }

    let file_ports = ports_toml.clone().get_property("files".to_string()).expect("Ports file error")
                                                            .get_array().expect("Port array error");
    if file_ports.len() > 0 {
        println!("File ports: ");
    }
    for port in file_ports {
        let port_path = port.get_string().expect("Port path error");
        println!("{}", port_path);
    }
}

fn ship(args: Vec<String>){
    if args.clone().len() > 0 {
        //println!("{:?}", args.clone());
        let command:String = String::from(args.clone()[0].as_ref());
        match command.as_ref(){
            "list" => ship_list(),
            _ => println!("Not a valid command"),
        }
    }
}

fn ship_list() {
    let ports_toml = toml::parse_file("ports.toml".to_string());

    let github_ports = ports_toml.clone().get_property("github".to_string()).expect("Ports file error")
                                                               .get_array().expect("Port array error");
    if github_ports.len() > 0 {
        println!("Github ports: ");
    }
    for port in github_ports {
        let port_path = port.get_string().expect("Port path error");
        println!(" - {}: ", port_path);
        for ship in ports::get_available_ships(port_path, 0){
            println!("   - {}", ship);
        }
    }

    let gitlab_ports = ports_toml.clone().get_property("gitlab".to_string()).expect("Ports file error")
                                                               .get_array().expect("Port array error");
    if gitlab_ports.len() > 0 {
        println!("Gitlab ports: ");
    }
    for port in gitlab_ports {
        let port_path = port.get_string().expect("Port path error");
        println!(" - {}: ", port_path);
        for ship in ports::get_available_ships(port_path, 1){
            println!("   - {}", ship);
        }
    }

    let other_ports = ports_toml.clone().get_property("other".to_string()).expect("Ports file error")
                                                               .get_array().expect("Port array error");
    if other_ports.len() > 0 {
        println!("Other ports: ");
    }
    for port in other_ports {
        let port_path = port.get_string().expect("Port path error");
        println!(" - {}: ", port_path);
        for ship in ports::get_available_ships(port_path, 2){
            println!("   - {}", ship);
        }
    }

    let file_ports = ports_toml.clone().get_property("files".to_string()).expect("Ports file error")
                                                            .get_array().expect("Port array error");
    if file_ports.len() > 0 {
        println!("File ports: ");
    }
    for port in file_ports {
        let port_path = port.get_string().expect("Port path error");
        println!(" - {}: ", port_path);
        for ship in ports::get_available_ships(port_path, 3){
            println!("   - {}", ship);
        }
    }
}
