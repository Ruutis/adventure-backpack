use std::collections::HashMap;

// HashMapin avulla esineiden lasku ja tulsotus.
pub fn print_item_summary(backpack: &[String]) {
    let mut counts: HashMap<String, u32> = HashMap::new();

    // Entry ja or_insert.
    for item in backpack {
        let counter = counts.entry(item.clone()).or_insert(0);
        *counter += 1;
    }

    // yhteenveto string-olioon.
    let total_items = backpack.len();
    let mut summary_text = format!("Item summary ({} items):\n", total_items);

    for (item, count) in &counts {
        summary_text.push_str(&format!("{}: {}\n", item, count));
    }

    // Tulostetaan yhteenveto.
    print!("{}", summary_text);
}