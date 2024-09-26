//! Problem: We want to transform our input data based on a rule, that takes following elements into account.
//!          I.e.: I have a list of meals. I want to filter out that list in a way, that only savory meals, followed
//!                by sweet meals are left over.
//! ATTRIBUTION: Thank you Aron for providing pointing out the use case of `windows` and providing this example in different
//!              words.

// region:    --- Boilerplate

struct Meal {
    name: &'static str,
    taste: Taste,
}

enum Taste {
    Savory,
    Sweet,
}

fn mock_meals() -> Vec<Meal> {
    vec![
        Meal {
            name: "Burger",
            taste: Taste::Savory,
        },
        Meal {
            name: "Cake",
            taste: Taste::Sweet,
        },
        Meal {
            name: "Pizza",
            taste: Taste::Savory,
        },
        Meal {
            name: "Pasta",
            taste: Taste::Savory,
        },
        Meal {
            name: "Ice-cream",
            taste: Taste::Sweet,
        },
        Meal {
            name: "Sandwich",
            taste: Taste::Savory,
        },
        Meal {
            name: "Mango",
            taste: Taste::Sweet,
        },
    ]
}

// endregion: --- Boilerplate

// region:    --- Solutions

/// Use a for loop.
fn classical_solution(meals: Vec<Meal>) -> Vec<&'static str> {
    let mut followed_by_sweet_dish = Vec::new();
    for (i, meal) in meals.iter().enumerate() {
        let Some(next_meal) = meals.get(i + 1) else {
            break;
        };

        if matches!(meal.taste, Taste::Sweet) || matches!(next_meal.taste, Taste::Savory) {
            continue;
        }

        followed_by_sweet_dish.push(meal.name);
    }

    followed_by_sweet_dish
}

fn chaining_solution(meals: Vec<Meal>) -> Vec<&'static str> {
    meals
        .windows(2)
        .filter_map(|pair| {
            if matches!(pair[0].taste, Taste::Sweet) || matches!(pair[1].taste, Taste::Savory) {
                None
            } else {
                Some(pair[0].name)
            }
        })
        .collect()
    // Additional exercise: Can you stop iterating after your resulting list has 2 entries in it?
}

// endregion: --- Solutions

// region:    --- Tests
#[test]
fn test_classical() {
    let meals = mock_meals();
    let names = classical_solution(meals);
    println!("{names:#?}");
}

#[test]
fn test_chained() {
    let meals = mock_meals();
    let names = chaining_solution(meals);
    println!("{names:#?}");
}
// endregion: --- Tests
