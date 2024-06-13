use kuchiki::parse_html;
use kuchiki::traits::*;
use kuchiki::NodeRef;

pub fn process_html(input: &str, process_text: Box<dyn Fn(&str) -> String>) -> String {
    let document = kuchiki::parse_html().one(input);

    // Collect all text nodes
    let text_nodes: Vec<NodeRef> = document
        .select("body")
        .unwrap()
        .flat_map(|n| n.as_node().descendants())
        .filter(|n| n.as_text().is_some())
        .map(|n| n.clone())
        .collect();

    // Process each text node
    for text_node in text_nodes {
        if let Some(text) = text_node.as_text() {
            let original_text = text.borrow().to_string();
            let processed_text = process_text(&original_text);

            if processed_text != original_text {
                if processed_text.contains('<') && processed_text.contains('>') {
                    let fragment = parse_html().one(format!("<div>{}</div>", processed_text));
                    let fragment_children: Vec<NodeRef> = fragment
                        .select("div")
                        .unwrap()
                        .next()
                        .unwrap()
                        .as_node()
                        .children()
                        .collect();

                    for child in fragment_children {
                        text_node.insert_before(child);
                    }
                    text_node.detach();
                } else {
                    *text.borrow_mut() = processed_text;
                }
            }
        }
    }

    let mut output = vec![];
    document.serialize(&mut output).unwrap();
    String::from_utf8(output).unwrap()
}

pub fn read_html_content(path: &str) -> String {
    let content = std::fs::read_to_string(path).unwrap();
    return content;
}

#[allow(dead_code)]
pub fn main() {
    let input_html = read_html_content("resources/sample.xml");

    let process_text = Box::new(move |input: &str| {
        if input.trim().is_empty() {
            return input.to_string();
        }

        let res = input.replace("fear", "touch");
        return res;
    });

    let processed_html = process_html(input_html.as_str(), process_text);
    println!("{}", processed_html);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_html() {
        let process_text = Box::new(move |input: &str| {
            if input.trim().is_empty() {
                return input.to_string();
            }

            input.replace("world", "xiaoxiao")
        });
        let input_html = r#"<!DOCTYPE html><html lang="en"><head><meta charset="UTF-8"><title>hello world</title></head><body><div>hello <span style="color:red">world</span><img src="title.jpg"></div></body></html>"#;
        let processed_html = process_html(input_html, process_text);
        let expect = r#"<!DOCTYPE html><html lang="en"><head><meta charset="UTF-8"><title>hello world</title></head><body><div>hello <span style="color:red">xiaoxiao</span><img src="title.jpg"></div></body></html>"#;
        assert_eq!(expect, processed_html);
    }
}
