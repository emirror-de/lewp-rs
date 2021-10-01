# lewp

Website generation suite with guarantee of always valid HTML5. Truly isolated modules during runtime.

Never touch confusing templates again! Write your website as dynamic as you can imagine without writing unclear code.

## Features

- [x] Build your HTML website fully from Rust source
- [x] Never touch confusing templates again
- [x] Always emit correct, minimized HTML5
- [ ] Website is fully split into self-contained modules, never interfering each other

## Roadmap

- [x] Skeleton to create your website by using DOM
- [x] A webpage can have Modules
- [x] A page is created with isolated modules (HTML only)
- [x] Modules can have Modules, infinite loops are prevented
    - [x] Submodules have `RuntimeInformation` available
- [ ] `<head>` module only
- [ ] Added CSS integration
    - [ ] File structure is defined
    - [ ] Combining files is implemented
    - [ ] Minimization of CSS is implemented
    - [ ] Directories can be configured on module level
- [ ] Modules are isolated (HTML, CSS)
- [ ] Added [html5-picture](https://github.com/emirror-de/html5-picture) support
    - [ ] Conversion of pictures on start is possible
    - [ ] API for a global register that holds all pictures and creates the HTML code
- [ ] Added JavaScript integration
    - [ ] File structure is defined
    - [ ] Combining files is implemented
    - [ ] Minimization is implemented
    - [ ] JavaScript is isolated
    - [ ] Directories can be configured on module level
- [ ] HTML can be streamed
- [ ] Modules are now fully isolated (HTML, CSS, JavaScript)
- [ ] CSS can be split up into "render critical" (will be inlined on rendering) and "non render critical" parts
    - [ ] CSS attributes can be configured
- [ ] JavaScript and CSS can be added to the binary
- [ ] Modules can send events to sub-modules

