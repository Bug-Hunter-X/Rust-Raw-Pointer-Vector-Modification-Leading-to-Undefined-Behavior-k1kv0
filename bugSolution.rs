fn main() {
    let mut v = vec![1, 2, 3];
    let index = 0;
    v[index] = 10; // Safe and idiomatic way to modify the vector element 
    println!("v: {:?}", v);
}