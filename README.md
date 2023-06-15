# ✨ Retina - A Browser Engine Project 🔗
Retina is a fresh approach to web browser engines, written entirely in Rust. It aims to stay close to web standards and specifications, focusing on correctness, performance, and safety. This is a monorepo that contains the main binary `retina`, alongside several libraries that define different aspects of the browser's operations.

## 📦️ Components
* [`retina`](retina) - The main application entry point.
* [`retina-common`](retina-common) - Shared types and utilities used across other libraries.
* [`retina-compositor`](retina-compositor) - Responsible for drawing the layout to the screen.
* [`retina-dom`](retina-dom) - Implements the [Document Object Model](https://dom.spec.whatwg.org/), managing and manipulating the tree structure of the webpage.
* [`retina-fetch`](retina-fetch) - Responsible for making HTTP requests to retrieve resources over the internet. This implements the [Fetch API](https://fetch.spec.whatwg.org/)
* [`retina-gfx`](retina-gfx) - Graphics subsystem for both the compositor and the GUI.
* [`retina-layout`](retina-layout) - Implements the CSS box model and layout algorithms.
* [`retina-page`](retina-page) - Orchestrates the creation and management of web pages.
* [`retina-style`](retina-style) - Implements the CSS representation.
* [`retina-style-computation`](retina-style-computation) - Computes the final style of the elements based on CSS rules.
* [`retina-style-parser`](retina-style-parser) - Parses CSS stylesheets and inline styles.
* [`retina-user-agent`](retina-user-agent) - Handles the behavior and identity of the web browser, and also specifies the browser/implementation-specific behavior.

## 🏃 Process Flow
This section describes the current process flow, which is currently simple but is crafted in such a way
that a multi-page and multi-process architecture is eventually possible, without coupling these too much
to the current single-page architecture.
1. __retina__ starts up, creates a window and a page
2. __retina-page__ resolves the URL and starts a page load
3. __retina-fetch__ loads the HTML contents of the URL
4. __retina-page__ invokes __retina-dom__
5. __retina-dom__ parses the HTML and constructs a DOM tree
6. __retina-page__ collects the stylesheets
7. __retina-page__ invokes __retina-style__
8. __retina-style_parser__ parses the CSS associated with these stylesheets
9. __retina-page__ invokes __retina-layout__
10. __retina-layout__ invokes __retina-style-computation__
11. __retina-style-computation__ resolves and computes the style per element
12. __retina-layout__ creates a CSS Box tree per element using the computed style
13. __retina-page__ invokes __retina-compositor__
14. __retina-compositor__ sends drawing commands to __retina-gfx__ using the Box tree
15. __retina-gfx__ translates the draw commands to `wgpu`-specific calls
16. __retina-gfx__ executes these commands on the page surface
17. `wgpu` invokes the underlying graphics library (OpenGL, Vulkan, etc.)
18. __retina-page__ sends a texture view of the page surface to the main process
19. __retina__ calls __retina-gfx__ to paint this texture view to the window surface

## 📚 Quick Links
* [CSS 2.2](https://www.w3.org/TR/CSS22/)
* [CSS Cascading and Inheritance Level 4](https://drafts.csswg.org/css-cascade-5/)
* [CSS Display Module Level 4](https://drafts.csswg.org/css-display-4)
* [CSS Indices](https://www.w3.org/TR/CSS/#indices)
* [CSS Selectors Module Level 4](https://drafts.csswg.org/selectors/)
* [CSS Syntax Module Level 4](https://drafts.csswg.org/css-syntax-3/)
* [CSS Values and Units Module Level 4](https://drafts.csswg.org/css-values/)
* [DOM Standard](https://dom.spec.whatwg.org/)
* [Fetch Standard](https://fetch.spec.whatwg.org/)
* [HTML Living Standard](https://html.spec.whatwg.org/multipage/)
* [WebGPU](https://www.w3.org/TR/webgpu/)

## ⚖️ License
Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at
http://www.apache.org/licenses/LICENSE-2.0.

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
