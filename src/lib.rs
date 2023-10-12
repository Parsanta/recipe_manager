use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{Write, Read},
};

#[derive(Debug)]
pub struct RecipeManager {
    pub recipes: HashMap<String, String>,
}

impl RecipeManager {
    pub fn new() -> Self {
        RecipeManager {
            recipes: HashMap::new(),
        }
    }

    pub fn store(&mut self, file: &mut File, name: String, rec: String) {
        file.write_all(format!("name:{}\n Recipes:{}\n \n", name, rec).as_bytes())
            .expect("error in writing to file");
    }

    pub fn add(&mut self, name: &str, rec: &str) {
        self.recipes.insert(name.to_string(), rec.to_string());
    }

    pub fn search(&self, name: &str) -> Option<&str> {
        if let Some(recipe) = self.recipes.get(name) {
            Some(recipe)
        } else {
            None
        }
    }

    pub fn view_from_file(&self){
        let mut file = File::open("data.txt").expect("error openin the file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("error reading the file");
        let records: Vec<&str> = contents.trim().split("\n \n").collect();
        println!("{:#?}",records);
    }

    pub fn edit(&mut self, name: &String, rec: &String) {
        self.add(name, rec);
    }

    pub fn remove(&mut self, name: &String) {
        self.recipes.remove(name);
    }

    pub fn save_to_file(&self) {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("data.txt")
            .expect("Error opening file for writing");

        for (name, recipe) in &self.recipes {
            let content = format!("name:{}\nRecipes:{}\n", name, recipe);
            file.write_all(content.as_bytes())
                .expect("Error writing to file");
        }
    }
}
