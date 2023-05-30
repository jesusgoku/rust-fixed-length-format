#![allow(unused_variables, dead_code)]

use crate::fixed_length_record::{Field, FixedLengthRecord, PadLocation};

use std::collections::HashMap;

pub mod fixed_length_record;

fn main() {
    let record = FixedLengthRecord {
        name: "XX",
        fields: vec![
            Field {
                name: "TYPE",
                start: 0,
                length: 2,
                pad_location: PadLocation::End,
                pad_string: None,
                default: Some(String::from("XX")),
            },
            Field {
                name: "NAME",
                start: 2,
                length: 50,
                pad_location: PadLocation::End,
                pad_string: None,
                default: None,
            },
        ],
    };

    [
        HashMap::from([("TYPE", "XX"), ("NAME", "JOHN")]),
        HashMap::from([("TYPE", "XX"), ("NAME", "JANE")]),
        HashMap::from([("TYPE", "XX")]),
        HashMap::from([("NAME", "JOHN")]),
    ]
    .iter()
    .for_each(|data| {
        println!("{}", record.build(&data));
    });
}
