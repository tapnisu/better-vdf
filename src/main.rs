use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct Test {
    test: TestData,
}

#[derive(Serialize, Deserialize, Debug)]
struct TestData {
    name: String,
    list: Vec<TestObj>,
    map: HashMap<u64, i64>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TestObj {
    obj: String,
    id: usize,
    weight: f32,
}

fn main() {
    let vdf = r#"
     "test"
     {
     	"name"		"Better VDF"
     	"list"
     	{
     		"0"
     		{
     			"obj"		"main_obj"
     			"id"		"19231"
     			"weight"		"12.9"
     		}
     		"1"
     		{
     			"obj"		"secondary_obj"
     			"id"		"381928"
     			"weight"		"5.12"
     		}
     	}
     	"map"
     	{
     		"228980"		"12318293"
     		"278319"		"-12393180"
     	}
     }
     "#;

    // Deserializing
    let test: Test = better_vdf::from_str(vdf).unwrap();

    println!("{test:#?}");

    // Serializing
    let serial = better_vdf::to_string(&test).unwrap();

    println!("{serial}");
}