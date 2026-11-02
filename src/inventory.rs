// Add new
pub fn add_item(backpack: &mut Vec<String>, item: String) {
    backpack.push(item);
}

// Tulostus
pub fn display_inventory(backpack: &[String]) {
    println!("Backpack:");
    for item in backpack {
        println!("- {}", item);
    }
}