<div align="center">

  <h1>Beanstalk 🌱</h1>

  <p>
    <strong>A compiled, statically typed, all-in-one language designed for building UIs and websites.</strong>
  </p>

  *The only BS in programming should be in the filename*

  ⚠️  <sub>This is currently a work in progress compiler. It's not reccomended you try and actually use it yet!</sub>  ⚠️

</div>

---

## Principles
  - Fast Development of UIs and  content heavy web pages
  - Concise, minimal and consistent syntax
  - All-in-one design with minimal dependencies and boilerplate
  - The compiler should do most of the annoying work for you and help catch common bugs without compromising on very fast compile times (No slow LLVM backend)
  - Batteries included. Opinionated about how to do common fiddly tasks

## Planned Features
Beanstalk can be thought of as:
1. Mega-extended Markdown 
2. What would a good Javascript/HTML/CSS replacement look like, if it was actually designed to be used everywhere and had the hindsight of modern programming language design with no constraints on convention or backwards compatibility?
3. Eventually a general UI language for native C/C++ based applications, with C planned to be the target output

Beanstalk outputs HTML, CSS and JS/WASM all in one consistent syntax without having to switch between any languages, or even a different file.

All the basic stuff needed for quickly building a good website will be built into the compiler, so you can focus on the content and high level design decisions, not the boilerplate. 

Beanstalk aims to become an entire self-contained ecosystem for building webpages, UIs and even server side code and native applications, with a focus on simplicity and speed of development.

## Overview
When creating a page or UI in Beanstalk, you make Scenes. 

Scenes are a markup syntax that can be used to write text content, HTML, CSS and JS/WASM all in one file with it's own consistent syntax. It is designed to be very concise and easy to read.

**Content**

Write text content in a simple dialect of markdown, with only a few dashes needed to start a new scene and get writing. Images, videos and other media are easy to add and style with basic sensible modern styling built in as a starting point.

**HTML**

Scenes provide a template for your styles and content, with the ability to add custom elements and styling.

**🔥 You can finally center that div with only one keyword! 🔥**

**CSS**

Scenes have utility classes built in. Use keywords at the start of scenes to not only defined your elememts, but style and position them as well.

**JS/WASM**

*CODE GEN NOT YET IMPLIMENTED, BUT COMING SOON!*

Beanstalk will be a full programming language that compiles into JS/WASM on the frontend (and eventually native code on the backend) allowing you to add dynamic and interactive behaviours in your scenes naturally and concisely. 

The goal is to create a language, designed from the ground up, to be able to handle most of a techstack within one ecosystem.

Reactivity, (scene) components, compile time evaluation and more will be built into the language with many more features already planned out in the language syntax for the future.

<sub>Much of the language is still in the early design stages, with many things being tested/changing during developemnt</sub>

Technologies currently used in the compiler:
- [Pico CSS](https://picocss.com/) for the default CSS styling

---

## Current Goals
  - Finish implementing all basic scene styling and keywords
  - Finish implimenting constant folding and basic compile time evaluation
  - Finish basic frontend codegen
  - Dev server built into compiler tools
  - C target codegen for WASM, server code and native apps
  - Built-in Raylib intergration 
  - Complete all the basic features of the initial compiler so it can actually be downloaded and used easily 
