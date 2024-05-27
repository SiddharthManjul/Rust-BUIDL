#[allow(dead_code, unused_variables)]

async fn async_call(url: &str) -> Result<serde_json::Value, reqwest::Error> {

    let response: serde_json::Value = reqwest::get(url)
    .await?
    .json::<serde_json::Value>()
    .await?;

    Ok(response)
}

#[cfg(test)]

#[allow(dead_code, unused_variables, unused_imports)]
mod tests {
    use std::result;

    use super::*;

    #[tokio::test]

    async fn tests_calls_async_fn() {
        let api_url: &str = "https://cat-fact.herokuapp.com/facts/";
        let result: Result<serde_json::Value, reqwest::Error> = async_call(&api_url).await;
        match result {
            Ok(r) => {
                dbg!(r);
            }
            Err(_) => {
                panic!("Failed to make request!!");
            }
        };
    }
}