use kuchiki::parse_html;
use kuchiki::traits::*;
use kuchiki::NodeRef;
use tauri::Runtime;

pub fn process_html<R: Runtime>(
    input: &str,
    process_text_fn: &(dyn Fn(&str) -> String),
    progress_fn: Option<&dyn Fn(f32, Option<&tauri::Window<R>>)>,
    tauri_window: Option<&tauri::Window<R>>,
) -> Result<String, String> {
    let document = kuchiki::parse_html().one(input);

    // Collect all text nodes
    let text_nodes: Vec<NodeRef> = document
        .select("body")
        .unwrap()
        .flat_map(|n| n.as_node().descendants())
        .filter(|n| n.as_text().is_some())
        .map(|n| n.clone())
        .collect();

    let total: f32 = text_nodes.len() as f32;
    let mut i: f32 = 0.0;
    // Process each text node
    for text_node in text_nodes {
        if let Some(progress) = progress_fn {
            i += 1.0;
            let current_progress = 0.2 + (0.9 - 0.2) * i / total; // map 0-100 to 20-90
            progress(current_progress, tauri_window);
        }

        if let Some(text) = text_node.as_text() {
            let original_text = text.borrow().to_string();
            let processed_text = process_text_fn(&original_text);

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
    let _ = document
        .serialize(&mut output)
        .map_err(|err| err.to_string());
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
            let processed_html = process_html::<Wry>(input, process_text.as_ref(), None, None);
            assert_eq!(processed_html.unwrap(), output);
        }
    }
}
