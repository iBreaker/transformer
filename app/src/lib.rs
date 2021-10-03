#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod app;
pub mod transformers;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate base;
