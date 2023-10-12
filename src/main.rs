pub mod lib;

use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use lib::RecipeManager;

fn get_user_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid input");
    input.trim().to_string()
}

fn main() {
    let mut input = String::new();
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("data.txt")
        .expect("Failed to open the file");

    let mut recipe_manager = RecipeManager::new();

    loop {
        println!("Recipes Manager");
        println!("Enter number accordingly");
        println!("1) Add recipe");
        println!("2) Search Recipe");
        println!("3) View all recipes");
        println!("4) Delete Recipe");
        println!("5) Edit Recipe");
        println!("6) Exit");

        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Invalid input");

        let input: i64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Invalid number.");
                continue;
            }
        };

        match input {
            1 => {
                let name = get_user_input("Enter the name for the recipe:");
                let recipe = get_user_input("Enter the recipe:");
                recipe_manager.add(&name, &recipe);
                recipe_manager.store(&mut file, name, recipe);
            }
            2 => {
                let name = get_user_input("Enter the name for the recipe search:");
                if let Some(recipe) = recipe_manager.search(name.trim()) {
                    println!("{}", recipe);
                } else {
                    println!("Recipe not found");
                }
            }
            3 => {
                println!("ALL RECIPES:");
                recipe_manager.view_from_file();
            }
            4 => {
                let name = get_user_input("Enter the name of the recipe to delete:");
                recipe_manager.remove(&name);
                recipe_manager.save_to_file();
            }
            5 => {
                let name = get_user_input("Enter the name of the recipe to edit:");
                let recipe = get_user_input("Enter the updated recipe:");
                recipe_manager.edit(&name, &recipe);
                recipe_manager.save_to_file();
            }
            6 => {
                break;
            }
            _ => {
                println!("Choose only from the mentioned options");
                continue;
            }
        }
    }
}
