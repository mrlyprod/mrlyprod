// The four error kinds and how each one prints.
// cargo run -p mrly-error --example kinds
use mrly_error::{Error, Result, overflow_error, shape_error, value_error};

fn show(label: &str, result: Result<()>) {
    if let Err(error) = result {
        println!("{label:<9} display: {error}");
        println!("{:<9} debug:   {error:?}", "");
    }
}

fn main() {
    // Each helper returns Err for any T, so it fits wherever a Result is expected.
    show("value", value_error("ratio must be between 0 and 1."));
    show(
        "shape",
        shape_error("data holds 5 elements, shape wants 6."),
    );
    show("overflow", overflow_error("output size overflows usize."));
    // Json wraps a message from the parser and adds a "json: " prefix.
    show("json", Err(Error::Json("expected value at line 1".into())));
}
