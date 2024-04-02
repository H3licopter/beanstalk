## GENERAL
- full expressions need to be supported in sceneheads
- Scene with only scenehead, followed by scene immediately doesn't work
- Other datatypes can be inserted directly into scenes. Raw strings, numbers, even collections should have a default behaviour etc.
- New non-inline scenes should allow text following it inline to be inline of it. (Maybe needs to be all wrapped in div?)
- Add in memory safety net for recursive function calls, or use a better pattern e.g tail recursion.

## HTML
- Image alt descriptions
- Markdown things to add:
  - Numbered lists
  - Underlining
  - strikethroughs ~~
  - blockquotes ~
  - Checkboxes / Radio buttons (will be empty scene for checkbox)

- Add most common HTML elements to the base language:
  - form, input, button, select, option, textarea
  - tables
  - header, footer, main, section, article, aside, 
  - nav

Needs to parse expressions inside of scenehead

## CSS
- Compile [custom pico css style](https://picocss.com/docs/sass) for the base of the CSS framework, remove pico- from the class names
- Create base BS css file for basic additional syling

## JS / WASM
- Mostly just compile to JS, then add wasm module for heavy lifting tasks and proper types. Too complex to try and compile too much dynamically to WASM format directly without building in LLVM or something that is too bloated.

## SYNTAX
- number types will be a single letter followed by the max size. u32, i8, f64, d63 (fixed point decimal with 63 decimal places). 
- think about making commas ignorable, does it work?
- Scene syntax is just triple dash muliline comment. So the parent scene is just a mutliline style comment.


## Notes
- Objects are stripped of methods when parsed into JSON or protobuf format
- Support for SPA solutions, or MPA focused only? (Probably just MPA)
- Print styles?
- Handling no JS browsers? - Allow for no js sites, just pure templates into HTML stuff?
- Uses rust markdown parser for parsing markdown into GFM markdown (Github Flavored Markdown)

## Future Stuff
@ - HTTP and sockets, server communication etc. 
defer
^ - pointers
& - Dereference pointer?
<< - Concurrency Channel
>> - Concurrency Channel
