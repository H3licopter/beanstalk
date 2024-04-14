#![allow(dead_code)]
pub enum DOMUpdate {
    InnerHTML,
    AppendChild,
    RemoveChild,
    ReplaceChild,
}

pub fn _generate_dom_update_js(update: DOMUpdate) -> &'static str {

    /* 
        JS Functions that accept the ID of an element
        and what the new updated value will be.

        There is a different function for each type of DOM update, 
        so only the necessary functions should be added at compile time into the JS output.

        The compiler will only generate the necessary JS functions for each kind type of DOM update that is needed in the program.
    */

    match update {
        DOMUpdate::InnerHTML => {
            &"function uInnerHTML(id,update){document.getElementById(id).innerHTML=update;}"
        }
        DOMUpdate::AppendChild => {
            &"function uAppendChild(id,update){document.getElementById(id).appendChild(update);}"
        }
        DOMUpdate::RemoveChild => {
            &"function uRemoveChild(id,update){document.getElementById(id).removeChild(update);}"
        }
        DOMUpdate::ReplaceChild => {
            &"function uReplaceChild(id,update){document.getElementById(id).replaceChild(update);}"
        }
    }
}