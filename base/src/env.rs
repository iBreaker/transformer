//
//  @auther: Breaker
//  @create: 2021-10-03 14:28
//

use std::env;

pub fn get_env(key:String, default: String) -> String{
    match env::var(key) {
        Ok(val) => return val,
        Err(_e) => return default,
    }
}