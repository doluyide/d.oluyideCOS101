fn main(){
    let v = vec![15, 25, 35, 45, 55];
    print_vector(v); // v moved
    println!("{}", v[0]); // ERROR - v is gone
}

fn print_vector(x:Vec<i32>){
    println!("Inside print_vector {:?}", x);
}