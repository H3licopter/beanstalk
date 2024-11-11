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
Beanstalk is a compiled, statically typed programming language which outputs HTML, CSS and WASM/JS all in one consistent syntax without having to switch between any languages. 

It's core syntax idea is using scenes, which are a declarative syntax built into an otherwise procedural language.

Scenes can be used to create content, styling and basic logic all in one place. It is designed to be very concise, easy to read and flexibly work within the rest of the code.

Scenes provide a template for your styles and content, with the ability to create custom elements and styling.
They can be nested and used as components in other scenes.

**Markdown Built In**
Write text content in a simpler dialect of markdown. Images, videos and other media are easy to add and style with basic sensible modern styling built in as a starting point.

**🔥 You can finally center that div with only one keyword! 🔥**

Scenes have utility classes built in. Use keywords at the start of scenes to define, style and position all your elements.
The compiler will only add any CSS you've used to the page itself.

### Compiled Output
Beanstalk is a programming language that uses Web Assembly to unlock stricter datatypes and more powerful computation.

Beanstalk also does a lot of heavy lifting at compile time, folding constants, type checking and optimizing the output to be as small as possible.

It's designed to have super fast compile times, so the built in hot-reloading development server can be used to see changes in real time. The compiler itself is written in Rust, and uses as few dependencies as possible to keep it fast and secure.

### Technologies currently used in the compiler
- [Pico CSS](https://picocss.com/) for the default CSS styling reset

<br>