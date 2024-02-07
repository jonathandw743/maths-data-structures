#![allow(dead_code)]

mod btree;
mod fbtree;
mod list;

use btree::*;
use fbtree::*;
use list::*;

fn main() {
    List::<usize>::showoff();
    println!("----------");
    FBTree::<usize>::showoff();
    println!("----------");
    BTree::<usize>::showoff();
}
