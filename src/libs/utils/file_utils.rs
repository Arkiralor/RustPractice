use std::fs::File;
use std::path::Path;

pub fn read_data(file_path: &str) -> Result<File, String> {
    //! Read the json file and return it's contents.
    let panic_msg: String = format!("File '{}' not found.", file_path);

    let normalised_file_path = match Path::new(file_path).canonicalize() {
        Ok(val) => val,
        Err(s) => panic!("{}", s),
    };

    let file_obj: File = File::open(normalised_file_path).expect("Error opening file");
    return Ok(file_obj);
}
