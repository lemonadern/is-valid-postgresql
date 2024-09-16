use std::fs::read_to_string;

use pg_query::Error;

fn main() -> Result<(), Error> {
    let src_file = std::env::args()
        .nth(1)
        .expect("Arguments error: please specify a file.");
    let src = read_to_string(&src_file).unwrap();

    let result = pg_query::parse(&src);

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
