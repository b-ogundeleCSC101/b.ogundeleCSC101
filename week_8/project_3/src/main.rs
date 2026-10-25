use std::io::Write;

fn main() {
    let header: [&str; 3] = ["Commissioners", "Ministry", "Zone"];
    let commissioners = vec!["Aigbogun Alamba Daudu", "Murtala Afeez Bendu", "Okorocha Calistus Ogbona", "Adewale Jimoh Akanbi", "Osazuwa Faith Etieye"];
    let ministry = vec!["Internal Affairs", "Justice", "Defense", "Power & Steel", "Petroleum"];
    let zone = vec!["South West", "North East", "South South", "South West","South East"];

    let mut file = std::fs::File::create("EFCC MERGE.csv").expect("failed to create table");
    file.write_all("EFCC MERGE OF CONVICTED MINISTERS\n".as_bytes()).unwrap();

    let header = format!("{},{},{}\n", header[0], header[1], header[2]);
    let row_1 = format!("{},{},{}\n", commissioners[0], ministry[0], zone[0]);
    let row_2 = format!("{},{},{}\n", commissioners[1], ministry[1], zone[1]);
    let row_3 = format!("{},{},{}\n", commissioners[2], ministry[2], zone[2]);
    let row_4 = format!("{},{},{}\n", commissioners[3], ministry[3], zone[3]);
    let row_5 = format!("{},{},{}\n", commissioners[4], ministry[4], zone[4]);

    file.write_all(header.as_bytes()).unwrap();
    file.write_all(row_1.as_bytes()).unwrap();
    file.write_all(row_2.as_bytes()).unwrap();
    file.write_all(row_3.as_bytes()).unwrap();
    file.write_all(row_4.as_bytes()).unwrap();
    file.write_all(row_5.as_bytes()).unwrap();

    println!("EFCC MERGER COMPLETE");
}
