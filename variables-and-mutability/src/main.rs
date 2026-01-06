fn main() {
    let coffee_price = 5.99;
    {
        //this is not shadowing because it is a different scope
        let coffee_price = 6.99;
        println!("The price is {coffee_price}");
        // println!("The price is {coffee_price}");
        let cookie_price = 1.99;
        println!("The {cookie_price}");
    }
    println!("The price is {coffee_price}");
// println!("The {cookie_price}");

}

