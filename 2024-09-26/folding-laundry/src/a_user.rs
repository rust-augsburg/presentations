//! Problem: We receive users from the network. Our database layer has a different representation of the users.
//! So we need to transform the vector of [NetworkUser]s into [DbUser]s.

// region:    --- Boilerplate

struct NetworkUser {
    first_name: String,
    last_name: String,
}

impl NetworkUser {
    pub fn new(first_name: &str, last_name: &str) -> Self {
        Self {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
        }
    }
}

#[derive(Debug)]
struct DbUser {
    full_name: String,
}

/// Mock network receive.
fn receive_users() -> Vec<NetworkUser> {
    vec![
        NetworkUser::new("anna", "wood"),
        NetworkUser::new("berta", "stone"),
        NetworkUser::new("cecile", "miller"),
        NetworkUser::new("diana", "winter"),
        NetworkUser::new("esther", "smith"),
    ]
}

/// Mock database write.
fn save_users(users: &[DbUser]) {
    println!("Saved following users to the database\n{users:#?}");
}

// endregion: --- Boilerplate

// region:    --- Solutions

/// Classical `for` loop solution
fn classical_transform_users(users: Vec<NetworkUser>) -> Vec<DbUser> {
    let mut db_users = Vec::new();

    for network_user in users {
        let full_name = format!("{} {}", network_user.first_name, network_user.last_name);
        let db_user = DbUser { full_name };
        db_users.push(db_user);
    }

    db_users
}

/// How do we get a vector of [DbUser]s from a Vector of [NetworkUser]s?
fn transform_users(users: Vec<NetworkUser>) -> Vec<DbUser> {
    todo!()
}

/// Verbose, step-by-step functional approach.
/// 1. Build an iterator of the vector of users
/// 2. Say: "For each [NetworkUser], I want to perform the steps within the curly braces"
/// 3. After all steps, collect the list of resulting [NetworkUser]s into a new vector.
fn transform_users_verbose(users: Vec<NetworkUser>) -> Vec<DbUser> {
    users
        .into_iter()
        .map(|u| {
            let full_name = format!("{} {}", u.first_name, u.last_name);

            let db_user = DbUser { full_name };

            db_user
        })
        .collect()
}

/// Cuts down a little on the verbosity.
/// You can pass functions to combinators like `map`.
///
/// *Rule*: When the signature of your function matches the signature of your combinator, you can
/// just pass "the name of the function" without specifying any arguments for it to the combinator.
/// So the call to `map` in this function is equivalent to:
/// ```Rust,ignore
/// # let users: Vec<NetworkUser> = vec![];
/// users.into_iter().map(|network_user| transform(network_user)).collect()
/// ```
fn transform_users_neat(users: Vec<NetworkUser>) -> Vec<DbUser> {
    fn transform(user: NetworkUser) -> DbUser {
        DbUser {
            full_name: format!("{} {}", user.first_name, user.last_name),
        }
    }

    users.into_iter().map(transform).collect()
}

// -- The next approach is a little neater. We implement a conversion trait (NetworkUser -> DbUser) and call that in the `map` combinator.

impl From<NetworkUser> for DbUser {
    fn from(user: NetworkUser) -> Self {
        DbUser {
            full_name: format!("{} {}", user.first_name, user.last_name),
        }
    }
}
fn transform_users_neater(users: Vec<NetworkUser>) -> Vec<DbUser> {
    users.into_iter().map(DbUser::from).collect()
}

// endregion: --- Solutions

#[test]
fn run_solution() {
    let network_users = receive_users();
    let db_users = transform_users(network_users);
    println!("Chained");
    save_users(&db_users);

    let network_users = receive_users();
    let db_users = classical_transform_users(network_users);
    println!("Classical");
    save_users(&db_users);

    let network_users = receive_users();
    let db_users = transform_users_verbose(network_users);
    println!("Verbose");
    save_users(&db_users);

    let network_users = receive_users();
    let db_users = transform_users_neat(network_users);
    println!("Neat");
    save_users(&db_users);

    let network_users = receive_users();
    let db_users = transform_users_neater(network_users);
    println!("Neater");
    save_users(&db_users);
}
