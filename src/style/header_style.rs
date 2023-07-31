use stylist::{css, StyleSource};

pub fn get_header_style() -> StyleSource {
    css!(
        r#"
            .header {
                display: flex;
                justify-content: space-between;
                align-items: center;
                background-color: rgba(0,0,0,0.9);
                color: white;
                padding: 0.8em;
            }

            h1,h2 {
                cursor: pointer;
                padding: 0.2em;
                border-radius: 10px;
            }

            h1,h2:hover,.filter-light:hover {
                transform: scale(1.1,1.1);
                box-shadow: 3px 3px 3px #E4EE8C, -4px 3px 3px #E4EE8C;
                transition: 0.5s;
            }

            .filter-light:hover {
                transform: scale(1.1,1.1);
                box-shadow: 3px 3px 3px grey, -4px 3px 3px grey;
                transition: 0.5s;
            }

            .header-link {
                text-decoration: none;
                color: white;
            }

            .filter-light {
                filter: invert(92%) sepia(36%) saturate(517%) hue-rotate(7deg) brightness(101%) contrast(87%);
                cursor: pointer;
                border-radius: 50%;
                padding: 0.2em;
            }
        "#
    )
}