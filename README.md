<div align="center">

  <h1>Beanstalk 🌱</h1>

  <p>
    <strong>An open source, statically typed, all-in-one language designed for building UIs and webpages.</strong>
  </p>

  *The only BS should be in the filename*

  <sub>This is currently a work in progress compiler. It's not reccomended you try and actually use it yet!</sub>

</div>

---

## Principles
  - Fast DX
  - Concise and minimal syntax
  - All-in-one design with minimal dependencies and boilerplate
  - The compiler should do most of the annoying work for you
  - Opinionated about how to do common tasks

## Planned Features
Beanstalk can be thought of as Markdown extended into it's own full language. The compiler is designed to eventually target more than just the web, but can currently output content, HTML, CSS and JS/WASM all in one consistent syntax without having to switch between any languages, or even a different file.

All the basic stuff needed for quickly building a good website will be built into the compiler, so you can focus on the content and design, not the boilerplate. 

Beanstalk aims to become an entire self-contained ecosystem for building webpages and UIs, with a focus on simplicity and speed of development without sacrificing performance.

## Overview
When creating a page or UI in Beanstalk, you make Scenes. Scenes are a markup syntax that can be used to write content, HTML, CSS and JS/WASM all in one file. It is very minimal and easy to read, with a few simple rules to follow.

**Content**

Write content in a simple dialect of markdown, with only a few dashes needed to start a new scene and get writing.

**HTML**

Scenes provide a template for your styles and content, with the ability to add custom elements and styling. They are like a far more minimal and more readable version of JSX.

**CSS**

Scenes have utility classes built in. Use keywords at the start of scenes to not only defined your elememts, but style and position them as well.

**JS/WASM**

(CODE GEN NOT YET IMPLIMENTED) Beanstalk will be an entire programming language that compiles into JS/WASM on the frontend and ties into your scenes naturally for dynamic content and interactivity. The language is planned to also compile to other platforms including server side so you can write your entire stack in Beanstalk.

Reactivity, (scene) components, compile time evaluation and more will be built into the language with many more features already planned out in the language syntax for the future.

<sub>Much of the language is still in the early design stages, with many things being tested/changing during developemnt</sub>

Technologies currently used in the compiler:
- [Pico CSS](https://picocss.com/) for the default CSS styling

---

## Current Goals
  - Complete all the basic features of the initial compiler
  - Make compiler CLI better
