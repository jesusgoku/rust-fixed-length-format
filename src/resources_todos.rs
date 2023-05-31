use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Todo {
    pub user_id: usize,
    pub id: usize,
    pub title: String,
    pub completed: bool,
}

#[derive(Clone, Copy)]
pub struct GetTodosOptions {
    pub limit: usize,
    pub offset: usize,
}

pub fn get_todos(options: GetTodosOptions) -> Result<Vec<Todo>, ureq::Error> {
    let GetTodosOptions { limit, offset } = options;
    let url = format!("https://jsonplaceholder.typicode.com/todos?_limit={limit}&_start={offset}");
    let body: Vec<Todo> = ureq::get(&url).call()?.into_json()?;

    return Ok(body);
}
