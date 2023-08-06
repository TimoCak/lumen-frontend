use stylist::{StyleSource, css};



pub fn get_thread_style() -> StyleSource {
    css!(
        r#"
            .thread-container {
                text-align: center;
                border: 2px;
                border-style: solid;
                border-radius: 10px;
                display: flex;
                flex-direction: column;
                gap: 1em;
                width: 80vw;
                margin-left: auto;
                margin-right: auto;
                margin-top: 2em;
                background-color: rgba(0,0,0,0.8);
                padding: 1em;
            }

            .thread-title {
                text-decoration: underline;
            }

            .thread-text {
                font-size: larger;
            }
        "#
    )
}