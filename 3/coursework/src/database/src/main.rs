use garde::{Valid, Validate};

#[derive(Validate)]
enum Data {
    Struct {
        #[garde(range(min=-10, max=10))]
        field: i32,
    },
    Tuple(#[garde(ascii)] String),
}

fn main() {
    let data = Data::Struct { field: 100 };
    let data = Data::Tuple("🧱".into());
    if let Err(e) = data.validate() {
        println!("invalid data: {e}");
    }
}
