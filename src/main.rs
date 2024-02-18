use maths_data_structures::*;

fn main() {
    list::List::<usize>::showoff();
    println!("----------");
    fbtree::FBTree::<usize>::showoff();
    println!("----------");
    btree::BTree::<usize>::showoff();
    println!("----------");
    natural_number::NaturalNumber::showoff();
}
