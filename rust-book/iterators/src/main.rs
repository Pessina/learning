fn main() {
    let vec = vec![1, 2, 3];
    let vec_iter = vec.iter();
    let mut vec2 = Vec::new();

    for i in vec_iter {
        vec2.push(i);
    }

    let vec_iter = vec.iter();
    let total: i32 = vec_iter.sum();

    println!("{total}");

    let vec_iter = vec.iter();
    let v2: Vec<_> = vec_iter.map(|x| x + 1).collect();
    println!("{:?}", v2)
}
