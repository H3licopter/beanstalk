<div align="center">

  <h1>Beanstalk 🌱</h1>

  <p>
    <strong>An all-in-one language for building UIs and websites</strong>
  </p>

  *The only BS in programming should be in the filename*

  <br>

  ---
  <br>
  <p>⚠️ This is currently a work in progress compiler. It's not reccomended you try and actually use it yet!</p>
  <p>⚠️ Core parts of the design are still subject to large changes and iteration as the compiler is developed</p>

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

It's core syntax feature is using scenes, which are a declarative syntax built into an otherwise procedural language.

Scenes are a markup syntax that can be used to create content, styling and logic all in one place. It is designed to be very concise, easy to read and flexibly interop with regular code with built in reactivity.

Scenes provide a template for your styles and content, with the ability to add custom elements and styling.
They can be nested and used as components in other scenes.

**Markdown Built In**
Write text content in a simple dialect of markdown. Images, videos and other media are easy to add and style with basic sensible modern styling built in as a starting point.

**🔥 You can finally center that div with only one keyword! 🔥**

Scenes have utility classes built in. Use keywords at the start of scenes to not only defined your elememts, but style and position them as well.

You can import CSS into your Beanstalk scenes for more complex styling.

### WASM
Beanstalk is a programming language that compiles mostly into Web Assembly, allowing you to add dynamic and interactive behaviours in your scenes naturally and concisely while performing faster than regular web page.

### Technologies currently used in the compiler
- [Pico CSS](https://picocss.com/) for the default CSS styling

<br>

---