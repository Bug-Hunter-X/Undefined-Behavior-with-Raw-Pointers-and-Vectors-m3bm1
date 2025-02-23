fn main() {
    let mut v = vec![1, 2, 3];
    // Safe way to modify the vector: use indexing
    v[0] = 4;
    println!( "{:?}", v);

    //Alternative approach using iter_mut()
    for i in v.iter_mut(){
        *i += 1;
    }
    println!( "{:?}", v);
} 