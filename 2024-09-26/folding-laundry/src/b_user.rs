//! Problem: We receive users from the network. Our database layer has a different representation of the users.
//! So we need to transform the vector of [NetworkUser]s into [DbUser]s.

// region:    --- Boilerplate
use std::num::ParseIntError;

struct NetworkUser {
    name: String,
    age: String,
}

impl NetworkUser {
    pub fn new(name: &str, age: &str) -> Self {
        Self {
            name: name.to_string(),
            age: age.to_string(),
        }
    }
}

#[derive(Debug)]
struct DbUser {
    name: String,
    age: u8,
}

fn receive_users() -> Vec<NetworkUser> {
    vec![
        NetworkUser::new("anna", "75"),
        NetworkUser::new("berta", "87"),
        NetworkUser::new("cecile", "16"),
        NetworkUser::new("diana", "31"),
        NetworkUser::new("esther", "44"),
    ]
}

fn save_users(users: &[DbUser]) {
    println!("Saved following users to the database\n{users:#?}");
}

// endregion: --- Boilerplate

// region:    --- Solutions

fn transform_users(network_users: Vec<NetworkUser>) -> Vec<DbUser> {
    todo!()
}

fn very_verbose_transform_and_log_errors(network_users: Vec<NetworkUser>) -> Vec<DbUser> {
    network_users
        .into_iter()
        .map(|network_user| {
            let maybe_valid_age = network_user.age.parse();
            if maybe_valid_age.is_err() {
                eprintln!("ERROR!");
                return Err(maybe_valid_age);
            }

            let valid_age = maybe_valid_age.unwrap();
            Ok(DbUser {
                name: network_user.name,
                age: valid_age,
            })
        })
        .filter(|conversion_result| conversion_result.is_ok())
        .map(|positive_result| positive_result.unwrap())
        .collect()
}

fn less_verbose_transform_and_ignore_errors(network_users: Vec<NetworkUser>) -> Vec<DbUser> {
    network_users
        .into_iter()
        .filter_map(|network_user| {
            let valid_age = network_user.age.parse().ok()?; // <-- `.ok()` throws away Result(OK or ERR) and turns it into Option(SOME or NONE)

            Some(DbUser {
                name: network_user.name,
                age: valid_age,
            })
        })
        .collect()
}

// -- Nice solution: Implement try from:
impl TryFrom<NetworkUser> for DbUser {
    type Error = ParseIntError;

    fn try_from(network_user: NetworkUser) -> Result<Self, Self::Error> {
        let valid_age = network_user.age.parse()?;
        Ok(Self {
            name: network_user.name,
            age: valid_age,
        })
    }
}

fn nice_transform_and_log_errors(network_users: Vec<NetworkUser>) -> Vec<DbUser> {
    network_users
        .into_iter()
        .map(DbUser::try_from)
        .filter_map(|result| {
            if let Err(e) = result {
                eprintln!("INVALID AGE: {e}");
                return None;
            }
            result.ok()
        })
        .collect()
}

// -- Also filter out any user that is less than 60 years old
fn transform_and_filter(network_users: Vec<NetworkUser>) -> Vec<DbUser> {
    network_users
        .into_iter()
        .map(DbUser::try_from)
        .filter_map(|result| {
            if let Err(e) = result {
                eprintln!("INVALID AGE: {e}");
                return None;
            }
            result.ok()
        })
        .filter(|db_user| db_user.age > 60) // <-- this means: Only keep users where `db_user.age > 60` is `true`!
        .collect()
}

// -- Also filter out any user that is less than 60 years old
fn transform_and_filter_and_format(network_users: Vec<NetworkUser>) -> Vec<DbUser> {
    network_users
        .into_iter()
        .map(DbUser::try_from)
        .filter_map(|result| {
            if let Err(e) = result {
                eprintln!("INVALID AGE: {e}");
                return None;
            }
            result.ok()
        })
        .filter(|db_user| db_user.age > 60) // <-- this means: Only keep users where `db_user.age > 60` is `true`!
        .map(|db_user| DbUser {
            name: db_user.name.to_uppercase(),
            age: db_user.age,
        })
        .collect()
}

// endregion: --- Solutions

#[test]
fn run_solution() {
    let users = receive_users();

    // Task: transform network users to database users!
    let users = transform_users(users);
    // let users = very_verbose_transform_and_log_errors(users);
    // let users = less_verbose_transform_and_ignore_errors(users);
    // let users = nice_transform_and_log_errors(users);
    // let users = transform_and_filter(users);
    // let users = transform_and_filter_and_format(users);

    save_users(&users);
}
