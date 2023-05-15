use std::{collections::LinkedList, fmt::Debug, marker::PhantomData};

pub struct Type<T, M, C, A, V, L>(PhantomData<(T, M, C, A, V, L)>);

impl<T, M, C, A, V, L> seal::Seal for Type<T, M, C, A, V, L> {}

impl<T, M, C, A, V, L> TypeMarker for Type<T, M, C, A, V, L>
where
    T: Debug + Clone,
    M: Debug + Clone,
    C: Debug + Clone,
    A: Debug + Clone + AsRef<[Var<Self>]>,
    V: Debug + Clone,
    L: Debug + Clone,
{
    type Code = T;
    type Call = C;
    type Method = M;
    type Assign = A;
    type Variable = V;
    type Literal = L;
}

pub trait TypeMarker: Sized + seal::Seal {
    type Code: Debug + Clone;
    type Call: Debug + Clone;
    type Method: Debug + Clone;
    type Assign: Debug + Clone + AsRef<[Var<Self>]>;
    type Variable: Debug + Clone;
    type Literal: Debug + Clone;
}

mod seal {
    pub trait Seal {}
}

#[derive(Debug, Clone)]
pub struct Function {}

#[derive(Debug, Clone)]
pub struct Scope<T: TypeMarker> {
    pub id: ScopeId,
    pub ir: LinkedList<Ir<T>>,
}

#[derive(Debug, Clone)]
pub struct ScopeId(usize);

#[derive(Debug, Clone)]
pub enum Ir<T: TypeMarker> {
    Scope(IrScope<T>),
    Call(Call<T>),
    Method(Method<T>),
    Assign(Assign<T>),
    Ret(Ret<T>),
    Code(Code<T>),
    Lit(Lit<T>),
}

#[derive(Debug, Clone)]
pub enum IrScope<T: TypeMarker> {
    Block(Scope<T>),
    If(IfScope<T>),
    IfElse(IfElseScope<T>),
    Loop(Scope<T>),
    Closure(Scope<T>),
}

#[derive(Debug, Clone)]
pub struct IfScope<T: TypeMarker> {
    pub cond: Code<T>,
    pub if_scope: Scope<T>,
}

#[derive(Debug, Clone)]
pub struct IfElseScope<T: TypeMarker> {
    pub cond: Code<T>,
    pub if_scope: Scope<T>,
    pub else_scope: Scope<T>,
}

#[derive(Debug, Clone)]
pub struct Method<T: TypeMarker> {
    pub receiver: Var<T>,
    pub path: T::Method,
    pub args: Vec<Var<T>>,
}

#[derive(Debug, Clone)]
pub struct Var<T: TypeMarker> {
    pub id: VarId,
    pub name: T::Variable,
}

#[derive(Debug, Clone)]
pub struct VarId(usize);

#[derive(Debug, Clone)]
pub struct Call<T: TypeMarker> {
    pub path: T::Call,
    pub args: Vec<Var<T>>,
}

#[derive(Debug, Clone)]
pub struct Ret<T: TypeMarker> {
    pub var: Var<T>,
    pub scope: ScopeId,
}

#[derive(Debug, Clone)]
pub struct Assign<T: TypeMarker> {
    pub vars: T::Assign,
    pub value: Box<Ir<T>>,
}

#[derive(Debug, Clone)]
pub struct Lit<T: TypeMarker>(pub T::Literal);

#[derive(Debug, Clone)]
pub struct Code<T: TypeMarker>(pub T::Code);
