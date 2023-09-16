# Mady

## State of project: **Learning**

When we start this project in high school class, we know nothing about rust/calculus/linear algebra. It is a crazy journal that we learn a lot of knowledge in the short time. The result of this project is success for a school project but not for a real life usage. It is lack of many basic functionality and some rabbit hole.

Now i get in the collage, and I have calculus and linear algebra class. I will come back when i acquired the knowledge this project need.

## This Project not died (yet)

We still planing how we can improve Mady, here is some goal.

### Mady 0.0 (old Mady project)

Old Mady analyze function base on AST, but simplicity come with limitation. It can't solve

- `for`
- `loop`
- `break`
- early `return`
- type inference (macro limitation)

### Mady 1.0 (Goal)

- language-independent IR for Mady
    - Support Control Flow Graph
- rustc_plugin
- better support for common crates (e.g. ndarray)
- buildin auto-diff graph optimizer