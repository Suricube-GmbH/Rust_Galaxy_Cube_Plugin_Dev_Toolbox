use serde::{Deserialize, Serialize};
use serde_json::*;
use std::collections::BTreeMap;
use strum_macros::EnumString;

#[derive(Clone, Debug, PartialEq, EnumString, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum ParametersType {
    bool(bool),
    i16(i16),
    i32(i32),
    i64(i64),
    u8(u8),
    u16(u16),
    u32(u32),   
    u64(u64),
    f32(f32),
    f64(f64),
    VecI32(Vec<i32>),
    VecU32(Vec<u32>),
    usize(usize),
    String(String),
}

pub fn get_actor_name(json_input: &String) -> String {
    let json_value: Value = from_str(&json_input).unwrap();
    if let Some(actor_name) = json_value.get("actor_name"){
        return actor_name.to_string()
    } else {
        return String::new()
    }
}

pub fn parameters_json_to_map(json_input: &String) -> BTreeMap<String, ParametersType> {
    let json_value: Value = from_str(&json_input).unwrap();
    let mut map_result: BTreeMap<String, ParametersType> = BTreeMap::new();

    if let Some(actor_desc) = json_value.get("parameters") {
        if let Some(actor_desc_map) = actor_desc.as_object() {
            for (key, value) in actor_desc_map {
                let param = match value {
                    Value::Bool(bool) => ParametersType::bool(*bool),
                    Value::Number(num) if num.is_u64() => ParametersType::u64(num.as_u64().unwrap()),
                    Value::Number(num) if num.is_i64() => ParametersType::i64(num.as_i64().unwrap()),
                    Value::Number(num) if num.is_f64() => ParametersType::f64(num.as_f64().unwrap()),
                    Value::Array(arr) if arr.iter().all(|x| x.is_u64()) => ParametersType::VecU32(
                        arr.iter().map(|x| x.as_u64().unwrap() as u32).collect(),
                    ),
                    Value::Array(arr) if arr.iter().all(|x| x.is_i64()) => ParametersType::VecI32(
                        arr.iter().map(|x| x.as_i64().unwrap() as i32).collect(),
                    ),
                    Value::String(s) => ParametersType::String(s.clone()),
                    _ => continue,
                };
                map_result.insert(key.clone(), param);
            }
        }
    }

    map_result
}

pub fn virtual_parameters_json_to_map(json_input: &String) -> BTreeMap<String, ParametersType> {
    let json_value: Value = from_str(&json_input).unwrap();
    let mut map_result: BTreeMap<String, ParametersType> = BTreeMap::new();

    if let Some(actor_desc) = json_value.get("virtual_parameters") {
        if let Some(actor_desc_map) = actor_desc.as_object() {
            for (key, value) in actor_desc_map {
                let param = match value {
                    Value::Bool(bool) => ParametersType::bool(*bool),
                    Value::Number(num) if num.is_u64() => ParametersType::u64(num.as_u64().unwrap()),
                    Value::Number(num) if num.is_i64() => ParametersType::i64(num.as_i64().unwrap()),
                    Value::Number(num) if num.is_f64() => ParametersType::f64(num.as_f64().unwrap()),
                    Value::Array(arr) if arr.iter().all(|x| x.is_u64()) => ParametersType::VecU32(
                        arr.iter().map(|x| x.as_u64().unwrap() as u32).collect(),
                    ),
                    Value::Array(arr) if arr.iter().all(|x| x.is_i64()) => ParametersType::VecI32(
                        arr.iter().map(|x| x.as_i64().unwrap() as i32).collect(),
                    ),
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
                    Value::Bool(bool) => ParametersType::bool(*bool),
                    Value::Number(num) if num.is_u64() => ParametersType::u64(num.as_u64().unwrap()),
                    Value::Number(num) if num.is_i64() => ParametersType::i64(num.as_i64().unwrap()),
                    Value::Number(num) if num.is_f64() => ParametersType::f64(num.as_f64().unwrap()),
                    Value::Array(arr) if arr.iter().all(|x| x.is_u64()) => ParametersType::VecU32(
                        arr.iter().map(|x| x.as_u64().unwrap() as u32).collect(),
                    ),
                    Value::Array(arr) if arr.iter().all(|x| x.is_i64()) => ParametersType::VecI32(
                        arr.iter().map(|x| x.as_i64().unwrap() as i32).collect(),
                    ),
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
            ParametersType::bool(v) => Value::Bool(v),
            ParametersType::i16(v) => Value::Number(v.into()),
            ParametersType::i32(v) => Value::Number(v.into()),
            ParametersType::i64(v) => Value::Number(v.into()),
            ParametersType::u8(v) => Value::Number(v.into()),
            ParametersType::u16(v) => Value::Number(v.into()),
            ParametersType::u32(v) => Value::Number(v.into()),
            ParametersType::u64(v) => Value::Number(v.into()),
            ParametersType::f32(v) => Value::Number(serde_json::Number::from_f64(v as f64).unwrap()),
            ParametersType::f64(v) => Value::Number(serde_json::Number::from_f64(v).unwrap()),
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

pub fn message_to_user(message: String) -> String {
    let message_json = format!("{{\"to_user_message\" : \"{}\"}}", message);

    message_json
}

pub fn add_message_to_user(message: String, json_to_follow: String) -> String {
    let mut json_value: Value = serde_json::from_str(&json_to_follow).unwrap();
    if let Value::Object(ref mut map) = json_value {
        map.insert("to_user_message".to_string(), Value::String(message));
    }
    to_string(&json_value).unwrap()
}

pub fn actor_function_request(
    actor_name: String,
    function_name: String,
    function_parameters_map: BTreeMap<String, ParametersType>,
) -> String {
    let function_parameters = {
        let mut json_map = serde_json::Map::new();
        for (key, value) in function_parameters_map {
            let json_value = match value {
                ParametersType::bool(v) => Value::Bool(v),
                ParametersType::i16(v) => Value::Number(v.into()),
                ParametersType::i32(v) => Value::Number(v.into()),
                ParametersType::i64(v) => Value::Number(v.into()),
                ParametersType::u8(v) => Value::Number(v.into()),
                ParametersType::u16(v) => Value::Number(v.into()),
                ParametersType::u32(v) => Value::Number(v.into()),
                ParametersType::u64(v) => Value::Number(v.into()),
                ParametersType::f32(v) => Value::Number(serde_json::Number::from_f64(v as f64).unwrap()),
                ParametersType::f64(v) => Value::Number(serde_json::Number::from_f64(v).unwrap()),
                ParametersType::VecI32(v) => Value::Array(v.into_iter().map(Value::from).collect()),
                ParametersType::VecU32(v) => Value::Array(v.into_iter().map(Value::from).collect()),
                ParametersType::usize(v) => Value::Number((v as u64).into()),
                ParametersType::String(v) => Value::String(v),
            };
            json_map.insert(key, json_value);
        }
        json_map
    };

    let function_component = json!({
        "function_component": {
            "actor_name": actor_name,
            "function_name": function_name,
            "function_parameters": function_parameters
        }
    });

    function_component.to_string()
}

pub fn add_actor_function_request(
    actor_name: String,
    function_name: String,
    function_parameters_map: BTreeMap<String, ParametersType>,
    json_to_follow: String,
) -> String {
    let mut json_value: Value = serde_json::from_str(&json_to_follow).unwrap();
    let function_parameters = {
        let mut json_map = serde_json::Map::new();
        for (key, value) in function_parameters_map {
            let json_value = match value {
                ParametersType::bool(v) => Value::Bool(v),
                ParametersType::i16(v) => Value::Number(v.into()),
                ParametersType::i32(v) => Value::Number(v.into()),
                ParametersType::i64(v) => Value::Number(v.into()),
                ParametersType::u8(v) => Value::Number(v.into()),
                ParametersType::u16(v) => Value::Number(v.into()),
                ParametersType::u32(v) => Value::Number(v.into()),
                ParametersType::u64(v) => Value::Number(v.into()),
                ParametersType::f32(v) => Value::Number(serde_json::Number::from_f64(v as f64).unwrap()),
                ParametersType::f64(v) => Value::Number(serde_json::Number::from_f64(v).unwrap()),
                ParametersType::VecI32(v) => Value::Array(v.into_iter().map(Value::from).collect()),
                ParametersType::VecU32(v) => Value::Array(v.into_iter().map(Value::from).collect()),
                ParametersType::usize(v) => Value::Number((v as u64).into()),
                ParametersType::String(v) => Value::String(v),
            };
            json_map.insert(key, json_value);
        }
        json_map
    };

    if let Value::Object(ref mut map) = json_value {
        map.insert("function_component".to_string(), json!({
            "actor_name": actor_name,
            "function_name": function_name,
            "function_parameters": function_parameters
        }));
    }
    to_string(&json_value).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_message() {
        // bad input for testing
        let actor_desc = "{\"actor_desc\" : {\"feur\" : 1,\"shrek\" : \"oui\"}}";

        let new_message = add_message_to_user("Hello world".to_string(), actor_desc.to_string());
        println!("{new_message}");

        let result_into_value: Value = serde_json::from_str(&new_message).unwrap();
        println!("{result_into_value}");
        println!("{}", to_string_pretty(&result_into_value).unwrap());
    }

    #[test]
    fn function_component() {
        let actor_desc = "{\"actor_desc\" : {\"feur\" : 1,\"shrek\" : \"oui\"}}".trim();

        let result_into_value: Value = serde_json::from_str(&actor_desc).unwrap();
        println!("{result_into_value}");
        println!("{}", to_string_pretty(&result_into_value).unwrap());

        let mut map: BTreeMap<String, ParametersType> = BTreeMap::new();
        map.insert("oui".to_string(), ParametersType::String("Yes".to_string()));
        map.insert("yes".to_string(), ParametersType::String("douze".to_string()));

        let result = add_actor_function_request(
            "actor_name".to_string(),
            "function_name".to_string(),
            map,
            actor_desc.to_string(),
        );

        println!("{result}");

        let result_into_value: Value = serde_json::from_str(&result).unwrap();

        println!("{result_into_value}");

        println!("{}", to_string_pretty(&result_into_value).unwrap());
    }
}
