mod inventory;
mod summary;

fn main() {
    // 5 Esinettä, 1 esine 2 kertaa.
    let mut backpack: Vec<String> = vec![
        String::from("Rope"),
        String::from("Potion"),
        String::from("Potion"),
        String::from("Map"),
        String::from("Torch"),
    ];

    
    inventory::add_item(&mut backpack, String::from("Silver key"));

    //Tulostetaan.
    inventory::display_inventory(&backpack);

    println!();

    // Lasku ja tulostus.
    summary::print_item_summary(&backpack);
}