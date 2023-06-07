pub mod file {
    use std::{fs, path::Path};

    pub fn read_file(path: &String) -> String {
        let filepath = Path::new(path);
        let display = filepath.display();
        let open_file_result = fs::read_to_string(filepath);

        match open_file_result {
            Ok(file) => file,
            Err(error) => panic!("Couldn't open file {} because of {:?}", display, error),
        }
    }

    pub fn write_file(path: String, markdown: String) {
        let filepath = Path::new(&path);
        let display = filepath.display();
        let write_file_result = fs::write(filepath, markdown);

        match write_file_result {
            Ok(_) => {}
            Err(error) => panic!("Couldn't write file {} because of {:?}", display, error),
        }
    }
}
