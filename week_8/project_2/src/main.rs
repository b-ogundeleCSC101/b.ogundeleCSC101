use std::io::Write;

fn main() {
    let student_name = vec!["Oluchi Mordi", "Adams Aliyu", "Shania Bolade", "Adekunle Gold", "Blanca Edemoh"];
    let matric_number = vec!["ACC10211111", "ECO10110101", "CSC10328828", "EEE11020202", "MEE10202001"];
    let department = vec!["Accounting", "Economics", "Computer", "Electrical", "Mechanical"];
    let level = vec!["300", "100", "200", "200", "100"];

    let mut file = std::fs::File::create("PAU-SMIS.csv").unwrap();
    file.write_all(b"PAU Student Management Information System\n");
    file.write_all(student_name.as_bytes()).unwrap();
    file.write_all(matric_number.as_bytes()).unwrap();
    file.write_all(department.as_bytes()).unwrap();
    file.write_all(level.as_bytes()).unwrap();


    println!("PAU SMIS table created");
}
