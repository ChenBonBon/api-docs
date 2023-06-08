pub mod markdown {
    use crate::parser::parser::MarkdownTag;

    pub fn build_markdown(parsed_tags: Vec<MarkdownTag>) -> String {
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
}
