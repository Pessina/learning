type Alias = i32;

type CustomResult = Result<i32, i32>;

fn eq<T: Eq + ?Sized>(s: &T, v: &T) -> bool {
    s == v
}

fn main() {
    let a: Alias = 5;
    let b: CustomResult = Ok(5);

    println!("Hello, world!");

    let a = eq("Felipe", "sss");
    println!("{}", a)
}
