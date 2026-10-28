fn main() {
    let v = vec![20,40,60,80];
    let v2 = v;
    let v2_return = display(v2);
    println!("In main {:?}", v);
}
 fn main(v:Vec<i32>)->Vec<i32> {
    println!("Inside display {:?}", v);
    return v;
 }
