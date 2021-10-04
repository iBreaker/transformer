//
//  @auther: Breaker
//  @create: 2021-10-03 14:21
//

use std::collections::HashMap;

use base;
use rand::prelude::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::Trait::TransformerTrait;

const BAIDU_APP_ID: &str = "BAIDU_APP_ID";
const BAIDU_API_APP_KEY: &str = "BAIDU_APP_KEY";
const BAIDU_API_END_POINT: &str = "http://api.fanyi.baidu.com";
const BAIDU_API_PATH: &str = "/api/trans/vip/translate";
const BAIDU_API_URL: &str = combine!(BAIDU_API_END_POINT, BAIDU_API_PATH);

pub struct Baidu {}

impl TransformerTrait for Baidu {
    fn transform(&self, input: String) -> String {
        match self.resp(input) {
            Err(e) => e.to_string(),
            Ok(s) => s,
        }
    }
    fn name(&self) -> String {
        "Baidu".to_string()
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct RequsetBody {
    appid: String,
    q: String,
    from: String,
    to: String,
    salt: u32,
    sign: String,
}

impl RequsetBody {
    fn build_sigin(&mut self, appkey: String) {
        let mut str: String = String::new();
        str.push_str(&self.appid);
        str.push_str(&self.q);
        str.push_str(&self.salt.to_string());
        str.push_str(&appkey);
        self.sign = format!("{:x}", md5::compute(&str));
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct trans_result {
    src: String,
    dst: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResponseBody {
    from: String,
    to: String,
    trans_result: Vec<trans_result>,
}

impl Baidu {
    #[tokio::main]
    async fn resp(&self, src: String) -> Result<String, reqwest::Error> {
        let appid = base::env::get_env(BAIDU_APP_ID.to_string(), "".to_string());
        let appkey = base::env::get_env(BAIDU_API_APP_KEY.to_string(), "".to_string());
        let salt = rand::random::<u32>();
        let mut body = RequsetBody {
            appid,
            q: src,
            from: "en".to_string(),
            to: "zh".to_string(),
            salt,
            sign: "".to_string(),
        };

        body.build_sigin(appkey);

        let client = Client::new();

        let resp = client.post(BAIDU_API_URL).form(&body).send().await?.json::<ResponseBody>().await?;

        if resp.trans_result.len() == 0 {
            return Ok( "".to_string())
        } else {
            return Ok(resp.trans_result[0].dst.clone())
        }
    }
}
