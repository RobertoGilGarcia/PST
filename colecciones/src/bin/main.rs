mod vec_demo;
mod vecdeque_demo;
mod hashmap_demo;
mod btreemap_demo;
mod btreeset_demo;
mod hashset_demo;
mod binaryheap_demo;


fn main() {
    println!("Vec demo:");
    vec_demo::run();


    println!("\nVecDeque demo:");
    vecdeque_demo::run();


    println!("\nHashMap demo:");
    hashmap_demo::run();


    println!("\nBTreeMap demo:");
    btreemap_demo::run();


    println!("\nBTreeSet demo:");
    btreeset_demo::run();


    println!("\nHashSet demo:");
    hashset_demo::run();


    println!("\nBinaryHeap demo:");
    binaryheap_demo::run();
}
