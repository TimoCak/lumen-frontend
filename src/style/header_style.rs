use stylist::{css, StyleSource};

pub fn get_header_style() -> StyleSource {
    css!(
        r#"
            .header {
                display: flex;
                justify-content: space-between;
                align-items: center;
                background-color: black;
                color: white;
                padding: 2em;
                font-family: monospace;
            }

            h1,h2 {
                cursor: pointer;
                padding: 5px;
            }

            h2:hover {
                transform: scale(1.1,1.1);
                box-shadow: 3px 3px 3px grey, -4px 3px 3px grey;
                transition: 0.5s;
            }
        "#
    )
}