use stylist::{css, StyleSource};

pub fn get_home_style() -> StyleSource {
    css!(
        r#" 
            .home-sections {
                margin-top: 1em;
                display: flex;
                flex-direction: column;
                align-items: flex-start;
                gap: 1em;
                width: 100%;
                color: white;
            }
            .home-section {
                padding: 1em;
                background-color: rgba(0,0,0,0.9);
                border-radius: 10px;
            }
            .lumen-intro {
                color: white;
                background-color: rgba(0,0,0,0.9);
                border-radius: 10px;
                padding: 1em;
                width: 50vw;
            }
        "#
    )
}