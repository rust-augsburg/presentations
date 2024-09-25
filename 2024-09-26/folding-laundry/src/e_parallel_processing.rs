//! Problem: We want to read 5 big files in dedicated threads and deal with their results.
//! We spawn a dedicated thread to process each file and then deal with the join handles of
//! those threads.
//! 
//! This task is very different to all prior tasks mainly because we are dealing with side effects here.
//! 
//! Hint: In a real world example, you would be very interested in checking out the 
//! [`rayon`](https://github.com/rayon-rs/rayon) crate.

// region:    --- Boilerplate
struct File {
    contents: String,
    corrupt: bool,
}

impl File {
    /// Mock reading a file from disk/the network.
    pub fn to_string(self) -> Result<String, Error> {
        if !self.corrupt {
            return Ok(self.contents);
        }

        Err(Error::CorruptFile {
            corrupt_content: self.contents,
        })
    }
}

fn mock_files() -> Vec<File> {
    vec![
        File {
            contents: "abcdefg".to_string(),
            corrupt: false,
        },
        File {
            contents: "1234567".to_string(),
            corrupt: true,
        },
        File {
            contents: "zzzzzzz".to_string(),
            corrupt: false,
        },
        File {
            contents: "uuuuuuu".to_string(),
            corrupt: false,
        },
        File {
            contents: "9999999".to_string(),
            corrupt: true,
        },
    ]
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Content is corrupted: {corrupt_content}.")]
    CorruptFile { corrupt_content: String },
    #[error("Could not join worker thread!")]
    JoinFailed,
}
// endregion: --- Boilerplate

fn classic_read_files(files: Vec<File>) -> Result<String, Error> {
    let mut join_handles = Vec::new();
    for file in files {
        join_handles.push(std::thread::spawn(|| file.to_string()));
    }

    let mut contents = "File contents: ".to_string();
    for handle in join_handles {
        let s = handle.join().map_err(|_| Error::JoinFailed)??;
        contents = format!("{}--{}", contents, s);
    }
    Ok(contents)
}

/// Converts all files within `files` into strings in parallel.
///
/// Returns an error if any of the files are "corrupt".
/// # Implementation details
/// Spawns a thread for each file within `files` and calls `to_string` within those threads.
fn read_files(files: Vec<File>) -> Result<String, Error> {
    let handles: Vec<_> = files
        .into_iter()
        .map(|f| std::thread::spawn(|| f.to_string()))
        .collect();
    handles
        .into_iter()
        .map(|handle| handle.join())
        .flat_map(|nested_result| match nested_result {
            Ok(inner) => Ok(inner),
            Err(_) => Err(Error::JoinFailed),
        })
        .try_fold("File contents: ".to_string(), |acc, file_result| {
            file_result.map(|s| format!("{}--{}", acc, s))
        })
}

fn ignore_errors_read_files(files: Vec<File>) -> String {
    let handles: Vec<_> = files
        .into_iter()
        .map(|f| std::thread::spawn(|| f.to_string()))
        .collect();
    handles
        .into_iter()
        .filter_map(|handle| handle.join().ok().and_then(|result| result.ok()))
        .reduce(|concat, file_content| format!("{}--{}", concat, file_content))
        .unwrap_or("no files were processed successfully!".to_string())
}

// region:    --- Testing
#[test]
fn test_solutions() {
    let files = mock_files();
    let s = read_files(files);
    println!("RESULT: {s:#?}");

    let files = mock_files();
    let s = classic_read_files(files);
    println!("RESULT: {s:#?}");

    let files = mock_files();
    let s = ignore_errors_read_files(files);
    println!("RESULT: {s:}");
}

// endregion: --- Testing
