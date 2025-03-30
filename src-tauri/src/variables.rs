#[test]
fn variables() {
    // creation
    let _a = 5;
    let _b: i32 = 10;
    // mutability
    let mut _c = 2;
    _c = 4;
    // shadowing
    let _c = "10";
    // scop
    {
        let d = "inner scop";
        println!("d : {}", d);
    }
    // println!("{}",d) // d is not valid here
}



