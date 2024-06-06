use std::collections::BTreeMap;
use serde_json::*;
use strum_macros::EnumString;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, EnumString, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum ParametersType {
    i32(i32),
    u32(u32),
    VecI32(Vec<i32>),
    VecU32(Vec<u32>),
    usize(usize),
    String(String),
}

pub fn parameters_json_to_map(json_input: &String) -> BTreeMap<String, ParametersType> {
    let json_value: Value = from_str(&json_input).unwrap();
    let mut map_result: BTreeMap<String, ParametersType> = BTreeMap::new();
    
    if let Some(actor_desc) = json_value.get("actor_desc") {
        if let Some(actor_desc_map) = actor_desc.as_object() {
            for (key, value) in actor_desc_map {
                let param = match value {
                    Value::Number(num) if num.is_u64() => ParametersType::u32(num.as_u64().unwrap() as u32),
                    Value::Number(num) if num.is_i64() => ParametersType::i32(num.as_i64().unwrap() as i32),
                    Value::Array(arr) if arr.iter().all(|x| x.is_u64()) => ParametersType::VecU32(arr.iter().map(|x| x.as_u64().unwrap() as u32).collect()),
                    Value::Array(arr) if arr.iter().all(|x| x.is_i64()) => ParametersType::VecI32(arr.iter().map(|x| x.as_i64().unwrap() as i32).collect()),
                    Value::String(s) => ParametersType::String(s.clone()),
                    _ => continue,
                };
                map_result.insert(key.clone(), param);
            }
        }
    }
    
    map_result
}

pub fn arguments_json_to_map(json_input: &String) -> BTreeMap<String, ParametersType> {
    let json_value: Value = from_str(&json_input).unwrap();
    let mut map_result: BTreeMap<String, ParametersType> = BTreeMap::new();
    
    if let Some(actor_desc) = json_value.get("function_parameters") {
        if let Some(actor_desc_map) = actor_desc.as_object() {
            for (key, value) in actor_desc_map {
                let param = match value {
                    Value::Number(num) if num.is_u64() => ParametersType::u32(num.as_u64().unwrap() as u32),
                    Value::Number(num) if num.is_i64() => ParametersType::i32(num.as_i64().unwrap() as i32),
                    Value::Array(arr) if arr.iter().all(|x| x.is_u64()) => ParametersType::VecU32(arr.iter().map(|x| x.as_u64().unwrap() as u32).collect()),
                    Value::Array(arr) if arr.iter().all(|x| x.is_i64()) => ParametersType::VecI32(arr.iter().map(|x| x.as_i64().unwrap() as i32).collect()),
                    Value::String(s) => ParametersType::String(s.clone()),
                    _ => continue,
                };
                map_result.insert(key.clone(), param);
            }
        }
    }
    
    map_result
}

pub fn parameters_map_to_json(map_input: BTreeMap<String, ParametersType>) -> String {
    let mut json_map = serde_json::Map::new();

    for (key, value) in map_input {
        let json_value = match value {
            ParametersType::i32(v) => Value::Number(v.into()),
            ParametersType::u32(v) => Value::Number(v.into()),
            ParametersType::VecI32(v) => Value::Array(v.into_iter().map(Value::from).collect()),
            ParametersType::VecU32(v) => Value::Array(v.into_iter().map(Value::from).collect()),
            ParametersType::usize(v) => Value::Number((v as u64).into()),
            ParametersType::String(v) => Value::String(v),
        };
        json_map.insert(key, json_value);
    }
    let desc = to_string(&Value::Object(json_map)).unwrap();

    format!("{{\"actor_desc\" : {}}}", desc).to_string()
}

pub fn message_to_user(message : String) -> String{
    let message_json =  format!("{{\"to_user_message\" : \"{}\"}}", message);

    message_json
}

pub fn add_message_to_user(message : String , json_to_follow : String) -> String{
    let message_json =  format!("{{{}, \"to_user_message\": \"{}\"}}", json_to_follow, message);

    message_json
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let json_input: &str = r#"{
            "actor_desc": {
                "input_or_output": 1,
                "mode": 1,
                "pin_number": 0
            },
            "function_parameters" : {
                "number" : 1
            }
        }"#;
        let parameter_map: BTreeMap<String, ParametersType> = parameters_json_to_map(&json_input.to_string());
        println!("{:?}", parameter_map);
        
        let argument_map: BTreeMap<String, ParametersType> = arguments_json_to_map(&json_input.to_string());
        println!("{:?}", argument_map);      

        println!("{:?}", argument_map.get("number"));
    }
}
