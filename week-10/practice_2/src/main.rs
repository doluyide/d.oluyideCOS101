fn main(){
    let v = vec![10, 20, 30];
    let v2 = v; // v moves to v2, v is dead

    display(v2); // v2 moves to display, v2 is dead
    println!("In main {:?}", v2); // ERROR - borrow of moved value
}

fn display(v:Vec<i32>){
    println!("inside display {:?}", v);
}