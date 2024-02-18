// TEMPORARY MARKDOWN PARSING USING LIB AFTER TRIMMING NON NEWLINE WHITESPACE
pub fn parse_markdown_to_html(md_content: &String) -> String {

    markdown::to_html(&md_content)
}

// fn get_closing_tag(element: &str) -> String {
//     let index = element.char_indices().rev().find(|c| c.1 == '/');
//     match index {
//         Some((i, _)) => {
//             let tag = element.split_at(i + 1)
//                 .1
//                 .replace(">", "");
//             tag
//         }
//         None => { String::new() }
//     }
// }