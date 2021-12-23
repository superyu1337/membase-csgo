use std::{collections::HashMap, convert::TryInto};

use toml::{Value, value::Table};

// to do: Automatically download offsets
pub fn get_offsets() -> HashMap<String, usize> {
    let offsets = read_offsets_from_file();

    let netvars = offsets["netvars"].as_table().unwrap().clone();
    let signatures = offsets["signatures"].as_table().unwrap().clone();

    let mut offset_table = Table::from(netvars);
    offset_table.extend(signatures.into_iter());

    let mut final_table: HashMap<String, usize> = HashMap::new();

    for (key, value) in offset_table {
        final_table.insert(key, value.as_integer().unwrap().try_into().unwrap());
    }

    return final_table;
}

fn read_offsets_from_file() -> Value {
    let file_content= std::fs::read("./offsets.toml")
        .expect("Read offsets file");

    let file_content = String::from_utf8(file_content)
        .expect("Convert bytes to string");

    file_content.parse::<Value>()
        .expect("Convert TOML string to data")
}