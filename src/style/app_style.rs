use stylist::{StyleSource, css};
//wichtige Farbe: #E4EE8C
pub fn get_app_style() -> StyleSource {
    css!(
        r#"

        body {
            margin: 0 !important; 
            background-color: rgba(200,200,200,1);
            font-family: monospace;
        }

        h3 {
            font-size: 36px;
            padding: 0px !important;
            margin: 0px !important;
        }  
        "#
    )
}