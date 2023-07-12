use stylist::{css, StyleSource};

pub fn get_input_field_style() -> StyleSource {
    css!(
        r#"
            input {
                font-size: 24px;
                border-radius: 10px;
            }

            .login-field {
                display: flex;
                flex-direction: column;
                justify-content: space-evenly;
                align-items: center;
                width: 100%
            }
        "#
    )
}
