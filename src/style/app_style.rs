use stylist::{StyleSource, css};
//wichtige Farbe: #E4EE8C
pub fn get_app_style() -> StyleSource {
    css!(
        r#"

        body {
            margin: 0 !important; 
            background-color: rgba(0,0,0,0.7);
            font-family: Rubik;
            color: white;
        }

        h3 {
            font-size: 36px;
            padding: 0px !important;
            margin: 0px !important;
        }  

        a {
            color: pink;
        }
        "#
    )
}