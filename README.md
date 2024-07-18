# RUST - Galaxy Cube Plugin Development Toolbox

The goal of this library is to make easier the extism plugin development for the Galaxy Cube Firmware for Suricube by providing Rust functions to manage JSON quickly and easily.

To make the datatypes easy to manipulate, we had an enum as follow to cover the most of the types that can be used : 

```rust
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
```

If you don't have the wanted type that you want or you own struct, you can always use the String type and the JSON format thanks to the serde_json librarie.

The few functions that are currently implement are :
- ```get_actor_name(json_input: &str) -> String``` : Take the JSON extism input give by the firmware and return the name of the actor.

- ```parameters_json_to_map(json_input: &str) -> BTreeMap<String, ParametersType``` : Take the JSON extism input give by the firmware and return components parameters in a map that make their treatment easier.

- ```virtual_parameters_json_to_map(json_input: &str) -> BTreeMap<String, ParametersType``` : Take the JSON extism input give by the firmware and return components virtual parameters in a map that make their treatment easier.
  
- ```arguments_json_to_map(json_input: &str) -> BTreeMap<String, ParametersType``` : Take the JSON extism input give by the firmware and return function arguments in a map that make their treatment easier.

- ```parameters_map_to_json(map_input: BTreeMap<String, ParametersType>) -> String``` : Take a map of parameters to give a json file of all the parameters that can be give in output for the firmware.

- ```add_parameters_map_to_json(map_input: BTreeMap<String, ParametersType>, json_to_follow: &str) -> String``` : Take a map of parameters to append a json file with all the parameters that can be give in output for the firmware.

- ```virtual_parameters_map_to_json(map_input: BTreeMap<String, ParametersType>) -> String``` : Take a map of virtual parameters to give a json file of all the virtual parameters that can be give in output for the firmware.

- ```add_virtual_parameters_map_to_json(map_input: BTreeMap<String, ParametersType>, json_to_follow: &str) -> String``` : Take a map of virtual parameters to append a json file with all the virtual parameters that can be give in output for the firmware.

- ```message_to_user(message : &str) -> String``` : Take a string of the user to return a JSON request that will make the firmware send a message to the user.

- ```add_message_to_user(message :  &str , json_to_follow :  &str) -> String``` : Take a JSON string made with the *parameters_map_to_json*  function and add a message of the user to return a JSON request that will make the firmware send a message to the user.

- ```actor_function_request( actor_name:  &str, function_name:  &str, function_parameters_map: BTreeMap<String, ParametersType>) -> String ``` : Take a map that represent the parameters of a function and make a function request to send at the firmware that will make execute the function to the target actor.

- ```add_actor_function_request(actor_name:  &str, function_name:  &str, function_parameters_map: BTreeMap<String, ParametersType>, json_to_follow: &str) -> String``` : Take a map that represent the parameters of a function and make a function request to send at the firmware that will make execute the function to the target actor. All of this is in complement of the other json instructions.

