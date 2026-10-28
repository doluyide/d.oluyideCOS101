fn main(){
    let x = vec![100, 200, 300];
    borrow_vector(&x); // passing reference with &

    println!("Printing from main x[0]={}", x[0]); // NOW IT WORKS
}

fn borrow_vector(z:&Vec<i32>){
    println!("Inside print_vector {:?}", z);
}