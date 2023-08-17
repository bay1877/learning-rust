fn main() {
    let v1 = vec![1, 2, 3];
    let mut v2 = Vec::new();
    v2.push("A");
    v2.push("B");
    v2.push("C");

    for i in 0..v1.len(){
        let item = v1[i];
        println!("{}: {}", i, item);
    }
    for item in v2.iter(){
        println!("{}", item);
    }
}
