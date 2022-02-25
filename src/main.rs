#[macro_use]
extern crate colour;
use colored::*;
use serde::{Deserialize, Serialize};
use serde_json;
use std::error::Error;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
struct JsonStruct {
    users: Vec<User>,
}

#[derive(Debug, Deserialize, Serialize)]
struct User {
    name: String,
    age: u8,
    id: String,
}

// create a function to read a json file
fn read_json(path: &str) -> Result<JsonStruct, Box<dyn Error>> {
    let template = JsonStruct { users: vec![] };

    if !Path::new(path).exists() {
        write_json(&template, path)?;
        return Ok(template);
    }

    let data = fs::read_to_string(path)?;

    if data.len() == 0 {
        write_json(&template, "./db.json").unwrap();
        return Ok(template);
    }
    let json: JsonStruct = serde_json::from_str(&data)?;
    Ok(json)
}

// create a function to write a json file
fn write_json(data: &JsonStruct, path: &str) -> Result<(), Box<dyn Error>> {
    let json = serde_json::to_string_pretty(&data)?;
    fs::write(path, json)?;
    Ok(())
}

// create a function to prompt the user for input
fn input_int(prompt: &str) -> u8 {
    green!(prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<u8>().unwrap()
}

// create a function to prompt the user for input
fn input_str(prompt: &str) -> String {
    green!(prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

// create a menu
fn menu() -> u8 {
    yellow!("Welcome to the user manager\n");
    cyan!("1. Agregar un usuario\n");
    cyan!("2. Eliminar a un usuario\n");
    cyan!("3. Editar usuario\n");
    cyan!("4. Listar usuarios\n");
    cyan!("5. Salir\n");
    let choice = input_int("Ingrese una opción: ");
    choice
}

fn main() {
    green!("Base de datos escrita en Rust 🦀\n");
    loop {
        let option = menu();

        // Read the json file
        let mut data: JsonStruct = read_json("./db.json").unwrap();

        if option == 1 {
            let name = input_str("Enter your name: ");
            let age = input_int("Enter your age: ");

            let id = Uuid::new_v4().to_string();

            data.users.push(User { name, age, id });

            write_json(&data, "./db.json").unwrap();
        } else if option == 2 {
            let name = input_str("Enter your name: ");

            for (i, user) in data.users.iter().enumerate() {
                if user.name == name {
                    data.users.remove(i);
                    break;
                }
            }

            write_json(&data, "./db.json").unwrap();
        } else if option == 3 {
            let name = input_str("Enter your name: ");
            for (i, user) in data.users.iter().enumerate() {
                if user.name == name {
                    let new_name = input_str("Enter your new name: ");
                    let new_age = input_int("Enter your new age: ");
                    data.users[i].name = new_name;
                    data.users[i].age = new_age;
                    break;
                }
            }
            write_json(&data, "./db.json").unwrap();
        } else if option == 4 {
            for user in data.users.iter() {
                println!("{}", "-----------------".green());
                println!("Mi nombre es {}", user.name.red());
                println!("Mi edad es {}", user.age.to_string().yellow());
            }
        } else if option == 5 {
            break;
        }
    }
}
