## GENERAL
- full expressions need to be supported in sceneheads. Code gen only needs to happen if eval expression returns an expression rather than literal.
  - create_expression should impliment shunting yard
  - eval_expression should then solve the expression
- Other datatypes can be inserted directly into scenes. Raw strings, numbers, even collections should have a default behaviour etc.
- Add in memory safety net for recursive function calls, or use a better pattern e.g tail call recursion.

## HTML
- Image alt descriptions
- Markdown things to add:
  - Numbered lists
  - Underlining
  - strikethroughs ~~
  - blockquotes ~
  - Checkboxes / Radio buttons (will be empty scene for checkbox)

- Add most common HTML elements to the base language:
  - form
  - tables
  - input
  - button
  - nav
  - select
  - textarea
  - header, footer
  - section, article (mostly auto generated?)
  - aside

## CSS
- padding: pad (l, r, t, b)
- margin (m)
- center
- rgba
- Compile [custom pico css style](https://picocss.com/docs/sass) for the base of the CSS framework, remove pico- from the class names and merge into main bs css file.

## JS / WASM
- Mostly just compile to JS, then add wasm module for heavy lifting tasks and proper types. Too complex to try and compile too much dynamically to WASM format directly without building in LLVM or something that is too bloated.

  import init, { } from './bs_wasm.js';
  async function run() {
    await init();
  }
  run();
  //js

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
