use std::collections::HashMap;

#[derive(Debug)]
enum ItemType {
    Weapon,
    Armor,
    Potion,
}

#[derive(Debug)]
struct Item {
    name: String,
    item_type: ItemType,
    value: u32,
}

struct Inventory {
    items: HashMap<String, Item>,
}

impl Inventory {
    fn new() -> Inventory {
        Inventory {
            items: HashMap::new(),
        }
    }

    fn add_item(&mut self, item: Item) {
        self.items.insert(item.name.clone(), item);
    }

    fn remove_item(&mut self, name: &str) -> Option<Item> {
        self.items.remove(name)
    }

    fn show_inventory(&self) {
        if self.items.is_empty() {
            println!("Inventory is empty.");
        } else {
            for item in self.items.values() {
                match item.item_type {
                    ItemType::Weapon => println!("Weapon: {:?} - Value: {}", item.name, item.value),
                    ItemType::Armor => println!("Armor: {:?} - Value: {}", item.name, item.value),
                    ItemType::Potion => println!("Potion: {:?} - Value: {}", item.name, item.value),
                }
            }
        }
    }
}

fn main() {
    let mut inventory = Inventory::new();

    let sword = Item {
        name: "Sword of Destiny".to_string(),
        item_type: ItemType::Weapon,
        value: 500,
    };

    let shield = Item {
        name: "Shield of Hope".to_string(),
        item_type: ItemType::Armor,
        value: 300,
    };

    let healing_potion = Item {
        name: "Healing Potion".to_string(),
        item_type: ItemType::Potion,
        value: 50,
    };

    inventory.add_item(sword);
    inventory.add_item(shield);
    inventory.add_item(healing_potion);

    println!("Current Inventory:");
    inventory.show_inventory();

    println!("\nRemoving Healing Potion...");
    inventory.remove_item("Healing Potion");

    println!("\nUpdated Inventory:");
    inventory.show_inventory();
}
