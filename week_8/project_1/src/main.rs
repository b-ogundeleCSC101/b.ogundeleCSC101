use std::io::Write;

fn main() {
    let column = "NIGERIAN BREWERY LIMITED PLC\n";
    let header = "Lager,Stout,Non Alcoholic\n";
    let row_1 = "33 Export, Legend, Maltina\n";
    let row_2 = "Desperados, Turbo King, Amstel Malta\n";
    let row_3 = "Goldberg, Williams, Malta Gold\n";
    let row_4 = "Gulder,  , Fayrouz\n";
    let row_5 = "Heineken,  ,  \n";
    let row_6 = "Star,  ,  \n";

    let mut file = std::fs::File::create("Nigerian Breweries PLC.csv").expect("create failed");
    file.write_all(column.as_bytes()).unwrap();
    file.write_all(header.as_bytes()).expect("Failed to create header");
    file.write_all(row_1.as_bytes()).expect("Failed to create row 1");
    file.write_all(row_2.as_bytes()).expect("Failed to create row 2");
    file.write_all(row_3.as_bytes()).expect("Failed to create row 3");
    file.write_all(row_4.as_bytes()).expect("Failed to create row 4");
    file.write_all(row_5.as_bytes()).expect("Failed to create row 5");
    file.write_all(row_6.as_bytes()).expect("Failed to create row 6");

    let _ = file.write_all(b"\nA Brewery supply of drinks created just for you...");
    println!("Table created");
}
