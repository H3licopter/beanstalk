use colour::red_ln;

// All the custom CSS styles that get 
pub fn get_bs_css(name: &str) -> &str {
    match name {
        "nav-0" => {
            ".bs-nav-0 {
                border-bottom-color: grey;
                border-bottom-style: solid;
                border-bottom-width: 1px;
                padding: 0.5rem 2rem 0rem 2rem;
            }"
        }
        "nav-1" => {
            ".bs-nav-1 {
                border-bottom-color: grey;
                border-bottom-style: solid;
                border-bottom-width: 1px;
                background-color: rgba(255, 255, 255, 0.01);
                padding: 0.5rem 2rem 0rem 2rem;
                margin-bottom: 1rem;
            }"
        }

        _ => {
            red_ln!("Error: CSS class not found");
            ""
        }
    }
}