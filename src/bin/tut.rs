use savacan_lib::math::multiset::MultiSet;

fn main() {
    let mut ms = MultiSet::new();
    ms.insert(5);
    ms.insert(6);
    ms.insert(9);
    ms.insert(5);
    ms.insert(1);
    ms.print();
    println!(
        "max: {}, min: {}",
        ms.get_max().unwrap(),
        ms.get_min().unwrap()
    );
}
