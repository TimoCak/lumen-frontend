use stylist::{StyleSource, css};



pub fn get_thread_style() -> StyleSource {
    css!(
        r#"
            .thread-container {
                text-align: center;
            }

            .thread-text {
                font-size: larger;
            }
        "#
    )
}