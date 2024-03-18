# TO DO
## GENERAL
- Bug with lots of newlines in nested scene, will not wrap all the scene elements in the scene tag correctly.
- Switch collections to using curly brace syntax. All access is done via dot operator.
- Other datatypes can be inserted directly into scenes. Raw strings, numbers etc.
- Add 'alt' attribute to images.
- Try making the spaces after an inline element significant rather than at the end of the element. HTML automatically parses them out.
- Tidy up code a bit and prep for isolated task to create a function to manage precedence and parse maths expressions in the language.
- Add in memory safety net for recursive function calls, or use a better pattern e.g tail recursion.

## HTML
- Markdown things to add:
  - Underlining
  - strikethroughs
- Add most common HTML elements to the base language:
  - audio, video
  - lists
  - form, input, button, select, option, textarea
  - tables
  - header, footer, main, section, article, aside, nav
- custom markdown parsing

## CSS
- Compile [custom pico css style](https://picocss.com/docs/sass) for the base of the CSS framework, remove pico- from the class names
- Figure out if direct Tailwind integration is possible. Maybe just use a style if this works? Otherwise create custom utility classes, if even needed.

## JS / WASM
- Create WASM bindings for the DOM

## SYNTAX
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