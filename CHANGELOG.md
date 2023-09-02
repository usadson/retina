# Changelog

## 0.3.0 - Unreleased

### HTML Features
1. Bitmap favicon support added
2. Anchor `<a href>` elements can now be clicked upon to navigate to another page

### CSS Features
1. Complex selector support added (`h1 > p.title`, `form input`, `label + input`, etc.)
2. Aliased `flow-root` as `flow` (for the time being) to improve general layout of websites
3. Default background color restored (from magenta back to white)
4. Monospace fonts are now also preloaded
5. Reduced line length when displaying CSS parse errors, which would otherwise - in cases of minified files - print out the whole CSS stylesheet
6. Physical units are now supported: `in`, `cm`, `pt`, `mm`, `Q`
7. Font faces are now correctly case-insensitively matched
8. Avoid loading the same font when the load was started in a previous cycle, but not yet finished
9. All generic font families are now recognized and translated per platform
10. CSS-wide keywords are now recognized (`inherit`, `initial`, etc.) to e.g. avoid searching for fonts named `"inherit"`. Note that this does not imply the semantics of the values are supported.
11. Empty style rules are now not registered in the `Stylesheet` structure to improve selector matching performance
12. `@font-face` rules are now supported, meaning you can use the fonts of e.g. Google Fonts or your own to improve the visuals of websites
13. Fix broken CSS-wide keyword recognition
14. Absolute CSS font sizes are supported (e.g. `medium`, `x-large`, `xx-small`)
15. Fix broken `text-decoration` shorthand property.
16. Fix crash in CSS parse error logging
17. Fix crash in anonymous layout that starts with whitespace (occurred e.g. on Wikipedia)
18. Screen scaling (DPI) solves [the text issue](https://github.com/usadson/retina/issues/23)
19. CSS [`cursor`](https://drafts.csswg.org/css-ui/#cursor) property is now supported
20. Relative `font-size`s (e.g. the `em` unit) are not resolved at compute time, so that they cascade correctly.
21. [`white-space: pre`](https://drafts.csswg.org/css-text/#valdef-white-space-pre) will now correctly honor forced line breaks (e.g. with the use of the `U+000A LINE FEED '\n'` character).
22. Fix: emoji will no longer [break lines unnecessarily](https://github.com/usadson/retina/issues/30).
23. Implemented styles for `<input type="text">`, `<input type="checkbox">`, `<input type="radio">` and buttons.
24. Support pseudo class [`:checked`](https://drafts.csswg.org/selectors/#checked)
25. Support pseudo class [`:placeholder-shown`](https://drafts.csswg.org/selectors/#placeholder-shown)
26. Display placeholder if `value` is empty
27. Add UA style for greying out the placeholder text color
28. Fixed incorrect assumption that the first node of a document is the `<html>` node (it can also be a `<!-- comment -->`)
29. Add support for `<input type="hidden">`

### Performance improvements
1. `ColorMaterialRenderer` and `TextureMaterialRenderer` are now globally shared instead of per `Artwork`, which previously made the creation time of tiles significantly slower.
2. Replace some `.expect(&format(...))` calls with a conditional `let Ok(..) = .. else { panic!(...) }` calls to avoid String allocations in normal cases.
3. Image bitmaps with the same URL share the same resources & only fetches once

### Networking
1. Document URLs without a scheme (e.g. `https`, `ftp`) are now prefixed with `https://` in order to be more relaxed with URLs given to the `RETINA_URL` environment variable.
2. Provide standard __HTTP__ headers with fetch, e.g. [`Accept`](https://httpwg.org/specs/rfc9110.html#field.accept), [`User-Agent`](https://httpwg.org/specs/rfc9110.html#field.user-agent)
3. Provide [__Fetch Metadata__](https://w3c.github.io/webappsec-fetch-metadata/) __HTTP__ headers, e.g. [`Sec-Fetch-Dest`](https://w3c.github.io/webappsec-fetch-metadata/#sec-fetch-dest-header), [`Sec-Fetch-Mode`](https://w3c.github.io/webappsec-fetch-metadata/#sec-fetch-mode-header)
4. Requests are now possibly associated with a _referrer_ ([__HTTP__](https://httpwg.org/specs/rfc9110.html#field.referer), [__Fetch__](https://fetch.spec.whatwg.org/#concept-request-referrer)), which [makes some sites work](https://twitter.com/awesomekling/status/1695003722613432764) load their fonts correctly.
5. [`Content-Encoding`](https://httpwg.org/specs/rfc9110.html#field.content-encoding) is now supported (`br`, `gzip`, `deflate`)

### General
1. Added a crash screen, displaying where the error in source code occurred
2. Updated dependencies, namely moved `boa` from Git checkout to crates.io release
3. Bitmap images are now freed from the main memory after they've been uploaded to the GPU
4. The keybinding `F10` now displays a memory occupancy for the DOM and Layout tree
5. FreeType segmentation fault solved by loading glyphs on those systems serially instead of parallel (as on Windows with DirectWrite)
6. Crash solved for default fonts that could not be found (namely default emoji fonts)
7. Line-based scrolling, which is the type of scrolling emitted by a mouse, is now fixed. Previously only pixel-based scrolling was supported (emitted by track pads and touch screens).
8. Silenced warning on FreeType systems when requesting the origin of a glyph.
9. Support WOFF and WOFF2 font compressions
10. Honor screen scaling (DPI) ([fixes #23](https://github.com/usadson/retina/issues/23))
11. Composited emoji (like 👩🏼‍💻) are now properly recognized
12. Support changing the cursor of the windows
13. Anchor elements (`<a href="..">`) can now be clicked to navigate
14. Hit-testing now honors the scroll position as well
15. Fix rare crash when GIFs have 0 frames
16. Avoiding redraws for GIFs with only one frame (static image)
17. Added a special JSON translator that creates a special DOM with pretty-printed JSON output
18. Fixed crash in `DocumentWrapper` which prevented some pages with `<!-- comments -->` to load.


## 0.2.0 - Released 2023-08-05
This is the first official release!

### General Information
Version 0.1.0 was before the project was restructured to which it is today. In this version, there are a couple notable features:
1. HTML is parsed into a reference-counted DOM
2. Basic CSS parser
3. Simple CSS selector matcher, only `<simple selector>`
4. Style collection & cascading
5. `@media` query support for `screen` and `all` types
6. Text & whitespace transformations
7. User Agent style sheet
8. Internal Fetch API implementation
9. Remote & local document & resource fetch
10. Window resize is acted upon
11. Logging using the `log` crate
12. Window title is based upon the `<title>` element or the __URL__
13. A simple DOM inspector with <kbd>F12</kbd>
14. Basic CSS box-model
15. Shortcuts like <kbd>Ctrl</kbd>+<kbd>W</kbd>, <kbd>F5</kbd>, <kbd>F12</kbd>
16. Extensive Font API
17. Text hints (e.g. ligatures, CJK character forms)
18. Text shaping using harfbuzz
19. Still bitmap image support (JPG, PNG, BMP, WebP, etc.)
20. Animated GIF support
21. HTML `<img>` element support
22. CSS `background-image` support (non-gradients)
23. Basic compositor with parallel tiles of 256x256
24. <kbd>Ctrl</kbd>+<kbd>V</kbd> to paste a URL to open
25. Scrolling support
26. Parallelized glyph rasterization
27. Improved text anti-aliasing
28. Non-colored emoji support
29. `text-decoration` support
30. And a lot of performance improvements!

### Changes
- `[deps]` Update dependencies
- `[page]` Accept continuous task messages when no timeout expired
- `[page]` Document the DirtyState mechanism
- `[gfx+page]` Remove redundant debug messages
- `[app]` Count FPS for paint-heavy scenario's
- `[page]` Time scroll responsiveness
- `[app]` Fix incorrect `App` creation
- `[page]` Animate through each frame of GIFs
- `[dom+gfx+page]` Generate textures from GIF frames
- `[dom]` Special support for decoding GIF animated images
- `[ci]` Add GitHub Pages link for Rustdoc
- `[app+page]` Show backtrace of page crash in Window
- `[app]` Show a crash screen when the pages crashes, instead of halting
- `[compositor]` Ensure bottommost text fragments are continued on a vertical tile boundary
- `[layout]` Support emoji text fragments
- `[font]` Debug FontDescriptor with FontHandle
- `[font]` Add `FamilyName::Emoji`
- `[layout]` Remove fast path of anonymous layout algorithm
- `[page]` Reduce maximum delay between paints
- `[compositor]` Recall staging belt after awaiting SubmissionFuture
- `[compositor]` Enable `wgpu` tracing/profiling
- `[compositor]` Ensure composition of the last tiles are submitted correctly
- `[font]` Improve glyph rendering by eliminating double AA
- `[compositor]` Remove dependency on retina-gfx-font-backend-font-kit
- `[gfx+font]` Move font descriptors and hints back to retina-gfx-font!
- `[font]` Move the font rendering code to a separate crate
- `[gfx]` Decouple `font-kit` backend from the main crate
- `[layout]` Hot fix crash in subtendril'ing
- `[font]` Reduce locking overhead for FontKitFont and GlyphAtlas
- `[font]` Add instrumentation to backend font implementation
- `[compositor]` Allow tracing of composition cycles
- `[gfx]` Cull textured rects outside the viewport
- `[compositor]` Cull out tile-non-intersecting text fragments
- `[compositor]` Fix some clippy hints
- `[compositor]` Use stdlib's thread scope instead of crossbeam's
- `[compositor]` Hide most messages behind `trace` log level
- `[compositor]` Re-enable background color filling
- `[compositor]` Clear the tile's background a normal color
- `[compositor]` Document the publicly accessible API
- `[compositor]` Don't request a composition cycle when the tile was cached
- `[compositor]` Delay waiting on submit until next composite cycle
- `[compositor]` Keep bottom tiles in cache when scrolling up
- `[page]` Don't resubmit the composited page
- `[compositor]` Merge `paint` and `composite` into one
- `[gfx]` Rename common log message to clear up confusion
- `[compositor]` Composite tiles in parallel
- `[compositor]` Composite immediately when a tile is ready
- `[compositor]` Debug the times taken per tile
- `[compositor]` Only paint tiles that intersect the viewport
- `[compositor]` Paint tiles according to the viewport's vertical position
- `[compositor]` Use a tile-based system for rendering the screen
- `[font]` Prepare basic latin by also including the U+0020 SPACE glyph
- `[ua]` Update test according to the CSS parsing improvements!
- `[gfx]` Trace bind & draw commands
- `[gfx]` Create a sampler once per texture renderer
- `[gfx-font]` Prepare renderer once per text run
- `[gfx]` Instrument more functions
- `[style]` Support compound `p.blue#main` selectors!
- `[style]` Move simple selector matching to a separate function
- `[style+page]` Load correct fonts using the given `font-weight`
- `[font]` Better propagate succes status for font loading
- `[gfx+style]` Support `font-variant-position`
- `[gfx]` Support East Asian glyph substitution
- `[style]` Support `font-variant-east-asian`
- `[dom+i18n+layout]` Support `text-transform` CSS property
- `[style]` Support `text-transform`
- `[compositor]` Support `text-decoration`: line-through and overline
- `[style-parser]` Support `font-weight`
- `[compositor]` Support basic `text-decoration` properties
- `[font]` Add accessors for baseline, underline offset+thickness
- `[style]` Add `text-decoration` properties
- `[layout]` Remove old debug logging
- `[common+layout]` Fix crash for multi-byte character index
- `[woff]` Start working on an WOFF(2) decompressor
- `[layout]` Translate `font-variant-caps` to `gfx-font` format
- `[gfx-font]` Support capital letter font hints
- `[style]` Support `font-variant-caps`
- `[page]` Add a CSS missing font loading mechanism
- `[gfx]` Font description should be `Hash` and `Eq`
- `[gfx-font]` Add `FontProvider::load_from_system()` API
- `[page]` Fix page task message timeout system
- `[page]` Generalize style resource loading into a separate function
- `[gfx-font]` Don't load fonts in the background if they don't exist
- `[style]` Support `font-kerning` and `font-variant-ligatures`
- `[gfx-font]` Add ability to change ligatures & kerning hints
- `[gfx+font+gui]` Remove `wgpu_glyph` dependency
- `[gfx-font]` Diverge path for grayscale or anti-aliasing
- `[gfx-font]` Remove old `ascent` per glyph from the font
- `[gfx-font]` Tidy up `glyph_iter`
- `[gfx]` Use HarfBuzz offsets & advances instead of the defaults
- `[gfx-font]` Fix baseline correction in text painting
- `[gfx-font]` Fix font height calculation
- `[gfx-font]` Use HarfBuzz for text shaping
- `[gfx-font]` Use the font size for atlas cache matching
- `[gfx-font]` Calculate alpha channel based on avg glyph mask
- `[gfx-font]` Use alpha blending for glyph painting
- `[gfx]` Allow color blending in `TextureMaterialRenderer`
- `[gfx-font]` Paint using the text color supplied
- `[gfx]` Give `TextureMaterialRenderer` better debug WebGPU labels
- `[gfx]` Fix incorrect resource reference in `paint_rect_textured_with`
- `[gfx]` Give texture paint resources a better name
- `[gfx-font]` Implement glyph position correctly
- `[gfx-font]` Chore: clean-up font_kit backend glue
- `[gfx-font]` Re-enable parallelized glyph rasterization
- `[gfx-font]` Remove debug `abort` in Glyph loading
- `[gfx]` Make `paint_rect_textured` extensible
- `[gfx]` Streamline material rendering code
- `[gfx-font]` Custom text rendering with `font_kit` glyph rasterization
- `[gfx+font]` Abstract `Font` painting
- `[gfx-font]` Rename `Font` to `WgpuFont`
- `[svg]` Start working on an SVG implementation
- `[gfx]` Don't emit scroll events if the cursor is outside the window
- `[fetch]` Special path for local page URL parsing
- `[layout]` Use serif font as the default
- `[style]` Support `<body>` presentational hints
- `[ci]` Install XCB package on Linux for clipboard to work
- `[app]` Use `copypasta` instead of `clipboard`
- `[ua]` Update stylesheet test because of CSS parser improvements
- `[app+page]` Open a website using an URL of the clipboard (Control+V)
- `[app+page]` Add keybind F6 to dump the style sheets
- `[page]` Set the URL as the title on (re)load
- `[style-comp]` Support pseudo selector specificity
- `[gfx-font]` Move `FontKitAbGlyphFontBridge::new()` outside the Windows cfg
- `[style-comp]` Support pseudo selector matching for links and empty elements
- `[style]` Support parsing pseudo class selectors
- `[gfx]` Fix incorrect text color
- `[layout]` Add struct ActualValues
- `[layout]` Pass the IFC state to the LayoutBox::run_layout() function
- `[layout]` Describe crash (panic) for subtendril errors in line box fragmentation
- `[style-parser]` Support `currentColor`
- `[style-parser]` Extract color parsing into a separate function
- `[layout+compositor]` Support CSS `currentColor`
- `[gfx-font]` Begin working on `font-kit` rendering support
- `[style]` Support `margin-block` and `margin-inline` properties
- `[style]` Parse `<body>` presentational `background` and `text` hints
- `[layout]` Run special path (replaced) for `<img>` elements
- `[page]` Don't crash on a network error in Fetch
- `[page]` Follow redirect URLs in HTTP responses
- `[fetch]` Add redirect URL getter for `Response`
- `[page]` Scroll to top on page (re)load
- `[page]` Warn if there isn't a layout root when calling paint()
- `[page]` Always clean dirty state before entering command/task loop
- `[common]` Remove failing example of StrExt::index_of_substring()
- `[page]` Queue relayout, repaint, etc. by keeping track of the dirty state
- `[app]` Map keys to PageUp, PageDown, Home, End
- `[page]` Add actions Page{Up,Down} and ScrollTo{Bottom,Top} (Home & End)
- `[page]` Construct the scroller with the initial viewport size
- `[page]` Ensure scroll position isn't negative
- `[layout]` Implement the line box fragment algorithm!
- `[layout]` Block formatting context: calculate the width of the container
- `[compositor]` Draw the `LineBoxFragment`s instead of the whole text
- `[layout]` Add a base constructor for FormattingContext
- `[layout]` Add the LineBoxFragment vector to LayoutBox
- `[layout]` Add the `LineBoxFragment` type
- `[gfx-font]` Calculate the width of the U+0020 space character beforehand
- `[layout]` Dimensions: add utilities for the combined edges
- `[common]` Add some substring utilities
- `[common]` Add `str::index_of_substring()`
- `[gfx]` Don't bound the text painting to the canvas size
- `[style-comp]` Inherited props should be applied cascaded before rule declarations
- `[style-comp]` Font properties should be inherited
- `[layout]` Fix CSS to ab_glyph font weight mapping
- `[page]` Remove the debug log of the stylesheets
- `[gfx+page]` Add scrolling input events and move viewport
- `[page]` Add Scroller construct
- `[gfx]` Add viewport position API
- `[style-comp]` Computate `@media` rules
- `[style-parser]` Parse simple `@media {screen,print,all}` rules
- `[style]` Add `@media` rules
- `[page]` Generate textures for loaded `background-image`s
- `[compositor]` Draw `background-image`s
- `[layout]` Load `background-image`s
- `[style]` Add the `background-image` property
- `[style]` Support CSS `<image>` values
- `[fetch]` Handle unknown schemes
- `[fetch]` Extract the HTTP-version out of the public `fetch` API
- `[compositor]` Render the images of <img> elements
- `[page]` Load <img> sources after the HTML was parsed
- `[dom]` <img>: Add `data_ref()` API to get the ImageData as reference
- `[gfx]` Create texture view at `Texture` creation
- `[fetch]` Fetch local non-document files
- `[fetch]` Prevent panic on Request building
- `[layout]` Default background color of the page should be white
- `[gfx]` Add the `Texture` API for uploading textures
- `[gfx]` Add `Context` accessor to the `CanvasPaintingContext`
- `[dom]` Extend APIs of ImageData
- `[dom]` Accessor for the image data directly, instead of the APIs
- `[dom]` Add helper to recurse children with the `Node` handle
- `[media-type]` Fix media type sniffing not rewinding the seek buffer
- `[dom]` Add `<img>` element and image loading algorithm
- `[fetch]` Add `Content-Type` (MIME) getter to `Response`
- `[media-type]` Add image media type sniffer
- `[fetch]` Fix url of the request initiator
- `[gfx]` Include win32 module
- `[gfx]` Introduce a GUI system interface
- `[gfx]` Rewrite painter to streamline canvas & window rendering
- `[gfx]` Render textured rects using the actual given rect
- `[gfx]` Fix textured vertices for matrix transform
- `[gfx]` Store viewport size in render pass
- `[gfx]` Remove clear color debug log
- `[gfx]` Move transformation math to a `math` module
- `[deps]` Update dependencies
- `[ci]` Use `[rust-cache]`(https://github.com/Swatinem/rust-cache)
- `[compositor+gfx+layout]` Skip painting layout boxes outside the viewport
- `[dom+layout]` Make nodes implement `PartialEq`
- `[gfx+page]` Make submitting async for completion
- `[gfx+layout+page]` Use bg color of `<html>` as canvas clear color
- `[docs]` Add page that tracks the environment variables that are used
- `[compositor+gfx]` Trace compositor calls into a chrome://tracing format
- `[gitignore]` Ignore trace JSON files
- `[deps]` Add `tracing` dependencies & tracing feature flag for wgpu
- `[retina]` Update default env log filters
- `[gfx]` Prevent triple resize event when the window opens
- `[gfx]` Don't submit every draw call
- `[page]` Print warning when layout paint takes more than 200ms
- `[page+layout]` Don't regenerate layout tree on resize
- `[retina-style]` Parse `float` property & value
- `[style-*]` Cascade HTML `style` attributes
- `[user-agent]` Fix the failing UA stylesheet test
- `[layout]` Remove anonymous layout debug `info!`
- `[gfx]` Make window resizable
- `[layout]` IFC: Specify the origin of line boxes based on previous ones
- `[layout]` Use `LineBox`es in the Inline Formatting Context
- `[layout]` Correctly wrap whitespace
- `[layout]` Pass the base FC to the child layout
- `[layout]` Add shared base for BFC and IFC
- `[gfx-font]` Size calculation should use 1.5x multiplier
- `[layout]` Start implementing the IFC
- `[style-comp]` The initial value of `display` must be `inline`
- `[ua]` Add more styles for phrasing content to the UASS
- `[layout]` BorderStyle of None should discard `border-width`
- `[layout]` Fix box internal calculation bugs
- `[layout]` Start implementing the BFC
- `[gfx+layout]` Find font based on the CSS computed style
- `[style]` Parse & resolve font-* properties
- `[style-comp]` `font-size` should be inheritable
- `[gfx]` Multiply the CSS font size with 1.5x
- `[gfx]` Fix text color bug
- `[cargo]` Update dependencies
- `[gfx+layout+page]` Paint text using the provided CSS font
- `[compositor+gfx]` Paint the text of layout nodes
- `[page]` Initialize a `BrowsingContext` after page load
- `[scripting]` Add scripting & platform object crates
- `[gfx-font]` Implement APIs for loading and retrieving fonts
- `[gfx]` Sketch out gfx-font crate
- `[layout]` Use content box instead of padding box for child boxes layout
- `[compositor]` Use `padding` properties for border placement
- `[layout]` Calculate padding box edges
- `[style+comp]` Parse `padding` declarations
- `[compositor+layout]` Correctly paint background
- `[style-comp]` Fix bug with `margin` shorthand
- `[app+page]` Reload page with the F5 key
- `[retina+page]` Dump layout tree with key F1
- `[ci]` `continue-on-error: true` for Rust channel fallbacks
- `[style-parser]` Refactor code for **cssparser** version upgrade
- `[cargo]` Update dependencies
- `[ci]` Use toolchain fallbacks if nightly channel is unavailable
- `[compositor+layout]` Use `margin` with layout calculation
- `[style]` Parse `margin` properties
- `[compositor]` Paint borders of layout nodes
- `[gfx]` Fix incorrect position and size of painting a colored rect
- `[layout+style]` Calculate border sizes with content size
- `[style]` Parse `border` properties
- `[layout]` Correctly compute CSS `<length>`s
- `[page]` Regenerate layout tree on canvas resize
- `[style]` Parse percentage (`25.5%`) values
- `[style]` Parse `vh` and `vw` units
- `[style]` Parse `em` and `rem` units
- `[style]` Parse `font-size` properties
- `[page]` Load `<link rel="stylesheet">` stylesheets
- `[fetch]` Add `url` and `new` APIs for `Request`
- `[dom]` Support `<link>` elements and their **`rel`** attributes
- `[style-parser]` Don't panic on unknown color values
- `[dom]` Implement `Sync` for `Node` and `NodeKind`
- `[everywhere]` Use atomic `StrTendril`
- `[docs]` Add license and CI badges
- `[docs]` Fix README crate link
- `[docs]` Add README with an introduction to the project
- `[docs]` Add Apache 2.0 license
- `[node]` Use atomic `RwLock` instead of sync `RefCell`
- `[dom]` Use atomic reference counting for `Node` handles
- `[page]` Introduce the PageTaskMessage pipeline
- `[debug]` Include attributes with elements
- `[dom]` Implement `Display` for `AttributeList`
- `[compositor]` Fix incorrect height of content
- `[style-computation]` Implement selector specificity
- `[layout]` Don't warn on `display: none`
- `[dom]` Decouple the `Rc`'ness of `NodeKind`
- `[dom]` Rename `Node` to `NodeInterface`
- `[dom]` Add & parse the **`Comment`** node
- `[dom]` Subclass `CharacterData` for `Text`
- `[debug]` Fix compile error for `cfg!` if statement
- `[debug+gfx+page]` Add a simple Win32 DOM tree viewer
- `[main+gfx+page]` Move to event-based window architecture
- `[dom]` Fix broken concatenation of adjacent Text nodes
- `[fetch]` Support `file://` document URLs
- `[fetch+page]` Load pages using the new `fetch` crate
- `[page]` Paint canvas' background as specified by HTML
- `[gfx+compositor]` Paint colored rect with actual rect dimensions
- `[compositor+gfx]` Render background colors
- `[dom]` Combine adjacent text nodes
- `[retina]` Set page URL to the window title
- `[style-parser]` Move style parsing to a separate crate
- Merge remote-tracking branch 'origin/fetch'
- `[style]` Improve declaration parsing
- `[layout]` Update generation for new `CssDisplay` types
- `[style+more]` Replace custom color parsing with `cssparser`
- `[fetch]` Introduce Fetch API crate
- `[layout+style]` Add formatting contexts and pseudo elements
- Merge branch 'main' of github.com:usadson/retina
- `[style]` Parse `<length>` values
- `[style-comp]` Add `width` and `height` computed properties
- `[retina]` Delete old files
- `[gfx]` Use `log` crate for logging
- `[layout]` Use `log` crate for logging
- `[gfx]` Use `log` crate for logging
- `[retina]` Use `log` crate for logging
- `[page]` Use `log` crate for logging
- `[gfx+page]` Handle resizing of the window/surface
- `[style]` Parse `display` as `<display-inside>` and `<display-outside>`
- `[style]` Parse attribute selectors
- `[style]` Parse `#id` and `.class` selectors
- `[style+comp]` Add `#id`, `.class` and ``[attr]`` selectors
- `[dom]` Add missing utilities for `NodeKind` and `AttributeList`
- `[page]` Dump stylesheets after they've been parsed
- `[style]` Warn on parse error and empty declarations
- `[user agent]` Extend the stylesheet to hide <head>
- `[ci]` Use Rust nightly for compilation
- `[user agent]` Move the `about:` pages to _retina-user-agent_
- `[main]` Default log level to `warn` on debug builds
- `[dom]` Warn about quirks mode & parse errors
- `[dom]` Store parsed attributes and display on dump
- `[dom]` Separate element creation by following the spec.
- `[dom]` Implement `DumpableNode` for `NodeKind`
- `[layout]` Implement `DumpableNode` for `LayoutBox`
- `[common]` Add `DumpableNode` trait for dumping node-trees
- `[layout]` Introduce `LineBox` type of inline boxes
- `[layout]` Fix generated layout box kinds
- `[user-agent]` Add User Agent stylesheet
- `[layout+dom]` Improve DOM tag in layout dump
- `[style]` Add type selectors (e.g. `h1`, `p`)
- `[layout]` Add dumping mechanism
- `[layout]` Append child layout boxes to `LayoutBox::children`
- `[layout]` Correction: initial containing block is parent of <html> element
- `[layout+more]` Add box dimensions to the `Box`
- `[style]` Add `background-color` property
- `[compositor]` Introduce the concept of the compositor
- `[gfx+page]` Add Canvas to draw the page to
- `[page]` Add basic Page infrastructure
- `[test/html]` Add simple `<br>` line break test
- `[layout]` Pass parent node to layout generator
- `[dom]` Add mutable children getter for `ParentNode`
- `[style-comp]` Inherit properties after cascading
- `[style]` Add `white-space` property
- `[style]` Add `white-space` value kind
- `[dom]` Use `Tendril` instead of `String` for text nodes
- `[layout]` Sketch out basic `Box` structure
- `[style]` Add `display` property
- `[style-comp]` Replace `ComputedStyle` with `PropertyMap`
- `[style]` Add `Rule::At` for `@rules`
- `[style-comp]` Cascade collected styles
- `[style]` Associate `CascadeOrigin` with `StyleRule`
- `[style]` Introduce cascade origin
- `[style-comp]` Add style collector
- `[style-comp]` Add selector matcher
- `[style]` Add CSS parser
- `[gfx]` Move `Color` to **retina-common** crate
- `[dom]` Use test in `test/html` as input
- `[dom]` Add simple DOM component
- `[gfx]` Handle keyboard events (e.g. Ctrl+W)
- `[gfx]` Separate components into different structures
- `[gfx]` Add initial data and code for graphics
- `[retina]` Initialize `env_logger`
- `[everywhere]` Restructure directories

## 0.1.0
- After this version, the project was completely restructured
- Simply CI created
- Initial creation
