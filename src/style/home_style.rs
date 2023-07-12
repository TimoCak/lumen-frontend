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
                color: white;
            }
            .home-section {
                padding: 1em;
                background-color: rgba(0,0,0,0.8);
                border-radius: 10px;
                width: 100vw;
            }
            .lumen-intro {
                display: block;
                margin-top: 1em;
                margin-bottom: 1em;
                margin-left: auto;
                margin-right: auto;               
                color: white;
                background-color: rgba(0,0,0,0.8);
                border-radius: 10px;
                padding: 0.5em;
                text-align: center;
                width: 50vw;
            }
        "#
    )
}