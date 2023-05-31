use pad::{Alignment, PadStr};
use std::collections::HashMap;

pub enum PadLocation {
    Start,
    End,
    Both,
}

pub struct Field<'a> {
    pub name: &'a str,
    pub start: usize,
    pub length: usize,
    pub pad_location: PadLocation,
    pub pad_string: Option<char>,
    pub default: Option<String>,
}

pub struct FixedLengthRecord<'a> {
    pub name: &'a str,
    pub fields: Vec<Field<'a>>,
}

impl FixedLengthRecord<'_> {
    // pub fn build(&self, data: &HashMap<&str, &str>) -> String {
    pub fn build(&self, data: &HashMap<&str, String>) -> String {
        let fields: Vec<String> = self
            .fields
            .iter()
            .map(|field| {
                let pad_location = match field.pad_location {
                    PadLocation::Start => Alignment::Right,
                    PadLocation::End => Alignment::Left,
                    PadLocation::Both => Alignment::Middle,
                };

                let pad_string = match field.pad_string {
                    Some(pad_string) => pad_string,
                    None => ' ',
                };

                let field_data = match data.get(field.name) {
                    Some(name) => name,
                    None => "",
                };

                field_data.pad(field.length, pad_string, pad_location, true)
            })
            .collect();

        fields.join("")
    }
}
