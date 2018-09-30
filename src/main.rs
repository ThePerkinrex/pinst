const PINST_VERSION:&str = "0.3.0"/*big.small.patch*/;
const PINST_BRANCH :&str = "alpha"/*alpha, nightly or stable*/;

/*
 Pinst uses a system that is simply separated in ports & ships.
 In ports there are a list of ships saying their name, where to find them & where to find the makefile to install them.
 Ships are just things to install, like formulae in homebrew.
 */

extern crate reqwest;

extern crate colored;


use std::env;

mod io;
mod toml;
mod ports;
mod ships;


use colored::Colorize;

fn main() {
    println!("Pinst {}-{} {}", PINST_VERSION, PINST_BRANCH, env!("TARGET"));
    if !io::path_exists("~/.pinst".to_string()) {
        println!("{}", "Setting up pinst".yellow().bold());
        setup();
        println!("{}", "Pinst setup".yellow().bold());
    }
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
            "install" => install(args),
            "uninstall" => uninstall(args),
            "update" => update_comm(args),
            _ => println!("Not a command"),
        }
    }else{
        println!("USAGE: pinst command [options]\n\n");
        println!("To see the list of available commands use 'pinst help'");

    }
}

fn setup(){
    // Setting up the .pinst dir
    io::create_empty_dir("~/.pinst".to_string());
    io::add_path_to_rc("~/.pinst".to_string());
    // Copying the pinst executable into the .pinst dir
    let exe = env::current_exe().expect("failed to get the current executable");
    io::copy(exe.to_str().expect("str expected").to_string(), "~/.pinst/pinst".to_string());
    // Creating the installed ships file
    io::create_empty_file("~/.pinst/ships.toml".to_string());
    io::overwrite("~/.pinst/ships.toml".to_string(), "[pinst]\n".to_string());
    io::write("~/.pinst/ships.toml".to_string(), "version = \"v".to_string() + PINST_VERSION + "-" + PINST_BRANCH + "\"\n\n");

    // Creating the ports.toml
    io::create_empty_file("~/.pinst/ports.toml".to_string());
    let mut gh:Vec<toml::TOMLValue> = Vec::new();
    gh.push(toml::TOMLValue::get_new_string("theperkinrex/pinst_port".to_string()));
    let mut ports_toml: toml::TOML = toml::TOML::new_w_explicit_name("TOML");
    ports_toml = ports_toml.add_property("github".to_string(), toml::TOMLValue::get_new_array(gh));
    ports_toml = ports_toml.add_property("gitlab".to_string(), toml::TOMLValue::get_new_array(Vec::new()));
    ports_toml = ports_toml.add_property("other".to_string(), toml::TOMLValue::get_new_array(Vec::new()));
    ports_toml = ports_toml.add_property("files".to_string(), toml::TOMLValue::get_new_array(Vec::new()));
    io::overwrite("~/.pinst/ports.toml".to_string(), ports_toml.to_string());
}

fn help(args: Vec<String>){
    //println!("HELP PAGE");
    if args.clone().len() > 0 {
        println!("Help for {}", args[0]);
    }else{
        println!("pinst help [command]: prints this page");
        println!("pinst port <command>: pinst port commands");
        println!("pinst ship <command>: pinst ship commands");
        println!("pinst install <ship_name>: install a ship named `ship_name`");
        println!("pinst uninstall <ship_name>: uninstall a ship named `ship_name`");
        println!("pinst update [ship_name]: update everything or a ship named `ship_name`");
    }
}

fn port(mut args: Vec<String>){
    //println!("PORT PAGE");
    if args.clone().len() > 0 {
        let command:String = String::from(args.clone()[0].as_ref());
        args.remove(0);
        match command.as_ref(){
            "list" => port_list(),
            "add" => port_add(args),
            "remove" => port_remove(args),
            _ => println!("Not a valid command"),
        }
    }
}

fn port_add(args: Vec<String>) {
    if args.clone().len() > 1 {
        let command = &args[0];
        match command.as_ref(){
            "github" => port_add_type(0, &args[1]),
            "gitlab" => port_add_type(1, &args[1]),
            "other" => port_add_type(2, &args[1]),
            "file" => port_add_type(3, &args[1]),
            _ => println!("Not a valid type"),
        }
    }
}

fn port_add_type(t: u8, path: &str) {
    let ports_toml:toml::TOML = toml::parse_file("~/.pinst/ports.toml".to_string());
    let mut gh_p:Vec<toml::TOMLValue> = ports_toml.clone().get_property("github".to_string()).expect("ports.toml is wrong").get_array().expect("Vec expected");
    let mut gl_p:Vec<toml::TOMLValue> = ports_toml.clone().get_property("gitlab".to_string()).expect("ports.toml is wrong").get_array().expect("Vec expected");
    let mut other_p:Vec<toml::TOMLValue> = ports_toml.clone().get_property("other".to_string()).expect("ports.toml is wrong").get_array().expect("Vec expected");
    let mut files_p:Vec<toml::TOMLValue> = ports_toml.clone().get_property("files".to_string()).expect("ports.toml is wrong").get_array().expect("Vec expected");
    if t == 0 {
        gh_p.push(toml::TOMLValue::get_new_string(path.to_string()));
    }else if t == 1 {
        gl_p.push(toml::TOMLValue::get_new_string(path.to_string()));
    }else if t == 2 {
        other_p.push(toml::TOMLValue::get_new_string(path.to_string()));
    }else if t == 3 {
        files_p.push(toml::TOMLValue::get_new_string(path.to_string()));
    }
    let mut new_toml:toml::TOML = toml::TOML::new_w_explicit_name("TOML");
    new_toml = new_toml.add_property("github".to_string(), toml::TOMLValue::get_new_array(gh_p));
    new_toml = new_toml.add_property("gitlab".to_string(), toml::TOMLValue::get_new_array(gl_p));
    new_toml = new_toml.add_property("other".to_string(), toml::TOMLValue::get_new_array(other_p));
    new_toml = new_toml.add_property("files".to_string(), toml::TOMLValue::get_new_array(files_p));
    io::overwrite("~/.pinst/ports.toml".to_string(), new_toml.to_string());
    println!("Port {} {}", path.green().bold(), "added!".green().bold());
}

fn port_remove(args: Vec<String>) {
    if args.clone().len() > 1 {
        let command = &args[0];
        match command.as_ref(){
            "github" => port_remove_type(0, &args[1]),
            "gitlab" => port_remove_type(1, &args[1]),
            "other" => port_remove_type(2, &args[1]),
            "file" => port_remove_type(3, &args[1]),
            _ => println!("Not a valid type"),
        }
    }
}

fn port_remove_type(t: u8, path: &str) {
    let ports_toml:toml::TOML = toml::parse_file("~/.pinst/ports.toml".to_string());
    let mut gh_p:Vec<toml::TOMLValue> = ports_toml.clone().get_property("github".to_string()).expect("ports.toml is wrong").get_array().expect("Vec expected");
    let mut gl_p:Vec<toml::TOMLValue> = ports_toml.clone().get_property("gitlab".to_string()).expect("ports.toml is wrong").get_array().expect("Vec expected");
    let mut other_p:Vec<toml::TOMLValue> = ports_toml.clone().get_property("other".to_string()).expect("ports.toml is wrong").get_array().expect("Vec expected");
    let mut files_p:Vec<toml::TOMLValue> = ports_toml.clone().get_property("files".to_string()).expect("ports.toml is wrong").get_array().expect("Vec expected");
    if t == 0 {
        let mut initialized=false;
        let mut i:usize = 0;
        for port in gh_p.clone() {
            if port == toml::TOMLValue::get_new_string(path.to_string()) {
                initialized=true;
                break;
            }
            i+=1;
        }
        if initialized {
            gh_p.remove(i);
        }
    }else if t == 1 {
        let mut initialized=false;
        let mut i:usize = 0;
        for port in gl_p.clone() {
            if port == toml::TOMLValue::get_new_string(path.to_string()) {
                initialized=true;
                break;
            }
            i+=1;
        }
        if initialized {
            gl_p.remove(i);
        }
    }else if t == 2 {
        let mut initialized=false;
        let mut i:usize = 0;
        for port in other_p.clone() {
            if port == toml::TOMLValue::get_new_string(path.to_string()) {
                initialized=true;
                break;
            }
            i+=1;
        }
        if initialized {
            other_p.remove(i);
        }
    }else if t == 3 {
        let mut initialized=false;
        let mut i:usize = 0;
        for port in files_p.clone() {
            if port == toml::TOMLValue::get_new_string(path.to_string()) {
                initialized=true;
                break;
            }
            i+=1;
        }
        if initialized {
            files_p.remove(i);
        }
    }
    let mut new_toml:toml::TOML = toml::TOML::new_w_explicit_name("TOML");
    new_toml = new_toml.add_property("github".to_string(), toml::TOMLValue::get_new_array(gh_p));
    new_toml = new_toml.add_property("gitlab".to_string(), toml::TOMLValue::get_new_array(gl_p));
    new_toml = new_toml.add_property("other".to_string(), toml::TOMLValue::get_new_array(other_p));
    new_toml = new_toml.add_property("files".to_string(), toml::TOMLValue::get_new_array(files_p));
    io::overwrite("~/.pinst/ports.toml".to_string(), new_toml.to_string());
    println!("Port {} {}", path.red().bold(), "removed!".red().bold());
}

fn port_list() {
    let ports_toml = toml::parse_file("~/.pinst/ports.toml".to_string());

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
    let ports_toml = toml::parse_file("~/.pinst/ports.toml".to_string());
    let installed_ships = ships::get_installed_ships();

    let github_ports = ports_toml.clone().get_property("github".to_string()).expect("Ports file error")
                                                               .get_array().expect("Port array error");
    if github_ports.len() > 0 {
        println!("Github ports: ");
    }
    for port in github_ports {
        let port_path = port.get_string().expect("Port path error");
        println!(" ▶︎ {}: ", port_path);
        for ship in ports::get_available_ship_names(port_path, 0){
            if installed_ships.contains(&ship) {
                if ships::is_ship_updatable(ship.clone()) {
                    println!("   {} {}", "⤓".yellow(), ship);
                } else if ships::is_ship_installer(ship.clone()){
                    println!("   {} {}", "▼".green(), ship);
                } else {
                    println!("   {} {}", "✓".green(), ship);
                }
            }else{
                if ships::is_ship_installer(ship.clone()){
                    println!("   {} {}", "▼".cyan(), ship);
                } else {
                    println!("   - {}", ship);
                }
            }
        }
    }

    let gitlab_ports = ports_toml.clone().get_property("gitlab".to_string()).expect("Ports file error")
                                                               .get_array().expect("Port array error");
    if gitlab_ports.len() > 0 {
        println!("Gitlab ports: ");
    }
    for port in gitlab_ports {
        let port_path = port.get_string().expect("Port path error");
        println!(" ▶︎ {}: ", port_path);
        for ship in ports::get_available_ship_names(port_path, 1){
            if installed_ships.contains(&ship) {
                if ships::is_ship_updatable(ship.clone()) {
                    println!("   {} {}", "⤓".yellow(), ship);
                } else if ships::is_ship_installer(ship.clone()){
                    println!("   {} {}", "▼".green(), ship);
                } else {
                    println!("   {} {}", "✓".green(), ship);
                }
            }else{
                if ships::is_ship_installer(ship.clone()){
                    println!("   {} {}", "▼".cyan(), ship);
                } else {
                    println!("   - {}", ship);
                }
            }
        }
    }

    let other_ports = ports_toml.clone().get_property("other".to_string()).expect("Ports file error")
                                                               .get_array().expect("Port array error");
    if other_ports.len() > 0 {
        println!("Other ports: ");
    }
    for port in other_ports {
        let port_path = port.get_string().expect("Port path error");
        println!(" ▶︎ {}: ", port_path);
        for ship in ports::get_available_ship_names(port_path, 2){
            if installed_ships.contains(&ship) {
                if ships::is_ship_updatable(ship.clone()) {
                    println!("   {} {}", "⤓".yellow(), ship);
                } else if ships::is_ship_installer(ship.clone()){
                    println!("   {} {}", "▼".green(), ship);
                } else {
                    println!("   {} {}", "✓".green(), ship);
                }
            }else{
                if ships::is_ship_installer(ship.clone()){
                    println!("   {} {}", "▼".cyan(), ship);
                } else {
                    println!("   - {}", ship);
                }
            }
        }
    }

    let file_ports = ports_toml.clone().get_property("files".to_string()).expect("Ports file error")
                                                            .get_array().expect("Port array error");
    if file_ports.len() > 0 {
        println!("File ports: ");
    }
    for port in file_ports {
        let port_path = port.get_string().expect("Port path error");
        println!(" ▶︎ {}: ", port_path);
        for ship in ports::get_available_ship_names(port_path, 3){
            if installed_ships.contains(&ship) {
                if ships::is_ship_updatable(ship.clone()) {
                    println!("   {} {}", "⤓".yellow(), ship);
                } else if ships::is_ship_installer(ship.clone()){
                    println!("   {} {}", "▼".green(), ship);
                } else {
                    println!("   {} {}", "✓".green(), ship);
                }
            }else{
                if ships::is_ship_installer(ship.clone()){
                    println!("   {} {}", "▼".cyan(), ship);
                } else {
                    println!("   - {}", ship);
                }
            }
        }
    }
}

fn install(args: Vec<String>){
    if args.clone().len() > 0 {
        let term = &args[0];
        let ship = ports::find_ship(term.to_string()).expect("Ship not found");
        ship.install();
    }
}

fn uninstall(args: Vec<String>){
    if args.clone().len() > 0 {
        let term = args[0].to_string();
        println!("Uninstalling {}", term);
        if io::path_exists("~/.pinst/".to_string()+&term) {
            io::remove_file("~/.pinst/".to_string()+&term);
            println!("Removed {}", "~/.pinst/".to_string()+&term);
        }
        let ships_toml = toml::parse_file("~/.pinst/ships.toml".to_string());
        let mut version = String::new();
        for object in ships_toml.get_objects(){
            if object.name == term {
                version = object.get_property("version".to_string()).expect("Version expected").get_string().expect("Version expected to be an string");
            }
        }
        if version != String::new() {
            let mut ships_str = io::read("~/.pinst/ships.toml".to_string());
            
            let ship_toml_str = "[".to_string()+&term+"]\nversion = \""+&version+"\"\n\n";
            ships_str = ships_str.replace(&ship_toml_str, "");
            
            io::overwrite("~/.pinst/ships.toml".to_string(), ships_str);
            println!("Uninstalled {} {}", term.red(), version.red());
        }else{
            println!("Can't uninstall {} as it isn't installed", term);
        }
    }
}

//#[allow(unused_must_use)]
fn update_comm(args: Vec<String>) {
    update(args).unwrap_or_default();
}

fn update(args: Vec<String>) -> Result<(),()>{
    if args.clone().len() > 0 {
        let term = args[0].clone().to_string();
        
        if ships::is_ship_updatable(term.clone()) {
            println!("Updating {}", &term);
            uninstall(args.clone());
            install(args.clone());
            println!("Updated {}", term.green().bold());
            Ok(())
        }else{
            println!("{} not updatable", term.clone());
            Err(())
        }
    }else{
        let mut idx:u8 = 0;
        
        for ship in ships::get_installed_ships() {
            let mut nargs:Vec<String> = Vec::new();
            nargs.push(ship);
            let r = update(nargs);
            if r.is_ok(){
                idx+=1;
            }
            
        }
        if idx == 0 {
            println!("Nothing to update");
            
        }
        Ok(())
    }
}
