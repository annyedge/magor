use octocrab::Octocrab;

use utils::get_env_var;

pub async fn find_all_repos() -> Result<(), Box<dyn std::error::Error>> {
    let token = get_env_var("GITHUB_TOKEN").expect("GITHUB_TOKEN is not set.");

    let octocrab = Octocrab::builder()
        .personal_token(token.to_string())
        .build()?;

    let user = &octocrab.current().user().await?;
    println!("Authenticated as: {}", user.login);

    let repos = octocrab::instance()
        .current()
        .list_repos_for_authenticated_user()
        .per_page(100)
        .page(1)
        .send()
        .await?;

    for repo in &repos.items {
        println!("{}: {:?}", repo.name, repo.html_url);
    }

    Ok(())
}
