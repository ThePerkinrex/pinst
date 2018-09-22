//use std::fs;
//use std::path::Path;
use toml;
use ports;
use io;
use colored::*;

#[derive(Debug, Clone)]
pub struct Ship {
    pub null: bool,
    pub dependencies: Vec<String>,
    pub content: Vec<String>,
    pub makefile: String,
    pub makefile_url: String,
    pub download_type: u8,
    pub port_name: String,
    pub name: String,
}

// Methods
#[allow(dead_code)]
impl Ship {
    fn download(self, location: String) -> Result<(),&'static str>{
        //println!("Makefile: {}({})", &self.makefile, &self.makefile_url);
        //println!("Location: {}({})", &location,  location.clone()+ &self.makefile);
        if self.download_type == 0 { // Github
            let mut url: String = String::from("https://raw.githubusercontent.com/");
            url += &self.port_name;
            url += "/master/";
            url += &self.makefile_url;
            io::download_file(url, location.clone() + &self.makefile);
        }

        if self.download_type == 1 { // Gitlab
            let mut url: String = String::from("https://gitlab.com/");
            url += &self.port_name;
            url += "/raw/master/";
            url += &self.makefile_url;
            io::download_file(url, location.clone() + &self.makefile);
        }

        if self.download_type == 2 { // other
            io::download_file(self.makefile_url.clone(), location.clone() + &self.makefile);
        }

        if self.download_type == 3 { // File
            io::copy(self.makefile_url.clone(), location.clone() + &self.makefile);
        }
        for c in self.content {
            let url_tmp: &str = c.as_ref();
            let split_url: Vec<&str> = url_tmp.split('/').collect();
            let filename_correct = split_url[split_url.len()-1].clone();
            io::download_file(c.clone(), location.clone() + filename_correct);
        }
        Ok(())
    }

    pub fn download_default(self) -> Result<(),&'static str>{
        self.download("~/.pinst/".to_string())
    }

    pub fn install(self) {
        println!("{}", "Installing dependecies".yellow().bold());
        for dependency in self.clone().dependencies {
            let dship = ports::find_ship(dependency).expect(&"Dependecy information is wrong".red());
            dship.install();
        }
        println!("Starting downloads for {}", self.name.green().bold());
        self.clone().download_default().expect("A download error occured");
        println!("Installing {}", self.name.green().bold());
        io::run_command("make -f ~/.pinst/".to_string() + &self.makefile + " install", false);
        println!("Cleaning up makefile");
        io::run_command("rm ~/.pinst/".to_string() + &self.makefile, false);
        println!("\n{} installed!", self.name.green().bold());
    }
}

// Static functions & constructors
#[allow(dead_code)]
impl Ship {
    pub fn null() -> Ship {
        return Ship {null: true, dependencies: Vec::new(), content: Vec::new(), makefile: String::new(), makefile_url: String::new(), download_type: 0, port_name: String::new(), name: String::new()};
    }

    pub fn new_from_toml(ship: toml::TOML, port_type: u8, port_name: String) -> Ship{
        #[allow(non_snake_case)]
        let ship_content_TOMLValue = ship.clone().get_property(String::from("content")).expect("Content property not found").get_array().expect("Expected content to be an array");
        let mut ship_content: Vec<String> = Vec::new();
        for toml_value in ship_content_TOMLValue {
            ship_content.push(toml_value.get_string().expect("String expected"));
        }
        let ship_dependencies_TOMLValue = ship.clone().get_property(String::from("dependencies")).expect("dependencies property not found").get_array().expect("Expected dependencies to be an array");
        let mut ship_dependencies: Vec<String> = Vec::new();
        for toml_value in ship_dependencies_TOMLValue {
            ship_dependencies.push(toml_value.get_string().expect("String expected"));
        }
        let ship_makefile: String;
        let makefile_toml:toml::TOMLValue = ship.clone().get_property(String::from("makefile")).expect("Makefile property not found");
        let ship_makefile_url: String = makefile_toml.get_string().expect("Expected makefile property to be an string");
        if port_type > 1 {
            let url_tmp: &str = ship_makefile_url.as_ref();
            let split_url: Vec<&str> = url_tmp.split('/').collect();
            println!("Split: {:?}", url_tmp.split('/'));
            ship_makefile = split_url[split_url.len()-1].clone().to_string();

        }else{
            let url_tmp: &str = ship_makefile_url.as_ref();
            let split_url: Vec<&str> = url_tmp.split('/').collect();
            ship_makefile = split_url[split_url.len()-1].clone().to_string();
        }
        let result: Ship = Ship {dependencies: ship_dependencies, content: ship_content, makefile_url: ship_makefile_url, makefile: ship_makefile, null: false, download_type: port_type, port_name: port_name, name: ship.name};
        return result.clone();
    }

    pub fn new_from_name(ship_name: String, port_name:String, port_type:u8) -> Ship {
        let mut result:toml::TOML = toml::TOML::null();

        if port_type == 0 { // Github
            let mut url: String = String::from("https://raw.githubusercontent.com/");
            url += &port_name;
            url += "/master/port.toml";
            result = toml::parse(io::read_from_url(url));
        }

        if port_type == 1 { // Gitlab
            let mut url: String = String::from("https://gitlab.com/");
            url += &port_name;
            url += "/raw/master/port.toml";
            result = toml::parse(io::read_from_url(url));
        }

        if port_type == 2 { // other
            result = toml::parse(io::read_from_url(port_name.clone()));
        }

        if port_type == 3 { // File
            result = toml::parse_file(port_name.clone());
        }

        return Ship::new_from_toml(result.get_object(ship_name.clone()).expect("Ship TOML error"), port_type, port_name);
    }
}
