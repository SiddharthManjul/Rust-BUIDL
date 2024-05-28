use std::io::{Error, ErrorKind};

#[allow(dead_code, unused_variables)]

async fn async_call(url: &str) -> Result<serde_json::Value, Error> {

    let response: reqwest::Response = reqwest::get(url)
    .await
    .map_err(|_| Error::new(ErrorKind::Other, "Could not retreive!"))?;
    // .expect("Could not retreive!"); --- Another way

    let json_response: serde_json::Value = response
    .json::<serde_json::Value>()
    .await
    .map_err(|_| Error::new(ErrorKind::Other, "Could not decode JSON!"))?;

    Ok(json_response)
}

#[cfg(test)]

#[allow(dead_code, unused_variables, unused_imports)]
mod tests {
    use std::result;

    use super::*;

    #[tokio::test]

    async fn tests_calls_async_fn_with_std_error() {
        let api_url: &str = "https://cat-fact.herokuapp.com/facts/";
        let result: Result<serde_json::Value, std::io::Error> = async_call(&api_url).await;
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