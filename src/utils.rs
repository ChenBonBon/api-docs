pub mod utils {
    use glob::glob;
    use jsdoc::Input;
    use swc_common::BytePos;

    pub fn convert_code_to_input(code: &String) -> Input {
        Input::new(BytePos::DUMMY, BytePos::SYNTHESIZED, &code)
    }

    pub fn get_file_paths(input_dir: &String, extension: String) -> Vec<String> {
        let mut file_paths = vec![];

        for entry in
            glob(&format!("{input_dir}/**/*.{extension}",)).expect("Failed to read glob pattern")
        {
            match entry {
                Ok(path) => {
                    file_paths.push(path.to_str().unwrap().to_string());
                }
                Err(_) => {}
            }
        }

        file_paths
    }
}
