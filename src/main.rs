// #![allow(unused_variables, dead_code, unused_imports)]

use crate::fixed_length_record::{Field, FixedLengthRecord, PadLocation};
use crate::resources_todos::{get_todos, GetTodosOptions};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;

pub mod fixed_length_record;
pub mod resources_todos;

fn main() {
    let record = FixedLengthRecord {
        name: "TODOS",
        fields: vec![
            Field {
                name: "USER_ID",
                start: 0,
                length: 4,
                pad_location: PadLocation::Start,
                pad_string: Some('0'),
                default: None,
            },
            Field {
                name: "ID",
                start: 4,
                length: 4,
                pad_location: PadLocation::Start,
                pad_string: Some('0'),
                default: None,
            },
            Field {
                name: "TITLE",
                start: 8,
                length: 100,
                pad_location: PadLocation::End,
                pad_string: None,
                default: None,
            },
            Field {
                name: "COMPLETED",
                start: 108,
                length: 1,
                pad_location: PadLocation::End,
                pad_string: None,
                default: None,
            },
        ],
    };

    let file_name = "data.txt";
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_name)
        .expect("is not possible open file");

    let mut options = GetTodosOptions {
        limit: 10,
        offset: 0,
    };

    loop {
        let todos = get_todos(options).expect("msg");

        todos
            .iter()
            .map(|todo| {
                let user_id = todo.user_id.to_string();
                let id = todo.id.to_string();
                let title = todo.title.to_uppercase();
                let completed = (if todo.completed { 1 } else { 0 }).to_string();

                HashMap::from([
                    ("USER_ID", user_id),
                    ("ID", id),
                    ("TITLE", title),
                    ("COMPLETED", completed),
                ])
            })
            .for_each(|data| writeln!(file, "{}", record.build(&data)).expect("todo mal"));

        if todos.len() < options.limit {
            break;
        } else {
            options.offset += options.limit;
        }
    }
}
