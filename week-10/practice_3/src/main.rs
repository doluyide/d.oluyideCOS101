fn main(){
    let v = vec![20, 40, 60, 80];
    let v2 = v;
    let v2_return = display(v2); // we get it back
    println!("In main {:?}", v2_return); // now it works
}

fn display(v:Vec<i32>)->Vec<i32>{
    println!("inside display {:?}", v);
    return v; // returning same vector
}
