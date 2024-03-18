// Loop through and replace all Markdown formatting with correct tags
// Also make sure to escape reserved HTML characters
pub fn add_tags(content: &mut String, i: &mut usize) -> String {
    *content = content
        .replace('<', "&lt;")
        .replace('>', "&gt;");
    
    while *i < content.len() - 1 {
        if !add_em_tags(content, i) {
            break;
        }
        *i += 1;
    }

    while *i < content.len() - 1 {
        if !add_superscript_tags(content, i) {
            break;
        }
        *i += 1;
    }

    content.trim_start().to_string()
}

// 1 aserisk = <em>
// 2 asterisks = <strong>
// 3 asterisks = <strong><em>
// 4+ asterisks = <b> (will have a custom very bold style for the tag in the future)
fn add_em_tags(content: &mut String, i: &mut usize) -> bool {
    let mut open_index: usize = 0;
    let mut close_index: usize = 0;
    let mut asterisk_open_count = 0;
    let mut asterisk_close_count = 0;

    // Get starting index of asterisks and determine strength of emphasis
    while let Some(char) = content.chars().nth(*i) {
        if char == '*' {
            asterisk_open_count += 1;
        } else {
            open_index = *i;
            break;
        }
        *i += 1;
    }

    // Check if asterisk_count is found at any point later in the content
    // And remeber the index.
    while let Some(char) = content.chars().nth(*i) {
        if char == '*' {
            asterisk_close_count += 1;
        } else if asterisk_close_count == asterisk_open_count {
            close_index = *i;
            break;
        }
        *i += 1;
    }

    // If close count is equal to open count, then emphasis is valid
    if asterisk_open_count == asterisk_close_count && asterisk_open_count > 0 {
        let emphasis_open = match asterisk_open_count {
            1 => "<em>",
            2 => "<strong>",
            3 => "<strong><em>",
            _ => "<b>",
        };
        let emphasis_close = match asterisk_open_count {
            1 => "</em>",
            2 => "</strong>",
            3 => "</em></strong>",
            _ => "</b>",
        };

        // Replace asterisks with emphasis tags
        content.replace_range(open_index - asterisk_open_count..open_index, emphasis_open);

        let new_close_index = close_index + emphasis_open.len() - asterisk_open_count;
        content.replace_range(
            new_close_index - asterisk_open_count..new_close_index,
            emphasis_close,
        );
    }

    if *i > content.len() - 1 || !content.contains('*') {
        return false;
    }

    true
}

fn add_superscript_tags(content: &mut String, i: &mut usize) -> bool {
    let mut open_index: usize = 0;
    let mut close_index: usize = 0;

    // Get starting index
    while let Some(char) = content.chars().nth(*i) {
        if char == '^' {
            open_index = *i;
            break;
        }
        *i += 1;
    }

    if open_index == 0 {
        return false;
    }

    // Get closing index
    while let Some(char) = content.chars().nth(*i) {
        if char == '^' {
            close_index = *i;
            break;
        }
        *i += 1;
    }

    if close_index == 0 {
        return false;
    }

    // Replace carets with sup tags
    content.replace_range(open_index..open_index + 1, "<sup>");
    content.replace_range(close_index..close_index, "</sup>");

    true
}
