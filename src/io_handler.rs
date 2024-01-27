use std::fs;

pub fn write_to_file(file_content: String, name: String) -> Result<(), std::io::Error> {
    fs::write(name, file_content).unwrap();
    Ok(())
}

pub fn print_file(file_content: String) -> Result<(), std::io::Error> {
    println!("{}", file_content);
    Ok(())
}

pub fn read_file(file_name: String) -> Result<String, std::io::Error> {
    let file_content =
        fs::read_to_string(file_name).expect("Something went wrong reading the file");
    Ok(file_content)
}
