struct Laptop {
    brand: String,
    price: u32,
}

impl Laptop {
    // method that costs a purchase
    fn cost(&self, qty: u32) -> u32 {
        self.price * qty
    }
}

fn main() {
    // From your table
    let hp = Laptop { brand: String::from("HP"), price: 650_000 };
    let ibm = Laptop { brand: String::from("IBM"), price: 755_000 };
    let toshiba = Laptop { brand: String::from("Toshiba"), price: 550_000 };
    let dell = Laptop { brand: String::from("Dell"), price: 850_000 };

    let header = vec!["Brand", "Price Each", "Qty to buy", "Cost"];

    // Customer buys 3 from each brand
    let qty = 3;

    let hp_cost = hp.cost(qty);
    let ibm_cost = ibm.cost(qty);
    let toshiba_cost = toshiba.cost(qty);
    let dell_cost = dell.cost(qty);

    // Display
    println!("{:<10} {:<12} {:<12} {}", header[0], header[1], header[2], header[3]);
    println!("--------------------------------------------------------");
    println!("{:<10} {:<12} {:<12} {}", hp.brand, hp.price, qty, hp_cost);
    println!("{:<10} {:<12} {:<12} {}", ibm.brand, ibm.price, qty, ibm_cost);
    println!("{:<10} {:<12} {:<12} {}", toshiba.brand, toshiba.price, qty, toshiba_cost);
    println!("{:<10} {:<12} {:<12} {}", dell.brand, dell.price, qty, dell_cost);

    // One total across the four brands
    let total = hp_cost + ibm_cost + toshiba_cost + dell_cost;

    println!("--------------------------------------------------------");
    println!("Total cost for 3 laptops from each brand = N{}", total);
}