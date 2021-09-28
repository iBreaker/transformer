use std::borrow::Cow;

pub trait TransformerTrait {
    fn transform(_: String)->String;
}

pub struct Base64Decode {}
impl TransformerTrait for Base64Decode{
    fn transform(input: String)->String{
        match base64::decode(input) {
            Ok(v) => {
                String::from_utf8(v)
                    .unwrap_or_else(|e|e.to_string())
            }
            Err(e) => e.to_string(),
        }
    }
}

pub struct Base64Encode {}
impl TransformerTrait for Base64Encode{
    fn transform(input: String)->String{
        return base64::encode(input.clone())
    }
}

pub struct MD5 {}
impl TransformerTrait for MD5{
    fn transform(input: String)->String{
        format!("{:x}", md5::compute(input))
    }
}

pub struct URLEncode {}
impl TransformerTrait for URLEncode{
    fn transform(input: String)->String{
        urlencoding::encode(input.as_str()).to_string()
    }
}

pub struct URLDecode {}
impl TransformerTrait for URLDecode{
    fn transform(input: String)->String{
        urlencoding::decode(input.as_str())
            .unwrap_or_else(|e| Cow::from(e.to_string())).to_string()
    }
}
