<div align="center">

  <h1>Beanstalk 🌱</h1>

  <p>
    <strong>A compiled, statically typed, all-in-one language designed for building UIs and websites.</strong>
  </p>

  *The only BS in programming should be in the filename*

  ⚠️  <p>This is currently a work in progress compiler. It's not reccomended you try and actually use it yet!</p>
  ⚠️  <p>Core parts of the design are still subject to large changes and iteration as the compiler is developed</p>
  ⚠️  <p>The CLI interface is not yet user friendly or designed for use outside of developing the compiler (yet)</p>

</div>

---

## Principles
  - Fast Development of UIs and content heavy web pages
  - Concise, minimal and consistent syntax
  - All-in-one design with minimal boilerplate
  - The compiler should do most of the annoying work for you and help catch common bugs without compromising on very fast compile times (No slow LLVM backend)
  - Batteries included. Opinionated about how to do common fiddly tasks
  - Fast to prototype an idea and optimize later

## Example

    -- Beanstalk example! (this is a comment)
    variable := "variables"

    float_constant :: 5 * 68.9

    beans_img : "https://upload.wikimedia.org/wikipedia/commons/thumb/d/d9/Heinz_Beanz.jpg/2560px-Heinz_Beanz.jpg"

    -- The following 3 dashes start a new scene body
    ---
    
    # Beanstalk scene bodies looks a bit like markdown
    But all of a sudden you can start nesting *scenes* inside of them!

    [rgb(140, 200, 255): 
    
    Scenes are a way to group and style sections of your content. 
    They can easily import [variable], and can contain expressions. 
    And it's all [ 60 + 40 ]% clean, concise and readable.
    
    ]

    The scene head can contain a lot of useful keywords to style and position your content.

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
      alt "BEANS"
    ]

## Planned Features
Beanstalk can be thought of as a mega-extended Markdown, or a high level modern UI focused programming language with no constraints on programming conventions or backwards compatibility.

Beanstalk outputs HTML, CSS and JS/WASM all in one consistent syntax without having to switch between any languages, or even a different files if you want to centralise all of your logic.

All the basic stuff needed for quickly building a good website will be built into the compiler, so you can focus on the content and high level design decisions, not the boilerplate. There is no Node.js or javascript framework hell to navigate and no verbose lower level language noise to distract from building what you want.

Beanstalk aims to become a self-contained ecosystem that can interop with exsisting javascript / wasm libraries. The long term goal is to impliment full WASM codegen so you can build server side or native code using a WASM runtime with the same high level syntax.

## Overview
When creating a page or UI in Beanstalk, you make Scenes.

Scenes are a markup syntax that can be used to write text content, HTML, CSS and JS/WASM all in one file with it's own consistent syntax. It is designed to be very concise, easy to read and flexibly interop with regular code with built in reactivity.

**Content**

Write text content in a simple dialect of markdown, with only a few dashes needed to start a new scene and get writing. Images, videos and other media are easy to add and style with basic sensible modern styling built in as a starting point.

**HTML**

Scenes provide a template for your styles and content, with the ability to add custom elements and styling.

**🔥 You can finally center that div with only one keyword! 🔥**

**CSS**

Scenes have utility classes built in. Use keywords at the start of scenes to not only defined your elememts, but style and position them as well.

**JS/WASM**

Beanstalk will be a full programming language that compiles into JS/WASM on the frontend (and eventually native code on the backend) allowing you to add dynamic and interactive behaviours in your scenes naturally and concisely. 

The goal is to create a language, designed from the ground up, to be able to handle most of a techstack within one ecosystem.

Reactivity, (scene) components, compile time evaluation and more will be built into the language with many more features already planned out in the language syntax for the future.

<sub>Much of the language is still in the early design stages, with many things being tested/changing during developemnt</sub>

Technologies currently used in the compiler:
- [Pico CSS](https://picocss.com/) for the default CSS styling

---

## Current Goals
  - Finish implementing all basic scene styling and keywords for scenes
  - Finish implimenting constant folding and basic compile time evaluation
  - Basic scene components and reactivity implemented
  - Cranelift backend for full WASM codegen and wasm runtime compile targets
  - Complete all the basic features of the initial compiler so it can actually be downloaded and used easily 
