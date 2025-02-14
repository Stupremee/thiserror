use thiserror_no_std::Error;

#[derive(Error, Debug)]
pub enum ErrorEnum {
    Confusing {
        #[source]
        a: std::io::Error,
        #[source]
        b: anyhow::Error,
    },
}

fn main() {}
