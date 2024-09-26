use github_client;

#[tokio::main]
async fn main() {
    let _ = github_client::find_all_repos().await;
}
