use std::borrow::Cow;

pub trait TransformerTrait {
    fn transform(&self, _: String)->String;
    fn name(&self)->String;
}

pub struct Base64Decode {}
pub type TS = Vec<&'static dyn TransformerTrait>;

pub fn get_transformers() -> TS{
    vec![&Base64Decode{}, &Base64Encode{}, &URLDecode{}, &URLEncode{}, &MD5{}]
}

impl TransformerTrait for Base64Decode{
    fn transform(&self,input: String)->String{
        match base64::decode(input) {
            Ok(v) => {
                String::from_utf8(v)
                    .unwrap_or_else(|e|e.to_string())
            }
            Err(e) => e.to_string(),
        }
    }
    fn name(&self,)->String{
        "Base64Decode".to_string()
    }
}

pub struct Base64Encode {}
impl TransformerTrait for Base64Encode{
    fn transform(&self, input: String)->String{
        return base64::encode(input.clone())
    }
    fn name(&self,)->String{
        "Base64Encode".to_string()
    }
}

pub struct MD5 {}
impl TransformerTrait for MD5{
    fn transform(&self,input: String)->String{
        format!("{:x}", md5::compute(input))
    }
    fn name(&self,)->String{
        "MD5".to_string()
    }
}

pub struct URLEncode {}
impl TransformerTrait for URLEncode{
    fn transform(&self,input: String)->String{
        urlencoding::encode(input.as_str()).to_string()
    }
    fn name(&self,)->String{
        "URLEncode".to_string()
    }
}

pub struct URLDecode {}
impl TransformerTrait for URLDecode{
    fn transform(&self,input: String)->String{
        urlencoding::decode(input.as_str())
            .unwrap_or_else(|e| Cow::from(e.to_string())).to_string()
    }
    fn name(&self,)->String{
        "URLDecode".to_string()
    }
}
