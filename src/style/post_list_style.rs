use stylist::{StyleSource, css};


pub fn get_post_list_style() -> StyleSource {
    css!(
        r#"
            .post-list-container {
                margin-left: auto;
                margin-right: auto;
                display: flex;
                flex-direction: column;
                text-align: center;
                width: 70vw;
                margin-top: 2em;
                background-color: rgba(0,0,0,0.8);
                padding: 1em;
                border-radius: 10px;
            }
        "#
    )
}