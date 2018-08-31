// toml parser mod

use std::ptr;
use std::str::FromStr;


#[derive(Debug, Clone)]
pub struct TOMLValue {
    array: Option<Vec<TOMLValue>>,
    string: Option<String>,
    number: Option<f64>,
    empty: bool
}
impl TOMLValue {
    pub fn to_string(self) -> String {
        let mut r:String = String::new();
        if self.empty {
            return String::from("Empty");
        }
        if self.string.is_some() {
            r += "\"";
            r += &self.string.unwrap();
            r += "\"";
        }else if self.number.is_some() {
            return self.number.unwrap().to_string();
        }else if self.array.is_some() {
            r += "[";
            let mut i:usize = 0;
            for value in self.array.clone().unwrap() {
                r += &value.to_string();
                if self.array.clone().unwrap().len()-1 != i{
                    r += ",";
                }
                i+=1;
            }
            r += "]";
        }

        return r;
    }


    pub fn new() -> TOMLValue{
        return TOMLValue {array: Option::None, string: Option::None, number: Option::None, empty: true};
    }

    pub fn get_array(array: Vec<TOMLValue>) -> TOMLValue{
        return TOMLValue {array: Option::Some(array), string: Option::None, number: Option::None, empty: false};
    }

    pub fn get_string(string: String) -> TOMLValue{
        return TOMLValue {array: Option::None, string: Option::Some(string), number: Option::None, empty: false};
    }

    pub fn get_number(number: f64) -> TOMLValue{
        return TOMLValue {array: Option::None, string: Option::None, number: Option::Some(number), empty: false};
    }
}

#[derive(Debug, Clone)]
pub struct TOML {
    name: String,
    objects: Vec<TOML>,
    properties: Vec<(String, TOMLValue)>,
    null: bool
}

impl TOML {
    pub fn get_property(self, name: String) -> Option<TOMLValue>{
        for property in self.properties {
             if name == property.0 && !self.null {
                 return Option::Some(property.1.clone());
             }
        }
        return Option::None;
    }

    pub fn get_object(self, name: String) -> Option<TOML>{
        for object in self.objects {
            if object.name == name && !self.null{
                return Option::Some(object.clone());
            }
        }
        return Option::None;
    }

    pub fn is_null(self) -> bool {
        return self.null;
    }

    pub fn to_string(self) -> String {

        let mut r:String = String::new();
        if self.name != String::from("TOML") {
            r += "[";
            r += &self.name;
            r += "]";
            r += "\n";
        }
        for p in self.properties {
            r += &p.0;
            r += " = ";
            r += &p.1.to_string();
            r += "\n";
        }
        for obj in self.objects {
            r += &obj.to_string();
            r += "\n";
        }
        return r;
    }

    pub fn new_w_name(name: String) -> TOML {
        return TOML {name: name, objects: Vec::new(), properties: Vec::new(), null: false};
    }

    pub fn new_w_explicit_name(name: &str) -> TOML {
        return TOML::new_w_name(String::from(name));
    }

    pub fn new_empty() -> TOML {
        return TOML::new_w_explicit_name("");
    }

    pub fn null() -> TOML {
        return TOML {name: String::new(), objects: Vec::new(), properties: Vec::new(), null: true};
    }
}

fn parse_value(tok: String) -> TOMLValue {
    let mut toml_value = TOMLValue::new();
    let tok_bytes:&[u8] = tok.as_bytes();
    let mut value_type: u8 = 0;
    let mut cur_tok: String = String::new();
    let mut cur_arr:Vec<TOMLValue> = Vec::new();
    /*
    0 -> Unknown
    1 -> String
    2 -> Number
    3 -> Array
    */
    let mut pchr:*mut char = ptr::null_mut();
    for chr_ptr in tok_bytes {
        let mut chr:char = *chr_ptr as char;
        if value_type == 0 {
            if chr == '"' {
                value_type = 1;

            }else if chr.is_digit(10) {
                value_type = 2;

                let mut utf8str: Vec<u8> = Vec::new();
                utf8str.push(chr as u8);
                cur_tok += &String::from_utf8(utf8str).unwrap();
            }else if chr == '[' {
                value_type = 3;
            }
        }else if value_type == 1 {
            if chr == '"' && pchr == &mut '\\'{
                let mut utf8str: Vec<u8> = Vec::new();
                utf8str.push(chr as u8);
                cur_tok += &String::from_utf8(utf8str).unwrap();
            }else if chr == '"' {
                toml_value = TOMLValue::get_string(cur_tok.clone());
                cur_tok = String::new();
            }else{
                let mut utf8str: Vec<u8> = Vec::new();
                utf8str.push(chr as u8);
                cur_tok += &String::from_utf8(utf8str).unwrap();

            }
        }else if value_type == 2 {

            if chr.is_digit(10) || chr == '.' {
                let mut utf8str: Vec<u8> = Vec::new();
                utf8str.push(chr as u8);
                cur_tok += &String::from_utf8(utf8str).unwrap();

            }
        }else if value_type == 3 {
            if chr == ',' {
                cur_arr.push(parse_value(cur_tok.clone()));
                cur_tok = String::new();
            } else if chr == ']' {
                cur_arr.push(parse_value(cur_tok.clone()));
                cur_tok = String::new();
                toml_value = TOMLValue::get_array(cur_arr.clone());
            } else {
                let mut utf8str: Vec<u8> = Vec::new();
                utf8str.push(chr as u8);
                cur_tok += &String::from_utf8(utf8str).unwrap();
            }
        }


        pchr = &mut chr;
    }
    if value_type == 2 {
        let n:f64 = f64::from_str(&cur_tok).unwrap();
        toml_value = TOMLValue::get_number(n);
    }
    if toml_value.empty {
        panic!("TOML parsing error, value expected, nothing found\nstring that created the panic: {}\n", tok);
    }
    return toml_value;
}

pub fn parse(toml: String) -> TOML{
    let mut cur_toml:TOML = TOML::new_w_explicit_name("TOML");
    let toml_str: &str = toml.as_ref();
    let mut parsing_object:TOML = TOML::null();
    let mut parse_stage: u8 = 0;
    /*
    0 -> None
    1 -> Object name
    2 -> Object properties
    3 -> Properties
    */
    for line in toml_str.split("\n") {


        let mut line_string:String = String::from(line);
        let line_chars:&[u8] = line_string.as_bytes();
        let mut cur_tok:String = String::new();
        let mut cur_p_name:String = String::new();
        let mut properties_stage:u8 = 0;
        /*
        0 -> Nop
        1 -> Name
        2 -> Value (parseValue())
        */
        if parse_stage == 2 {
            if String::from(line).eq(&String::from("")) {
                cur_toml.objects.push(parsing_object.clone());
                parsing_object = TOML::null();
                parse_stage = 0;
            }
        }
        for chr_u8 in line_chars {
            let mut chr: char = *chr_u8 as char;
            if parse_stage == 0 {
                if chr == '[' {
                    parse_stage = 1;
                } else if chr.is_alphabetic() {
                    parse_stage = 3;
                    let mut utf8str: Vec<u8> = Vec::new();
                    utf8str.push(chr as u8);
                    cur_tok += &String::from_utf8(utf8str).unwrap();
                }
            }else if parse_stage == 1 {
                if chr == ']' {
                    parsing_object = TOML::new_w_name(cur_tok.clone());
                    //println!("pobj name: {}", cur_tok);
                    cur_tok = String::new();
                    parse_stage = 2;
                }else{
                    let mut utf8str: Vec<u8> = Vec::new();
                    utf8str.push(chr as u8);
                    cur_tok += &String::from_utf8(utf8str).unwrap();
                }
            }else if parse_stage == 2 {
                if properties_stage == 0 && chr.is_alphabetic() && chr != ' ' {
                    properties_stage = 1;
                    let mut utf8str: Vec<u8> = Vec::new();
                    utf8str.push(chr as u8);
                    cur_tok += &String::from_utf8(utf8str).unwrap();
                }else if properties_stage == 1 && chr == '=' {
                    cur_p_name = cur_tok.clone();
                    cur_tok = String::new();
                    properties_stage = 2;
                }else if properties_stage == 1 && chr.is_alphabetic() && chr != ' ' {
                    let mut utf8str: Vec<u8> = Vec::new();
                    utf8str.push(chr as u8);
                    cur_tok += &String::from_utf8(utf8str).unwrap();
                }else if properties_stage == 2 {
                    let mut utf8str: Vec<u8> = Vec::new();
                    utf8str.push(chr as u8);
                    cur_tok += &String::from_utf8(utf8str).unwrap();
                }
            } else if parse_stage == 3 {
                if properties_stage == 0 && chr.is_alphabetic() && chr != ' ' {
                    properties_stage = 1;
                    let mut utf8str: Vec<u8> = Vec::new();
                    utf8str.push(chr as u8);
                    cur_tok += &String::from_utf8(utf8str).unwrap();
                }else if properties_stage == 1 && chr == '=' {
                    cur_p_name = cur_tok.clone();
                    cur_tok = String::new();
                    properties_stage = 2;
                }else if properties_stage == 1 && chr.is_alphabetic() && chr != ' ' {
                    let mut utf8str: Vec<u8> = Vec::new();
                    utf8str.push(chr as u8);
                    cur_tok += &String::from_utf8(utf8str).unwrap();
                }else if properties_stage == 2 {
                    let mut utf8str: Vec<u8> = Vec::new();
                    utf8str.push(chr as u8);
                    cur_tok += &String::from_utf8(utf8str).unwrap();
                }
            }
        }
        if parse_stage == 2 && properties_stage == 2 {
            parsing_object.properties.push((cur_p_name, parse_value(cur_tok)));
            cur_p_name = String::new();
            cur_tok = String::new();
            properties_stage = 0;
        }
        if parse_stage == 3 && properties_stage == 2 {
            cur_toml.properties.push((cur_p_name, parse_value(cur_tok)));
            parse_stage = 0;
        }
    }
    if parse_stage == 2 {

        cur_toml.objects.push(parsing_object.clone());
    }

    return cur_toml;
}
