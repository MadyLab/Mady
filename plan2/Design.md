
## Goal

- Avoid seperating process of function(or scope that change/read variable of its outer scope)

    Simplify algorithm(But algorithm of DAG would be more complex)

- Avoid parsing ``grad_XXX_function`` twice
- Basic control flow (if,else)
- Process loop with known iteration

## How

Maintain a mapping(variable name to ``stack<Node of variable>``).

When assigning a variable, push a ``Node`` containing initial value to mapping[``variable name``].

When changing a variable, simply record the step on ``Node``, take ``a = b * a`` for example:

```rust
enum Op<T,F> where F:Fn(/*...*/)/*->...*/{
    Op(Box<dyn F>), Const(T)
}

let result = Node::new(Op::Multiply);

result.add_child_sync(b);
result.add_child_sync(a);

a = result;
```

When meeting control flow (if,else), copy current mapping.

## Problem to solve

- How to find similar path structure on DAG?

    Current solution is through version, which is pretty inefficient.

- How to solve loop with unknown iteration

    IDK



