// Otsu picks the cut that best splits dark cells from light ones.
// cargo run -p mrly-tensor --example threshold
use mrly_tensor::Tensor;

fn main() {
    // Two clusters: dark values near 20 and light values near 200.
    let data = vec![15, 20, 25, 18, 22, 195, 200, 205, 198, 210];
    let t = Tensor::of(data.clone(), vec![2, 5]).unwrap();

    let cut = t.otsu_threshold();
    println!("values    {data:?}");
    // The cut is the highest value of the dark group.
    println!("otsu cut  {cut}");

    // binarize(threshold) gives 1 for cells at or above the threshold.
    println!("binarize  {:?}", t.binarize(128).bytes().unwrap());
    // binarize_otsu uses cut + 1 for you, so the dark group lands on 0.
    let bits = t.binarize_otsu();
    println!("otsu bits {:?}", bits.bytes().unwrap());
    println!("ones {}, zeros {}", bits.count(1), bits.count(0));
}
