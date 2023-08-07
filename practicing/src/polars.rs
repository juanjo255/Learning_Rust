use polars::{prelude::*, lazy::dsl::dtype_cols};

pub fn using_polars() {
    
    let q = LazyCsvReader::new("./datasets/data.csv")
    .has_header(true)
    .finish().unwrap();

    let df = q.collect();
    println!("{:?}", df);
}