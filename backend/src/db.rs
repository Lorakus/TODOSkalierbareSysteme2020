use crate::models::{TodoList};
use deadpool_postgres::Client;
use std::io;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn get_todos(client: &Client) -> Result<Vec<TodoList>, io::Error>{

    let statement = client.prepare("select * from todo_list").await.unwrap();

    let todos = client.query(&statement, &[])
        .await
        .expect("ERROR GETTING TODO")
        .iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<TodoList>>();
    Ok(todos)
}