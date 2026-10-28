struct Electronics {
    Brand:String,
    Quantity:u32,
    Price:u32
}
impl Electronics{
    fn total(&self)->u32 {
        3 * self.Price
    }
}
fn main() {
   let brand1 = Electronics {
    Brand:String::from("HP"),
    Quantity:10,
    Price: 650_000
   };
   let brand2 = Electronics {
    Brand:String::from("IBM"),
    Quantity:6,
    Price:755_000
   };
   let brand3 = Electronics {
    Brand:String::from("Toshiba"),
    Quantity:10,
    Price:550_000
   };
   let brand4 = Electronics {
    Brand:String::from("Dell"),
    Quantity:4,
    Price:850_000
   };
   display(brand1);
   display(brand2);
   display(brand3);
   display(brand4);
}
fn display(brand:Electronics) {
    println!("**---------Welcome to Alaba Electronics---------**\n");
   println!("Here are the brands and prices we offer");
   println!("Brand\t\t Quantity \t\t Price each");
   println!("{}\t\t{}\t\tN{}",brand.Brand, brand.Quantity, brand.Price);
   println!("Total price of 3 laptops of each brand: {}", brand.total());
    }


