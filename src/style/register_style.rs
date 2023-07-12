use stylist::{css, StyleSource};

pub fn get_register_style() -> StyleSource {
    css!(
        r#"
        .login-form {
            font-size: 24px;
            display: flex;
            align-items: center;
            flex-direction: column;
            gap: 2em;
            margin-top: 2em;
            margin-left: auto;
            margin-right: auto;
            background-color: rgba(0,0,0,0.8);
            color: white;
            padding: 1em;
            width: 60vw;
            border-radius: 10px;
        }
        
        .button-container {
            text-align: center;
        }
        "#
    )
}