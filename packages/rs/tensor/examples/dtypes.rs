// A tensor holds u8, u16, u32 or i32 cells, and reads only as its own type.
// cargo run -p mrly-tensor --example dtypes
use mrly_tensor::{Dtype, Tensor};

fn main() {
    let small = Tensor::of(vec![1, 2, 3], vec![3]).unwrap();
    let wide = Tensor::u16(vec![300, 400, 500], vec![3]).unwrap();
    let signed = Tensor::i32(vec![-5, 0, 5], vec![3]).unwrap();

    println!("small  {:?} max {}", small.dtype(), Dtype::U8.max());
    println!("wide   {:?} max {}", wide.dtype(), Dtype::U16.max());
    println!("signed {:?} max {}", signed.dtype(), Dtype::I32.max());

    // Typed views: bytes, u16s, u32s, i32s.
    println!("wide.u16s   {:?}", wide.u16s().unwrap());
    println!("signed.i32s {:?}", signed.i32s().unwrap());
    // at() reads any dtype as i64.
    println!("signed.at(0) {}", signed.at(0));

    // The wrong view is a shape error.
    println!("{}", wide.bytes().unwrap_err());
    // Data must fill the shape exactly.
    println!("{}", Tensor::u8(vec![1, 2, 3], vec![2, 2]).unwrap_err());
}
