use colour::red_ln;

// All the custom CSS styles injected into the HTML dynamically
pub fn get_bs_css(name: &str) -> &str {
    match name {
        // Titles (bigger more dramatic headings than h1)
        "title-0" => {
            ".bs-title-0 {
                font-size: 3.5rem;
                font-weight: bold;
                letter-spacing: 0.4rem;
                word-spacing: 0.5rem;
                margin: 1.5rem 0 1.4rem 0;
                line-height: 3.7rem;
            }"
        }
        "title-1" => {
            ".bs-title-1 {
                font-size: 3rem;
                font-weight: bold;
                letter-spacing: 0.3rem;
                word-spacing: 0.4rem;
                margin: 1.3rem 0 1.2rem 0;
                line-height: 3.5rem;
            }"
        }
        "title-2" => {
            ".bs-title-2 {
                font-size: 2.5rem;
                font-weight: bold;
                letter-spacing: 0.3rem;
                word-spacing: 0.35rem;
                margin: 1.2rem 0 1rem 0;
                line-height: 3.3rem;
            }"
        }
        // Nav
        "nav-0" => {
            ".bs-nav-0 {
                padding: 0rem 2rem 0rem 2rem;
            }"
        }
        "nav-1" => {
            ".bs-nav-1 {
                border-bottom-color: grey;
                border-bottom-style: solid;
                border-bottom-width: 1px;
                padding: 0rem 2rem 0rem 2rem;
            }"
        }

        _ => {
            red_ln!("Error: CSS class not found");
            ""
        }
    }
}
