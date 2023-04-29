<!-- Sorry, I haven't figure out how to use fcitx5-gtk3-frontend on fedora 37 KDE spin -->
<!-- Unfortuantely, vscode(electron.js) is base on gtk3 -->
# copy on write Directed acyclic graph

## Goals

- Keep current state
- Copy on write
- Shorten path

## Theory

In this explaination, we don't care about ADG's implementation.

## General Remark

The expected struct would like:

```rust
struct AddressNode<N,E>{
    children:Rc<ChildrenNode<N,E>>,
    data:Rc<DataNode<N>>,
    version:Vec<usize>
}
struct ChildrenNode<N,E>{
    children:(E,Rc<AddressNode<N,E>>)
}
struct DataNode<N>{
    data:N
}
```

## Version Counter

Each graph has a incremental counter, which record how many operation has been done.

When we decide to archive, we create a new counter for that archive node and its derivatives, then increase their version's dimension by one.

And we just use Rc to reclaim unused memory and check version, and ensure node(bypass)'s version(of all dimensions)  is below current's.
