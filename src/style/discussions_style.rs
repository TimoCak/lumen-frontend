use stylist::{StyleSource, css};

pub fn get_discussions_style() -> StyleSource {
    css!(
        r#"
            .discussions-container {
                text-align: center;
                
                color: white;
            }
        "#
    )
}