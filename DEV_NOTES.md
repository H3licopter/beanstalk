# TO DO
## GENERAL
- Should move any html tag creation from create_scene_node to web_parser and have a unique AST node before that instead.
- Other datatypes can be inserted directly into scenes. Raw strings, numbers etc.
- Add 'alt' attribute to images.
- Try making the spaces after an inline element significant rather than at the end of the element. HTML automatically parses them out.
- Tidy up code a bit and prep for isolated task to create a function to manage precedence and parse maths expressions in the language.
- Add in memory safety net for recursive function calls, or use a better pattern e.g tail recursion.

## HTML
- Markdown things to add:
  - Numbered lists
  - Underlining
  - strikethroughs ~~
  - blockquotes ~
  - Checkboxes / Radio buttons (will be empty scene for checkbox)

- refactor markdown parser to parse all custom markdown in the same loop (look through the content once and replace all markdown with the correct HTML in one pass). Maybe a collection of parsing states that can contain each markdown type.

- Add most common HTML elements to the base language:
  - audio, video
  - lists
  - form, input, button, select, option, textarea
  - tables
  - header, footer, main, section, article, aside, nav

## CSS
- Compile [custom pico css style](https://picocss.com/docs/sass) for the base of the CSS framework, remove pico- from the class names
- Create base BS css file for basic additional syling

## JS / WASM
- Create WASM bindings for the DOM

## SYNTAX
- number types will be a single letter followed by the max size. u32, i8, f64, d63 (fixed point decimal with 63 decimal places). d0 f0, i0 etc means the compiler decides. d0 = array of binary digits in memory (arbitary precision)
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
