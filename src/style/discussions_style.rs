use stylist::{StyleSource, css};

pub fn get_discussions_style() -> StyleSource {
    css!(
        r#"
            .discussions-container {
                text-align: center;
                margin-top: 2em;
                color: white;
            }

            .toggle-button-container {
                width: 10vw;
                margin-left: auto;
                margin-right: auto;
            }
        "#
    )
}