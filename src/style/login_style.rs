use stylist::{css, StyleSource};

pub fn get_login_style() -> StyleSource {
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

            button {
                font-size: 20px;
                background-color: rgba(0,0,0,0.8);
                border-radius: 10px;
                cursor: pointer; 
                transform: scale(1.1,1.1);
                box-shadow: 3px 3px 3px #E4EE8C, -4px 3px 3px #E4EE8C;
                color: white;
                transition: 0.5s;
                padding: 0.2em;
            }

            .button-container {
                text-align: center;
            }
        
        "#        
    )
}