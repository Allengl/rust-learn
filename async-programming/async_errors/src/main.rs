use std::time::Duration;

use tokio::time::sleep;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut tasks = Vec::new();
    for id in 1..=5 {
        tasks.push(fetch_user(id));
    }
    let results = futures::future::join_all(tasks).await;
    println!("{:?}", results);

    // let final_result = results.into_iter().collect::<anyhow::Result<Vec<String>>>();
    // let user = final_result?;
    // println!("{:?}", user);
    let mut errors = Vec::new();
    let ok_users: Vec<String> = results
        .into_iter()
        .filter_map(|r| r.map_err(|e| errors.push(e)).ok())
        .collect();
    println!("ok_users: {:?}", ok_users);
    println!("errors: {:?}", errors);

    Ok(())
}

async fn fetch_user(id: u32) -> anyhow::Result<String> {
    if id == 3 {
        anyhow::bail!("User {id} not does not exist");
    }
    sleep(Duration::from_secs(1)).await;
    Ok(format!("User {}", id))
}
