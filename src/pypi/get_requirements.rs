use std::error::Error;

use futures::future::join_all;

use super::api_data::Response;

pub async fn get_dev_requirements() -> Result<String, Box<dyn Error>> {
    let mut ret_vec: Vec<String> = Vec::new();
    let urls = [
        "https://pypi.org/pypi/flake8/json",
        "https://pypi.org/pypi/black/json",
        "https://pypi.org/pypi/mypy/json",
    ];

    let mut requests = vec![];

    for url in urls {
        requests.push(reqwest::get(url));
    }

    let responses = join_all(requests).await;

    for response in responses {
        let res_json = response?.json::<Response>().await?;

        ret_vec.push(format!(
            "    {}>={}",
            res_json.info.name, res_json.info.version
        ));
    }
    Ok(ret_vec.join("\n"))
}
