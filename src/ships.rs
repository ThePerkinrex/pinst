
use toml;
use io;

#[derive(Debug, Clone)]
pub struct Ship {
    pub null: bool,
    pub content: Vec<String>,
    pub makefile: String,
    pub download_type: u8,
}

// Methods
impl Ship {

}

// Static functions & constructors
impl Ship {
    pub fn null() -> Ship {
        return Ship {null: true, content: Vec::new(), makefile: String::new(), download_type: 0};
    }

    pub fn new_from_toml(ship: toml::TOML, port_type: u8) -> Ship{
        let ship_content_TOMLValue = ship.clone().get_property(String::from("content")).expect("Content property not found").get_array().expect("Expected content to be an array");
        let mut ship_content: Vec<String> = Vec::new();
        for toml_value in ship_content_TOMLValue {
            ship_content.push(toml_value.get_string().expect("String expected"));
        }
        let ship_makefile = ship.get_property(String::from("makefile")).expect("Makefile property not found").get_string().expect("Expected makefile property to be an string");
        ship_content.push(ship_makefile.clone());
        return Ship {content: ship_content, makefile: ship_makefile, null: false, download_type: port_type};
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
            result = toml::parse_file(port_name);
        }

        return Ship::new_from_toml(result.get_object(ship_name.clone()).expect("Ship TOML error"), port_type);
    }
}
