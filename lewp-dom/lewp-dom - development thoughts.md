# lewp-dom - development thoughts

## Typed nodes

In order to provide typed nodes without reinventing the wheel on DOM elements, it is necessary to either wrap (in this case `markup5ever::Node`) the original node type, compose a new type with the original node, or integrate all properties of the node into the new type and write an interface that allows to access and reuse them.

In case of `lewp-dom` it has been decided to use the latter. This is due to several conflicts that arose with the other two approaches during development and testing. Validation for example is not possible recursively with wrapping the original node by a new type, due to the trait making it possible requires to get the children of a typed node as parameter, but then the type of the node is not available anymore when passed to the method.

## HTML5 specifcation validation

When compiled in `debug` mode, the `TypedNode::append_child` and `TypedNode::append_children` methods validate all nodes being passed. If a node is not allowed, it does not get appended. This makes sure that you do not violate the HTML5 spec. When built in `release` mode, this validation is removed. This is intended to speed up the performance during runtime.

However due to this conception, it is still possible to violate the specification if you want to force it. Because the validation methods are removed in `release` build configuration, all nodes are appended. In addition to that it is possible to append a node to a child of the node where it is usually not allowed. During development this is considered an edge case and does not happen if you build your HTML from the inner to the outer nodes. The following example shows this case.

```
// An anchor node with a div node is created
<a><div></div></a>
// Later during the algorithm, a second anchor node is created and
// appended to the div
<a><div><a></a></div></a>
// This violates the spec, but is possible with lewp-dom
```

If you are wondering how this is possible, see the following snippet which is **not** recommended to be used.

```rust
let div_element = div(vec![]);
let a = a(vec![div_element]);
div_element.append_child(a(vec![text("This is not allowed!")]));
```
