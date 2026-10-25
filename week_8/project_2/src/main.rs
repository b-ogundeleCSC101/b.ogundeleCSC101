use std::io::Write;

fn main() {
    let header = vec!["Student Name", "Matric No.", "Department", "Level"];
    let student_name = vec!["Oluchi Mordi", "Adams Aliyu", "Shania Bolade", "Adekunle Gold", "Blanca Edemoh"];
    let matric_number = vec!["ACC10211111", "ECO10110101", "CSC10328828", "EEE11020202", "MEE10202001"];
    let department = vec!["Accounting", "Economics", "Computer", "Electrical", "Mechanical"];
    let level = vec!["300", "100", "200", "200", "100"];

    let mut file = std::fs::File::create("PAU-SMIS.csv").unwrap();
    file.write_all("PAU Student Management Information System\n".as_bytes()).unwrap();
    
    let _row_0 = format!("{},{},{},{}\n", header[0], header[1], header[2], header[3]);
    let row_1 = format!("{},{},{},{}\n", student_name[0], matric_number[0], department[0], level[0]);
    let row_2 = format!("{},{},{},{}\n", student_name[1], matric_number[1], department[1], level[1]);
    let row_3 = format!("{},{},{},{}\n", student_name[2], matric_number[2], department[2], level[2]);
    let row_4 = format!("{},{},{},{}\n", student_name[3], matric_number[3], department[3], level[3]);
    let row_5 = format!("{},{},{},{}\n", student_name[4], matric_number[4], department[4], level[4]);

    file.write_all(_row_0.as_bytes()).unwrap();
    file.write_all(row_1.as_bytes()).unwrap();
    file.write_all(row_2.as_bytes()).unwrap();
    file.write_all(row_3.as_bytes()).unwrap();
    file.write_all(row_4.as_bytes()).unwrap();
    file.write_all(row_5.as_bytes()).unwrap();


    println!("PAU SMIS table created");
}
