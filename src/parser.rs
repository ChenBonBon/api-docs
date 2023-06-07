pub mod parser {
    use std::{
        sync::{Arc, Mutex},
        thread, vec,
    };

    use jsdoc::ast::{
        DescriptionTag, ExampleTag, FunctionTag, ParameterTag, ReturnTag, SinceTag, Tag, TagItem,
        UnknownTag,
    };
    use regex::Regex;

    #[derive(Debug, Clone)]
    pub struct MarkdownTag {
        pub r#type: String,
        pub value: String,
    }

    pub fn parse_tags(tags: &Vec<TagItem>) -> Vec<MarkdownTag> {
        let mut parsed_tags: Vec<MarkdownTag> = Vec::new();
        let mut params: Vec<ParameterTag> = Vec::new();
        let parsed_param_tags: MarkdownTag;

        for tag in tags {
            let _tag = tag.clone();
            match _tag.tag {
                Tag::Parameter(param_tag) => {
                    params.push(param_tag);
                }
                _ => {
                    parsed_tags.push(parse_tag(tag));
                }
            }
        }
        parsed_param_tags = parse_param_tags(params);
        parsed_tags.push(parsed_param_tags);

        parsed_tags
    }

    pub fn parse_tag(tag: &TagItem) -> MarkdownTag {
        let _tag = tag.clone();
        let tag_name = _tag.tag_name.value.to_string();
        let tag_item = _tag.tag;

        match tag_item {
            Tag::Since(since_tag) => {
                let since_tag_md = parse_since_tag(since_tag);
                MarkdownTag {
                    r#type: tag_name,
                    value: since_tag_md,
                }
            }
            Tag::Description(description_tag) => {
                let description_tag_md = parse_description_tag(description_tag);
                MarkdownTag {
                    r#type: tag_name,
                    value: description_tag_md,
                }
            }
            Tag::Function(function_tag) => {
                let function_tag_md = parse_function_tag(function_tag);
                MarkdownTag {
                    r#type: tag_name,
                    value: function_tag_md,
                }
            }
            Tag::Return(return_tag) => {
                let return_tag_md = parse_return_tag(return_tag);
                MarkdownTag {
                    r#type: tag_name,
                    value: return_tag_md,
                }
            }
            Tag::Example(example_tag) => {
                let return_tag_md = parse_example_tag(example_tag);
                MarkdownTag {
                    r#type: tag_name,
                    value: return_tag_md,
                }
            }
            Tag::Unknown(unknown_tag) => {
                if tag_name.eq("category") {
                    let category_tag_md = parse_category_tag(unknown_tag);
                    MarkdownTag {
                        r#type: tag_name,
                        value: category_tag_md,
                    }
                } else {
                    MarkdownTag {
                        r#type: tag_name,
                        value: "".to_string(),
                    }
                }
            }
            _ => MarkdownTag {
                r#type: tag_name,
                value: "".to_string(),
            },
        }
    }

    fn parse_function_tag(function_tag: FunctionTag) -> String {
        match function_tag.name {
            Some(name) => format!("# {}\n", name.value.to_string()),
            None => "".to_string(),
        }
    }

    fn parse_description_tag(description_tag: DescriptionTag) -> String {
        format!("{}\n", description_tag.text.value)
    }

    fn parse_since_tag(since_tag: SinceTag) -> String {
        format!("#### 引入版本\n`{}`\n", since_tag.version.value)
    }

    fn parse_category_tag(category_tag: UnknownTag) -> String {
        format!("#### 分类\n{}\n", category_tag.extras.value.to_string())
    }

    fn parse_param_tags(param_tags: Vec<ParameterTag>) -> MarkdownTag {
        let parsed_tags = Arc::new(Mutex::new(vec![]));
        let mut handles = vec![];
        let mut output =
            "#### 参数\n|  名称   | 类型  | 描述  |\n|  ----  | ----  | ----  |\n".to_string();

        for param_tag in param_tags {
            let parsed_tags = Arc::clone(&parsed_tags);
            let handle = thread::spawn(move || {
                // output = format!("{}{}\n", output, parse_param_tag(param_tag));
                parsed_tags.lock().unwrap().push(parse_param_tag(param_tag));
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        for parsed_tag in parsed_tags.lock().unwrap().to_vec() {
            output = format!("{}{}\n", output, parsed_tag);
        }

        MarkdownTag {
            r#type: "params".to_string(),
            value: output,
        }
    }

    fn parse_param_tag(param_tag: ParameterTag) -> String {
        let name;
        let mut description = "".to_string();
        let splited_desc: Vec<&str> = param_tag.desc.value.split(" ").collect();

        if splited_desc.len() >= 2 {
            name = splited_desc[0].to_string();
            description = splited_desc[1..].join(" ").to_string();
        } else {
            name = splited_desc[0].to_string();
        }

        match param_tag.ty {
            Some(r#type) => format!(
                "|  {}  |  {}  | {}  |",
                name,
                r#type.value.replace("{", "").replace("}", ""),
                description
            ),
            None => "".to_string(),
        }
    }

    fn parse_return_tag(return_tag: ReturnTag) -> String {
        let description = return_tag.description.value;

        match return_tag.ty {
            Some(r#type) => format!(
                "#### 返回值\n| 类型  | 描述  |\n| ----  | ----  |\n|  {}  | {}  |\n",
                r#type.value.replace("{", "").replace("}", ""),
                description
            ),
            None => "".to_string(),
        }
    }

    fn parse_example_tag(example_tag: ExampleTag) -> String {
        let regex = Regex::new(r"\* (.*)\n").unwrap();
        let text = &example_tag.text.value;
        let mut example = "#### 示例\n```ts\n".to_string();
        for cap in regex.captures_iter(text) {
            example = format!("{}  {}\n", example, &cap[1]);
        }

        format!("{}\n```", example)
    }
}
