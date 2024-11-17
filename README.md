<div align="center">

  <h1>Beanstalk 🌱</h1>

  <p>
    <strong>A language for bringing joy back to web development</strong>
  </p>

  *The only BS in programming should be in the filename*

  <br>

  ---
  <br>
  <p>⚠️ This is currently a work in progress compiler. It's not reccomended you try and actually use it yet!</p>
  <p>⚠️ The design and direction of the language is still subject to change overtime</p>

  <h1>
    <a href="https://h3licopter.github.io/beanstalk">
      Documentation
    </a>
  </h1>
  <p>The docs were created entirely using Beanstalk. The output of the compiler is directly pushed to Github pages.</p>
  <a href="https://github.com/H3licopter/beanstalk-plugin">Language support / syntax highlighting plugin for Visual Studio / VSCode can be found here</a>

</div>
<br>
<br>

# Overview
Beanstalk is a compiled programming language which outputs HTML, CSS and WASM/JS all in one consistent syntax without having to switch between any languages. 

With fast compile times and built in hot-reloading, Beanstalk is designed to feel like a scripting language with all the power of being compiled.

### Scenes

It's core syntax idea is using scenes, which are a declarative syntax built into an otherwise procedural language.

Scenes can be used to create content, styling and basic logic all in one place.

Scenes provide a template for your styles and content, with the ability to create custom elements and styling.
They can be nested and used as components in other scenes.

**Markdown Built In**
Write content in a simpler dialect of markdown. Images, videos and other media are easy to add and style with a sensible modern CSS starting point.

**🔥 You can finally center that div with only one keyword! 🔥**

Use keywords at the start of scenes to define, style and position all your elements.
The compiler will only create any CSS or JS you've actually used.

### Compiled Output
Beanstalk uses Web Assembly to unlock more datatypes than what JS can offer on it's own.

Being compiled means folding constants, type checking and optimizing the output to be as small as possible is all done for you.

The built in hot-reloading development server can be used to see changes in real time. 

The compiler itself is written in Rust, and uses as few dependencies as possible to keep it fast and secure.

### Modern Language Design
The code itself outside of scenes takes inspiration from the best of new programming language ideas, while focusing in on being extremely simple, concise and batteries-included.

*Design Goals*
- Simple but powerful type system that helps avoid bugs without getting in the way of productivity
- Errors as values with extremely concise error handling syntax
- Low symbol noise, intuitive keywords
- Easy to learn for beginners and experienced programmers
- No weird syntax or design inconsistencies, everything should make sense and be intuitive in what it's doing under the hood

### Technologies currently used in the compiler
- [Pico CSS](https://picocss.com/) for the default CSS styling reset

<br>
