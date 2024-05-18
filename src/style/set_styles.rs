use web_sys::Document;

use crate::get_document;

pub(crate) const INITIAL_HEAD: &str = r#"
<link data-trunk rel="copy-dir" href="assets/">
<link rel="icon" type="image/x-icon" href="/assets/images/joystick.svg">
<link rel="stylesheet" href="/assets/css/app_style.css">
<link rel="stylesheet" href="/assets/css/header_style.css"/>
<meta charset="utf-8" />
<meta name="viewport" content="width=device-width, initial-scale=1.0" />
<title>Lumen</title>
"#;

pub fn set_initial_style() {
    let document: Document = get_document();

    document
        .get_element_by_id("head")
        .unwrap()
        .set_inner_html(INITIAL_HEAD);
}

pub fn set_component_style(css: Vec<String>) {
    let document: Document = get_document();

    let mut current_style: String = INITIAL_HEAD.to_string();

    for file in css {
        let component_style = format!(r#"<link rel="stylesheet" href="/assets/css/{}">"#, file);
        current_style.push_str(&component_style);
    }

    document
        .get_element_by_id("head")
        .unwrap()
        .set_inner_html(&current_style);
}
