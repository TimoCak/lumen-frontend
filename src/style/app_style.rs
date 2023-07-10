use stylist::{StyleSource, css};

pub fn get_app_style() -> StyleSource {
    css!(
        r#"

        * { margin: 0 !important; }
        
        "#
    )
}