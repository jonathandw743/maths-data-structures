use maths_data_structures::*;

fn main() {
    list::List::<usize>::showoff();
    println!("----------");
    fbtree::FBTree::<usize>::showoff();
    println!("----------");
    btree::BTree::<usize>::showoff();
}
