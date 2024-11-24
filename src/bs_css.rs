use colour::red_ln;
use const_format::formatcp;

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
        "codeblock-0" => {
            const LIGHT: &str = "#ddd";
            const DARK: &str = "#222";
            const COMMENT_DARK: &str = "#838c86";
            const COMMENT_LIGHT: &str = "#759061";
            const KEYWORD_DARK: &str = "#ea943e";
            const KEYWORD_LIGHT: &str = "#ad590e";
            const STRING_DARK: &str = "#4fba11";
            const STRING_LIGHT: &str = "#007130";
            const NUMBER_DARK: &str = "#96b5ed";
            const NUMBER_LIGHT: &str = "#073f85";
            const OPERATOR_DARK: &str = "#9ec1ff";
            const OPERATOR_LIGHT: &str = "#2b476a";
            const STRUCT_DARK: &str = "#a17fbd";
            const STRUCT_LIGHT: &str = "#ac2187";
            const TYPE_DARK: &str = "#54c5d3";
            const TYPE_LIGHT: &str = "#0d8db3";
            const PARENTHESIS_DARK: &str = "#bcac52";
            const PARENTHESIS_LIGHT: &str = "#ad7e0d";

            formatcp!(
                "codeblock{{color:light-dark({},{})}}.bs-code-comment{{color:light-dark({},{})}}.bs-code-keyword{{color:light-dark({},{})}}.bs-code-string{{color:light-dark({},{})}}.bs-code-number{{color:light-dark({},{})}}.bs-code-operator{{color:light-dark({},{})}}.bs-code-struct{{color:light-dark({},{})}}.bs-code-type{{color:light-dark({},{})}}.bs-code-parenthesis{{color:light-dark({},{})}}",
                DARK, LIGHT, COMMENT_LIGHT, COMMENT_DARK, KEYWORD_LIGHT, KEYWORD_DARK, STRING_LIGHT, STRING_DARK, NUMBER_LIGHT, NUMBER_DARK, OPERATOR_LIGHT, OPERATOR_DARK, STRUCT_LIGHT, STRUCT_DARK, TYPE_LIGHT, TYPE_DARK, PARENTHESIS_LIGHT, PARENTHESIS_DARK
            )
        }

        _ => {
            red_ln!("Error: CSS class not found");
            ""
        }
    }
}
