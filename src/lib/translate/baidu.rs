

use reqwest::{self, blocking};
use serde::{Serialize};

const BAIDU_URL : &str = "https://fanyi-api.baidu.com/api/trans/vip/translate";
const BAIDU_APPID : &str = "";
const BAIDU_KEY : &str = "";

pub fn translatef(word :&str) -> String{
    let f = Form::new(word, "auto", "auto");
    let resp = send(&f).expect("error").text().expect("err");
    print!("{:?}", resp);
    resp

}

#[derive(Serialize, Debug)]
struct Form {
    q: String,
    from: String,
    to: String,
    appid: String,
    salt: String,
    sign: String,
}

impl Form {
    fn new(q: &str, from: &str,to: &str) -> Self{
        let mut sf = Self { 
            q: q.to_string(), 
            from: from.to_string(),
            to: to.to_string(), 
            appid: BAIDU_APPID.to_string(), 
            salt: BAIDU_APPID.to_string(), 
            sign: "".to_string(),
        };
        sf.s();
    
        println!("{:?}", sf);
        sf
    }

    fn s(& mut self){
        let key = BAIDU_KEY.to_string();
        let tmp = self.appid.clone() + &self.q + &self.salt + &key;
        self.sign = format!("{:x}", md5::compute(tmp.as_bytes()));
    }
}

fn send<T: serde::Serialize>(form :&T) -> Result<blocking::Response, reqwest::Error>{
     blocking::Client::new()
    .post(BAIDU_URL).form(form).send()
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_translatef(){
        translatef("test");
        ()
    }
}