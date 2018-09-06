
use toml;
use io;

/*    PORT TYPES
    0 -> Github
    1 -> Gitlab
    2 -> Other
    3 -> File
*/

pub fn get_available_ships(port_name:String, port_type:u8) -> Vec<String> {
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
