assert(document != null, "document is null");

assert(window != null, "window is null");
assert(window?.document != null, "window.document is null");
assert(this === window, "Window is not the global object");
assert(typeof document === "object", `document is not an object: ${typeof document}`);

assert(document === window.document);
assert(document.constructor.name == "Document", `document.constructor.name is not 'Document': '${document.constructor.name}'`);

assert(typeof document.title === "string", `document.title is not a string: ${typeof document.title}, keys: ${JSON.stringify(Object.keys(document))}`);
assert(document.title === "", `document.title is not empty: "${document.title}"`);

document.title = "Hello World";
assert(document.title == "Hello World", `document.title is not 'Hello World': "${document.title}"`);
