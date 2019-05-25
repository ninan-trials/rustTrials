#![recursion_limit = "128"]
extern crate reqwest;
#[macro_use]
extern crate serde_json;

use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::process;

use serde_json::Value;

use crate::pipeline_config::PipelineConfig;

mod pipeline_config;

pub struct InputArguments {
    json_filename: String,
}

impl InputArguments {
    pub fn new(args: &[String]) -> Result<InputArguments, &'static str> {
        if args.len() < 2 {
            return Err("Requires additional parameters");
        }
        let json_filename = args[1].clone();

        Ok(InputArguments { json_filename })
    }
}

pub fn create_pipeline_from_file(input: InputArguments) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(&input.json_filename)?;
    let pipeline_config = create_pipeline_config_from_json_string(file_content)
        .unwrap_or_else(|err| {
            println!("Pipeline Config could not be created: {}", err);
            process::exit(1);
        });

    let returned_json: serde_json::Value = reqwest::Client::new()
        .post("http://localhost:8153/go/api/admin/pipelines")
        .header("Accept", "application/vnd.go.cd.v6+json")
        .header("Content-Type", "application/json")
        .json(&pipeline_config)
        .send()?
        .json()?;

    let mut errors = &returned_json["data"]["errors"].as_object();
    if errors.is_none() {
        println!("No Errors")
    } else {
        let errors = errors.unwrap();
        println!("Errors: {:#?}", errors.get("name").unwrap());
    }
    Ok(())
}

fn create_pipeline_config_from_json_string(json_string: String) -> Result<PipelineConfig, Box<dyn Error>> {
    let pipeline_config: PipelineConfig = serde_json::from_str(json_string.as_str())?;
    Ok(pipeline_config)
}
