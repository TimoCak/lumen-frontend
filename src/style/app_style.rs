use stylist::{StyleSource, css};

pub fn get_app_style() -> StyleSource {
    css!(
        r#"

        * { 
            margin: 0 !important;            
        }

        h3 {
            text-decoration: underline;  
            font-size: 36px;
        }

        body {
            background-color: rgba(200,200,200,1);
            font-family: monospace;
        }
        
        "#
    )
}