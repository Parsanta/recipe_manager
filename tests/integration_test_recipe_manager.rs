use recipe_manager::RecipeManager;

#[test]
fn test_integration_add_and_search_recipe() {

    let mut recipe_manager = RecipeManager::new();
    recipe_manager.add(&"Pasta".to_string(), &"Recipe for pasta".to_string());
    assert_eq!(recipe_manager.search("Pasta").unwrap(), "Recipe for pasta");
}
