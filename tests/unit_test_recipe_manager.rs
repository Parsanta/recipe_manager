#[cfg(test)]
mod tests {
    use recipe_manager::RecipeManager;
    #[test]
    fn test_add_recipe() {
        let mut recipe_manager = RecipeManager::new();
        recipe_manager.add("apple", "mash");
        assert_eq!(recipe_manager.recipes.get("apple"), Some(&"mash".to_string()));
    }

    #[test]
    fn test_search_recipe() {
        let mut recipe_manager = RecipeManager::new();
        recipe_manager.add("apple", "mash");
        assert_eq!(recipe_manager.search("apple"), Some("mash"));
    }

    #[test]
    fn test_search_recipe_not_found() {
        let recipe_manager = RecipeManager::new();
        assert_eq!(recipe_manager.search("non_existent_recipe"), None);
    }

    #[test]
    fn test_edit_recipe() {
        let mut recipe_manager = RecipeManager::new();
        recipe_manager.add("apple", "mash");
        recipe_manager.edit(&"apple".to_string(), &"updated_mash".to_string());
        assert_eq!(recipe_manager.recipes.get("apple"), Some(&"updated_mash".to_string()));
    }

    #[test]
    fn test_remove_recipe() {
        let mut recipe_manager = RecipeManager::new();
        recipe_manager.add("apple", "mash");
        recipe_manager.remove(&"apple".to_string());
        assert_eq!(recipe_manager.recipes.get("apple"), None);
    }
}
