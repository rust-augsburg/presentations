//! Problem: We have multiple layers of optional values. We only care for the values that are actually there.
//! How do we sum up all optional ages of optional users?

// region:    --- Boilerplate
struct User {
    age: Option<u8>,
}

fn mock_users() -> Vec<Option<User>> {
    vec![
        Some(User { age: None }),
        Some(User { age: Some(33) }),
        None,
        Some(User { age: Some(55) }),
        Some(User { age: None }),
        Some(User { age: Some(77) }),
    ]
}

// endregion: --- Boilerplate

// region:    --- Solutions

fn classic_sum_user_ages(users: &[Option<User>]) -> u32 {
    let mut sum = 0;
    for user in users {
        let Some(user) = user else {
            continue;
        };

        let Some(age) = user.age else {
            continue;
        };
        sum += age as u32;
    }
    sum
}

fn sum_user_ages(users: &[Option<User>]) -> u32 {
    todo!()
}

fn verbose_sum_user_ages(users: &[Option<User>]) -> u32 {
    users
        .iter()
        .flatten()
        .map(|u| u.age)
        .flat_map(|u| u.map(|u| u as u32))
        .sum()
}
// endregion: --- Solutions

// region:    --- Testing
#[test]
fn test_solutions() {
    let users = mock_users();

    let sum = classic_sum_user_ages(&users);
    println!("classical sum: {sum}");

    let sum = sum_user_ages(&users);
    println!("chained sum: {sum}");

    let sum = verbose_sum_user_ages(&users);
    println!("verbose chained sum: {sum}");
}

// endregion: --- Testing
