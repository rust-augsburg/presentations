//! Problem: We receive users from the network. We want to gather specific information form a set
//! of received user data.
//! Task 1: get the sum of the users' ages!
//! Task 2: sum only the ages of users whose name ends in 'a'!
//! Task 3: count all occurrences of the character `a` in the user names!

// region:    --- Boilerplate
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

fn receive_users() -> Vec<NetworkUser> {
    vec![
        NetworkUser::new("anna", "75"),
        NetworkUser::new("berta", "87"),
        NetworkUser::new("cecile", "16"),
        NetworkUser::new("diana", "31"),
        NetworkUser::new("esther", "44"),
    ]
}

// endregion: --- Boilerplate

// region:    --- Solutions

/// Sum of all ages
fn task_01_for_loop(users: &[NetworkUser]) -> u32 {
    let mut sum = 0;
    for u in users {
        sum += u.age.parse::<u32>().unwrap();
    }
    sum
}

fn task_01(users: &[NetworkUser]) -> u32 {
    todo!()
}

/// Task 2: sum only the ages of users whose name ends in 'a'!
fn task_02_for_loop(users: &[NetworkUser]) -> u32 {
    let mut sum = 0;
    for u in users {
        if u.name.ends_with('a') {
            sum += u.age.parse::<u32>().unwrap();
        }
    }
    sum
}

fn task_02(users: &[NetworkUser]) -> u32 {
    todo!()
}

/// Task 3: count all occurrences of the character `a` in the user names!
///
/// The solution that is commented out combines a for loop with a `filter`
/// and a `count` operation on the `chars` iterator.
fn task_03_for_loop(users: &[NetworkUser]) -> u32 {
    // let mut sum = 0;
    // for u in users {
    //     let chars = u.name.chars();
    //     sum += chars.filter(|&c| c == 'a').count() as u32;
    // }
    // sum

    let mut sum = 0;
    for u in users {
        for c in u.name.chars() {
            sum += (c == 'a') as u32;
        }
    }
    sum
}

fn task_03(users: &[NetworkUser]) -> u32 {
    todo!()
}
// endregion: --- Solutions

#[test]
fn run_solution() {
    let users = receive_users();

    let users = receive_users();

    // Task: get the sum of the users' ages!
    let age_sum = task_01(&users);
    println!("Sum of all ages: {age_sum}");
    // Task: sum only the ages of users whose name ends in 'a'!
    let age_sum = task_02(&users);
    println!("Sum of all ages of users with names ending in `a`: {age_sum}");

    // Task: count all occurrences of the character `a` in the user names!
    let a_count = task_03(&users);
    println!("Count of the letter `a` in all names: {a_count}");
}
