
use toml;
use io;
use ships::Ship;

/*    PORT TYPES
    0 -> Github
    1 -> Gitlab
    2 -> Other
    3 -> File
*/

pub fn get_available_ship_names(port_name:String, port_type:u8) -> Vec<String> {
    let mut result:Vec<String> = Vec::new();

    if port_type == 0 { // Github
        let mut url: String = String::from("https://raw.githubusercontent.com/");
        url += &port_name;
        url += "/master/port.toml";
        let port_toml = toml::parse(io::read_from_url(url));
        for ship in port_toml.get_objects(){
            result.push(ship.name);
        }
    }

    if port_type == 1 { // Gitlab
        let mut url: String = String::from("https://gitlab.com/");
        url += &port_name;
        url += "/raw/master/port.toml";
        let port_toml = toml::parse(io::read_from_url(url));
        for ship in port_toml.get_objects(){
            result.push(ship.name);
        }
    }

    if port_type == 2 { // other
        let port_toml = toml::parse(io::read_from_url(port_name.clone()));
        for ship in port_toml.get_objects(){
            result.push(ship.name);
        }
    }

    if port_type == 3 { // File
        let port_toml = toml::parse_file(port_name);
        for ship in port_toml.get_objects(){
            result.push(ship.name);
        }
    }

    return result;
}

pub fn get_available_ships(port_name:String, port_type:u8) -> Vec<Ship> {
    let mut result:Vec<Ship> = Vec::new();

    if port_type == 0 { // Github
        let mut url: String = String::from("https://raw.githubusercontent.com/");
        url += &port_name;
        url += "/master/port.toml";
        let port_toml = toml::parse(io::read_from_url(url));
        for ship in port_toml.get_objects(){
            result.push(Ship::new_from_toml(ship, port_type, port_name.clone()));
        }
    }

    if port_type == 1 { // Gitlab
        let mut url: String = String::from("https://gitlab.com/");
        url += &port_name;
        url += "/raw/master/port.toml";
        let port_toml = toml::parse(io::read_from_url(url));
        for ship in port_toml.get_objects(){
            result.push(Ship::new_from_toml(ship, port_type, port_name.clone()));
        }
    }

    if port_type == 2 { // other
        let port_toml = toml::parse(io::read_from_url(port_name.clone()));
        for ship in port_toml.get_objects(){
            result.push(Ship::new_from_toml(ship, port_type, port_name.clone()));
        }
    }

    if port_type == 3 { // File
        let port_toml = toml::parse_file(port_name.clone());
        for ship in port_toml.get_objects(){
            result.push(Ship::new_from_toml(ship, port_type, port_name.clone()));
        }
    }

    return result;
}

pub fn find_ship(name: String) -> Option<Ship> {
    let ports_toml = toml::parse_file("~/.pinst/ports.toml".to_string());

    let github_ports = ports_toml.clone().get_property("github".to_string()).expect("Ports file error")
                                                               .get_array().expect("Port array error");

    for port in github_ports {
        let port_path = port.get_string().expect("Port path error");

        for ship in get_available_ships(port_path, 0){
            if ship.name == name {
                return Option::Some(ship);
            }
        }
    }

    let gitlab_ports = ports_toml.clone().get_property("gitlab".to_string()).expect("Ports file error")
                                                               .get_array().expect("Port array error");

    for port in gitlab_ports {
        let port_path = port.get_string().expect("Port path error");
        for ship in get_available_ships(port_path, 1){
            if ship.name == name {
                return Option::Some(ship);
            }
        }
    }

    let other_ports = ports_toml.clone().get_property("other".to_string()).expect("Ports file error")
                                                               .get_array().expect("Port array error");

    for port in other_ports {
        let port_path = port.get_string().expect("Port path error");
        for ship in get_available_ships(port_path, 2){
            if ship.name == name {
                return Option::Some(ship);
            }
        }
    }

    let file_ports = ports_toml.clone().get_property("files".to_string()).expect("Ports file error")
                                                            .get_array().expect("Port array error");
    
    for port in file_ports {
        let port_path = port.get_string().expect("Port path error");
        for ship in get_available_ships(port_path, 3){
            if ship.name == name {
                return Option::Some(ship);
            }
        }
    }

    return Option::None;
}
