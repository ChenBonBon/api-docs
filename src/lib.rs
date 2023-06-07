use file::file::{read_file, write_file};
use glob::glob;

use parser::parser::{parse_tags, MarkdownTag};
use utils::utils::convert_code_to_input;

use std::{
    fs,
    path::Path,
    sync::{Arc, Mutex},
    thread,
};

pub mod file;
pub mod parser;
pub mod utils;

fn build_markdown(parsed_tags: Vec<MarkdownTag>) -> String {
    let mut _parsed_tags = parsed_tags.clone();
    let mut markdown = "".to_string();
    let mut sorted_parsed_tags: Vec<MarkdownTag> = Vec::new();
    let orders = vec![
        "function".to_string(),
        "description".to_string(),
        "since".to_string(),
        "category".to_string(),
        "params".to_string(),
        "returns".to_string(),
        "example".to_string(),
    ];

    for name in orders {
        let tag_index = _parsed_tags.iter().position(|tag| tag.r#type == *name);
        match tag_index {
            Some(index) => {
                let tag_result = _parsed_tags.get(index);
                match tag_result {
                    Some(tag) => {
                        sorted_parsed_tags.push(tag.clone());
                        _parsed_tags.remove(index);
                    }
                    None => {}
                }
            }
            None => {}
        }
    }

    sorted_parsed_tags = [sorted_parsed_tags, _parsed_tags].concat();

    for tag in sorted_parsed_tags {
        markdown = format!("{}{}\n", markdown, tag.value);
    }

    markdown
}

fn get_file_paths(input_dir: String) -> Vec<String> {
    let mut file_paths = vec![];

    for entry in glob(&format!("{input_dir}/**/*.js",)).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                file_paths.push(path.to_str().unwrap().to_string());
            }
            Err(_) => {}
        }
    }

    file_paths
}

pub fn parse(input_dir: String, output_dir: String) {
    let file_paths = get_file_paths(input_dir.clone());
    let outputs = Arc::new(Mutex::new(vec![]));
    let mut handles = vec![];

    for file_path in file_paths {
        let out_file_path = file_path.replace(&input_dir, &output_dir);
        let outputs = Arc::clone(&outputs);
        let handle = thread::spawn(move || {
            let file = read_file(&file_path);
            let input = convert_code_to_input(&file);
            let parse_result = jsdoc::parse(input);
            match parse_result {
                Ok(result) => {
                    let doc = result.1;
                    let parsed_tags = parse_tags(&doc.tags);
                    let markdown = build_markdown(parsed_tags);
                    let mut outputs = outputs.lock().unwrap();
                    outputs.push([out_file_path, markdown]);
                }
                Err(error) => panic!("Couldn't parse input {:?} because of {:?}", input, error),
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    for output in outputs.lock().unwrap().to_vec() {
        let output_file_path = &output[0];
        let markdown = &output[1];
        let parent_result = Path::new(output_file_path).parent();
        let file_name = Path::new(output_file_path).file_stem().unwrap();

        match parent_result {
            Some(parent) => {
                if !parent.exists() {
                    let display = parent.display();
                    match fs::create_dir_all(parent) {
                        Ok(_) => {}
                        Err(error) => {
                            panic!("Couldn't create dir {} because of {:?}", display, error)
                        }
                    }
                }

                write_file(
                    format!(
                        "{}/{}.md",
                        parent.to_str().unwrap(),
                        file_name.to_str().unwrap()
                    ),
                    markdown.to_string(),
                );
            }
            None => write_file(
                format!("{}/{}.md", output_dir, file_name.to_str().unwrap()),
                markdown.to_string(),
            ),
        }
    }
}
