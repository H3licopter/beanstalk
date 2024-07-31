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

  <h3>
    <a href="https://h3licopter.github.io/beanstalk">
      Documentation
    </a>
  </h3>
  <p>The docs are written entirely in Beanstalk!</p>

</div>
<br>
<br>

# Principles
  - Fast Development of content heavy web pages
  - Concise, minimal and consistent syntax
  - As little boilerplate as possible
  - Fast compile times with hot module reloading for rapid development
  - Batteries included. Built in dev server and other various tools.

<br>

# Example

    -- Beanstalk example! (this is a comment)
    float_constant :: 5 * 68.9

    beans_img :: "https://upload.wikimedia.org/wikipedia/commons/thumb/d/d9/Heinz_Beanz.jpg/2560px-Heinz_Beanz.jpg"

    [:
    
    # Beanstalk scene bodies looks a bit like markdown
    But all of a sudden you can start nesting *scenes* inside of them!

    [rgb(140, 200, 255): 
      This block of next is now light blue!
      
      Scenes are a way to group and style sections of your content. 
      They can easily import variables, and can contain expressions. 
      And it's all [ 60 + 40 ]% clean, concise and readable.
    ]

    The scene head contains a keywords to style and position your content. This replaces the role of both HTML element names and CSS classes and combines them into a mix of both.

    ## Why use Beanstalk?
    - You can write your content naturally, without any HTML or CSS boilerplate.
    - Break out into using powerful compiled expressions anytime
    - Modern programming language syntax and modern UI sensibilities all in one file.

    You can easily add variables into your scenes such as that float defined earlier ([float_constant]). 
    And all of this content automatically gets wrapped in the correct HTML tags and CSS classes.

    # Cool Things
    You can easily add images, videos, and other media to your content,
    and even add multiple within the same scenehead for them to automatically form a grid.

    **Here's a grid of beans you didn't ask for**
    [
      img(beans_img)
      img(beans_img)
      img(beans_img)
      img(beans_img) 
      alt("BEANS")
    ]

<br>

# Overview
Beanstalk is a compiled, statically typed programming language which outputs HTML, CSS and JS/WASM all in one consistent syntax without having to switch between any languages. 

It's core syntax feature is using scenes.

Scenes are a markup syntax that can be used to create content, styling and logic all in one place. It is designed to be very concise, easy to read and flexibly interop with regular code with built in reactivity.

### Markdown
Write text content in a simple dialect of markdown. Images, videos and other media are easy to add and style with basic sensible modern styling built in as a starting point.

### HTML
Scenes provide a template for your styles and content, with the ability to add custom elements and styling.
They can be nested and used as components in other scenes.

**🔥 You can finally center that div with only one keyword! 🔥**

### CSS
Scenes have utility classes built in. Use keywords at the start of scenes to not only defined your elememts, but style and position them as well.

You can also import CSS into your Beanstalk scenes for more complex styling.

### JS/WASM
Beanstalk will be a full programming language that compiles into Web Assembly, allowing you to add dynamic and interactive behaviours in your scenes naturally and concisely in it's own language.

Reactivity, (scene) components, compile time evaluation and more will be built into the language with many more features already planned out in the language syntax for the future.

### Technologies currently used in the compiler
- [Pico CSS](https://picocss.com/) for the default CSS styling

<br>

---

### Current Goals
  - Finish implementing all basic scene styling and keywords for scenes
  - Finish implimenting constant folding and basic compile time evaluation
  - Basic scene components and reactivity implemented
  - Full WASM codegen and wasmer runtime compile target
  - Complete all the basic features of the initial compiler so it can actually be downloaded and used easily 
