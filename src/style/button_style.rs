use stylist::{css, StyleSource};

pub fn get_button_style() -> StyleSource {
    css!(
        r#"
        button {
            background-color: rgba(0,0,0,0.8);
            border-radius: 10px;
            cursor: pointer; 
            transform: scale(1.1,1.1);
            box-shadow: 3px 3px 3px #E4EE8C, -4px 3px 3px #E4EE8C;
            color: white;
            transition: 0.5s;
            padding: 0.2em;
        }
        "#
    )
}