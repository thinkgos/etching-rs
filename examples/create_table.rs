use sea_orm::entity::EntityName;
use sea_orm::{DbBackend, Schema, Statement};
use std::error::Error;
use std::fs::File;
use std::io::Write;

use entity::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let db_mysql = DbBackend::MySql;
    let schema = Schema::new(db_mysql);

    let content = db_mysql
        .build(&schema.create_table_from_entity(Announce))
        .to_string();

    let a = Announce {};
    let filename = format!("{}.sql", a.table_name());
    write_file(&filename, &content)?;
    Ok(())
}

fn write_file(filename: &str, content: &str) -> Result<(), Box<dyn Error>> {
    File::create(filename)?.write_all(content.as_bytes())?;

    Ok(())
}
