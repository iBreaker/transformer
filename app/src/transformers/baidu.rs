//
//  @auther: Breaker
//  @create: 2021-10-03 14:21
//
use base::env;
use super::Trait::TransformerTrait;

const BAIDU_API_APP_ID: &str = "BAIDU_API_APP_ID";
const BAIDU_API_APP_KEY: &str = "BAIDU_APP_KEY";
const BAIDU_API_END_POINT: &str = "http://api.fanyi.baidu.com";
const BAIDU_API_PATH: &str = "/api/trans/vip/translate";
const BAIDU_API_URL: &str = combine !(BAIDU_API_END_POINT , BAIDU_API_PATH);


pub struct Baidu {
    app_id: String,
    app_key: String,
}

pub fn new() -> &'static Baidu {
    &Baidu{
        app_id: env::get_env(BAIDU_API_APP_ID.to_string(), String::from("")),
        app_key: env::get_env(BAIDU_API_APP_ID.to_string(), String::from("")),
    }
}

impl TransformerTrait for Baidu {
    fn transform(&self, input: String) -> String {
        return base64::encode(input.clone());
    }
    fn name(&self) -> String {
        "Baidu".to_string()
    }
}