// This program simulates the distribution of aid kits (food or hygiene supplies)
// to families affected by floods. Families can register their preference for a kit type,
// such as food supplies or hygiene products. If a preference is provided, they receive the
// specified kit type. If no preference is provided, the program will select the kit type
// that is more available in stock.

#[derive(Debug, PartialEq, Copy, Clone)]
enum KitType {
    Food,
    Hygiene,
}

struct Stock {
    kits: Vec<KitType>,
}

impl Stock {
    // Distribute a kit based on user preference or availability
    fn distribute_kit(&self, preference: Option<KitType>) -> KitType {
        // Check if the user has a preference; if not, choose the most available kit type
        // Change this if statement to use closures... <<++++++++++++++++++++++++++++++++
        if let Some(pref) = preference {
            pref
        } else {
            self.most_available()
        }
    }

    // Determine which kit type has the most items available
    fn most_available(&self) -> KitType {
        let mut num_food = 0;
        let mut num_hygiene = 0;

        // Count the quantity of each kit type in stock
        for kit in &self.kits {
            match kit {
                KitType::Food => num_food += 1,
                KitType::Hygiene => num_hygiene += 1,
            }
        }
        // Return the kit type with the higher count
        if num_food > num_hygiene {
            KitType::Food
        } else {
            KitType::Hygiene
        }
    }
}

fn main() {
    // Initialize the stock with available kits
    let stock = Stock {
        kits: vec![KitType::Hygiene, KitType::Food, KitType::Food],
    };

    // Family 1 has a preference for a food kit
    let family_pref1 = Some(KitType::Food);
    let kit_for_family1 = stock.distribute_kit(family_pref1);
    println!(
        "The family with preference {:?} receives the {:?} kit",
        family_pref1, kit_for_family1
    );

    // Family 2 has no preference
    let family_pref2 = None;
    let kit_for_family2 = stock.distribute_kit(family_pref2);
    println!(
        "The family with no preference receives the {:?} kit",
        kit_for_family2
    );
}
