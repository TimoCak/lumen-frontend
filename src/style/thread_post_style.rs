use stylist::{css, StyleSource};

pub fn get_thread_post_style() -> StyleSource {
    css!(
        r#"
            .thread-post-container {
                display: flex;
                flex-direction: column;
                gap: 1em;
                color: white;
                background-color: rgba(0,0,0,0.8);
                padding: 1em;
                width: 60vw;
                margin-left: auto;
                margin-right: auto;
                border-radius: 10px;
                font-size: 24px;
            }

            .button-container {
                text-align: center;
            }

            textarea {
                height: 300px;
                font-size: 16px;
            }
        "#
    )
}
