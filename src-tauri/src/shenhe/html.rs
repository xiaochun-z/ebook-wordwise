use kuchiki::{traits::TendrilSink, NodeRef};
use tauri::Runtime;

use super::types::ProgressReporter;

pub fn process_html<R: Runtime>(
    input: &str,
    process_text_fn: &(dyn Fn(&str) -> String),
    reporter: Option<&ProgressReporter<R>>,
) -> Result<String, String> {
    let document = kuchiki::parse_html().one(input);

    let body = document
        .select_first("body")
        .map_err(|_| "Failed to select body")?;

    let total = body.as_node().descendants().count() as f32;
    let mut i = 0.0;

    // Process each text node incrementally
    for descendant in body.as_node().descendants() {
        i += 1.0;
        if let Some(text_node) = descendant.as_text() {
            if let Some(r) = reporter {
                let current_progress = i / total;
                r.report(current_progress);
            }

            let original_text = text_node.borrow().to_string().trim().to_string();
            if original_text.is_empty() {
                continue;
            }

            println!("original_text: {}", original_text);
            let processed_text = process_text_fn(&original_text);

            if processed_text != original_text {
                if processed_text.contains('<') && processed_text.contains('>') {
                    // Parse the processed text as HTML fragment
                    let fragment =
                        kuchiki::parse_html().one(format!("<div>{}</div>", processed_text));
                    let fragment_children: Vec<NodeRef> = fragment
                        .select_first("div")
                        .unwrap()
                        .as_node()
                        .children()
                        .collect();

                    for child in fragment_children {
                        descendant.insert_before(child);
                    }
                    //descendant.detach();
                    *text_node.borrow_mut() = "".to_string();
                } else {
                    *text_node.borrow_mut() = processed_text;
                }
            }
        }
    }
    let mut output = vec![];
    document
        .serialize(&mut output)
        .map_err(|err| err.to_string())?;
    std::str::from_utf8(&output)
        .map(|res| res.to_string())
        .map_err(|err| err.to_string())
}

pub fn read_html_content(path: &str) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|err| err.to_string())
}

pub fn write_html_content(path: &str, contents: &str) -> Result<(), String> {
    std::fs::write(path, contents).map_err(|err| err.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tauri::Wry;
    #[test]
    fn test_process_html() {
        let process_text = Box::new(move |input: &str| {
            if input.trim().is_empty() {
                return input.to_string();
            }

            input.replace("world", "xiaoxiao").replace("fear", "fare")
        });
        let data = [
            (
                r#"<!DOCTYPE html><html lang="en"><head><meta charset="UTF-8"><title>hello world</title></head><body><div>hello <span style="color:red">world</span><img src="title.jpg"></div></body></html>"#,
                r#"<!DOCTYPE html><html lang="en"><head><meta charset="UTF-8"><title>hello world</title></head><body><div>hello <span style="color:red">xiaoxiao</span><img src="title.jpg"></div></body></html>"#,
            ),
            (
                r#"<html><head></head><body class="calibre"><p class="calibre_1">for <span style="color:red">fear</span> I</p></body></html>"#,
                r#"<html><head></head><body class="calibre"><p class="calibre_1">for <span style="color:red">fare</span> I</p></body></html>"#,
            ),
        ];

        for (input, output) in data {
            let processed_html = process_html::<Wry>(input, process_text.as_ref(), None).unwrap();
            assert_eq!(processed_html, output);
        }
    }
}
