use crate::{settings::get_meta_config, ast::AstNode};
use super::generate_html::create_html_boilerplate;

// Parse ast into valid JS, HTML and CSS
pub fn parse(_ast: Vec<AstNode>) -> String {
  let js = String::new();
  let _wasm = String::new();
  let html = String::new();
  let css = String::new();

  // TO DO: Parse the ast

  create_html_boilerplate(get_meta_config())
    .replace("page-js", &js)
    .replace("page-template", &html)
    .replace("page-css", &css)
}
