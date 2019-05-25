#[macro_use]
extern crate serde_derive;
extern crate serialize;
extern crate serde;
extern crate serde_json;
extern crate valico;

use std::fs::File;
use std::io::Read;
use serde_json::Value;

use serialize::json;
use valico::json_dsl;
use valico::json_schema;


fn main() {
    // let mut file = File::open("/Users/ninanjohn/Trials/rustTrials/stub_verifier/src/text.json").unwrap();
    // let mut data = String::new();
    // file.read_to_string(&mut data).unwrap();
    

    // let json:Value = serde_json::from_str(&data).unwrap();
    // println!("{}", json["Address"]["City"]);

	
	let mut schemaFile = File::open("/Users/ninanjohn/Trials/rustTrials/stub_verifier/src/schema.json").unwrap();
    let mut jsonSchemaString = String::new();
    schemaFile.read_to_string(&mut jsonSchemaString).unwrap();

    let json_v4_schema: json::Json = serde_json::from_str(&jsonSchemaString).unwrap();

    let mut scope = json_schema::Scope::new();
    let schema = scope.compile_and_return(json_v4_schema.clone()).ok().unwrap();

    println!("Is valid: {}", schema.validate(&json_v4_schema).is_valid())
    // let state = jsonSchemaString.process(&mut json_v4_schema, &None);	

    // println!("Is valid: {}", state.is_valid())

}