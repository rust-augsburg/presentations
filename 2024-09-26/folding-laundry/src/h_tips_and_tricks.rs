//! Working with chains means that we are working with "iterators" a lot. This module means to provide a
//! few tips on how to work with iterators.
//!
//! ## 1
//! Constructing iterators can lead to very ugly types that we do not want to carry over to functions,
//! that might need to deal with iterators.
//! Say we have some kind of function that does something to a `User` iterator and returns that iterator.
//! How do we keep its signature flexible?
//! ## 2
//! All methods that extract information for you to use immediately, *consume* an iterator, which means
//! - actual computational work will be done
//! - you will not be able to use the iterator afterwards.
//! We call the behavior of iterators `lazy`. We can chain all sorts of `map`s and `filter`s and whatnot
//! together and create a *blueprint* for computational work to be done. The actual work will be done when
//! we call consuming methods like `collect` or `count`.
//!
//! For practical purposes this means, that when we need to continuously extract iteration information, we
//! should not use iterators but solve the problem with traditional for loops.
//! ## 3
//! There are many examples when you would want to combine iterators and for loops.

// region:    --- function signatures

fn name_starts_with_c(
    character: char,
    iter: impl Iterator<Item = NetworkUser>,
) -> impl Iterator<Item = NetworkUser> {
    iter.filter(move |u| u.first_name.starts_with(character))
}

fn name_starts_with_pattern<'a>(
    pattern: &'a str,
    iter: impl Iterator<Item = NetworkUser> + 'a,
) -> impl Iterator<Item = NetworkUser> + 'a {
    iter.filter(move |u| u.first_name.starts_with(pattern))
}

// endregion: --- function signatures

// region:    --- When we do not want to use iterators

/// A somewhat contrived example that wants to filter out the first two users whose first name
/// ends with an `a` and then return a vector of at most two users.
fn contrived_example(users: &[NetworkUser]) -> Vec<NetworkUser> {
    let mut filtered_users = Vec::new();
    for u in users {
        if !u.first_name.ends_with('a') {
            continue;
        }

        filtered_users.push(u.clone());
        if filtered_users.len() >= 2 {
            break;
        }
    }
    filtered_users
}

// endregion: --- When we do not want to use iterators
/// Collect at most two users where the next user's first name has at least six letters.
fn filter_contrived(users: &[NetworkUser]) -> Vec<NetworkUser> {
    let mut peekable = users.into_iter().peekable();
    let mut filtered_users = Vec::new();
    while let Some(u) = peekable.next() {
        let Some(&next_user) = peekable.peek() else {
            break;
        };

        if next_user.first_name.len() < 6 {
            continue;
        }
        filtered_users.push(u.clone());
        if filtered_users.len() >= 2 {
            break;
        }
    }

    filtered_users
}
// region:    --- Combine iterators and for loops

// endregion: --- Combine iterators and for loops

// region:    --- Boilerplate

#[derive(Debug, Clone)]
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

// endregion: --- Boilerplate

#[test]
fn handle_signatures() {
    let users = receive_users();

    let iter = users.into_iter().filter(|u| u.first_name.len() > 4);
    const START_WITH: char = 'c';
    let starting_with = name_starts_with_c(START_WITH, iter);
    let filtered: Vec<_> = starting_with.collect();
    println!("{filtered:#?}");
}

#[test]
fn test_filter_contrived() {
    let users = receive_users();
    let users = filter_contrived(users.as_slice());
    println!("{users:#?}");
}
