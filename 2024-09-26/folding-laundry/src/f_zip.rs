//! Problem: You have two vectors of related data (user first name, user last name) and want to transform them
//! into one vector of a specific data structure.

// region:    --- Boilerplate

#[derive(Debug)]
struct User {
    name: String,
    age: u8,
}

fn mock_names() -> Vec<String> {
    vec![
        "anna".to_string(),
        "berta".to_string(),
        "cecile".to_string(),
        "diana".to_string(),
        "esther".to_string(),
    ]
}

fn mock_ages() -> Vec<u8> {
    vec![22, 33, 44]
}

// endregion: --- Boilerplate

// region:    --- Solutions

fn classical_build_users(names: Vec<String>, ages: Vec<u8>) -> Vec<User> {
    let mut users = Vec::new();
    if names.len() > ages.len() {
        let mut names = names.into_iter();
        for age in ages.into_iter() {
            users.push(User {
                name: names.next().expect("checked length"), // <-- is guaranteed yield `name` strings for the length of the ages vector
                age,
            });
        }

        return users;
    }

    // `ages` vector is smaller or equal to the size of the `names` vector
    for (i, name) in names.into_iter().enumerate() {
        users.push(User { name, age: ages[i] })
    }

    users
}

fn zip_up_users(names: Vec<String>, ages: Vec<u8>) -> Vec<User> {
    todo!()
}
// endregion: --- Solutions

#[test]
fn test_solutions() {
    let names = mock_names();
    let ages = mock_ages();

    let users = classical_build_users(names, ages);
    println!("classical\n{users:#?}");

    let names = mock_names();
    let ages = mock_ages();

    let users = zip_up_users(names, ages);
    println!("chaining\n{users:#?}");
}
