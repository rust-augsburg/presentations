//! Problem:    You have a list of URIs and want to download them via network calls.
//!             You like the idea of writing your logic within a `.map` combinator...

// region:    --- Boilerplate

async fn download_file(uri: String) -> String {
    println!("Downloading {uri}...");
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    println!("Downloaded {uri}...");
    format!("Downloaded {uri}!")
}

fn mock_uris() -> Vec<String> {
    vec![
        "1.2.3.4:8080/some.pdf".to_string(),
        "4.3.2.1:8081/some.pdf".to_string(),
        "5.5.5.5:8082/some.pdf".to_string(),
        "2.2.2.2:8083/some.pdf".to_string(),
    ]
}

// endregion: --- Boilerplate

// region:    --- Solutions

async fn classical_sequential_download(files: Vec<String>) -> Vec<String> {
    let mut downloads = Vec::new();
    for uri in files {
        downloads.push(download_file(uri).await);
    }
    downloads
}

async fn classical_parallel_download(files: Vec<String>) -> Vec<String> {
    let mut download_handles = Vec::new();

    for uri in files {
        let handle = tokio::spawn(async { download_file(uri).await });
        download_handles.push(handle);
    }

    let mut downloads = Vec::new();
    for handle in download_handles {
        let download = handle.await.expect("can await task");
        downloads.push(download);
    }
    downloads
}

async fn download_parallel_within_a_chain(files: Vec<String>) -> Vec<String> {
    // build futures
    let futures = files
        .into_iter()
        .map(|uri| async move { download_file(uri).await });
    // poll them (meaning, execute!)
    futures::future::join_all(futures).await
}
// endregion: --- Solutions

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_solutions() {
        let uris = mock_uris();
        let downloads = classical_sequential_download(uris).await;
        println!("--------------------------------");
        let uris = mock_uris();
        let downloads = classical_parallel_download(uris).await;
        println!("--------------------------------");
        let uris = mock_uris();
        let downloads = download_parallel_within_a_chain(uris).await;
    }
}
