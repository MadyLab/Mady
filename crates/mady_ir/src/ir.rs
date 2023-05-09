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
pub struct Scope<const L: usize, T: TypeMarker> {
    pub id: ScopeId,
    pub ir: LinkedList<Ir<L, T>>,
}

#[derive(Debug, Clone)]
pub struct ScopeId(usize);

#[derive(Debug, Clone)]
pub enum Ir<const L: usize, T: TypeMarker> {
    Scope(IrScope<L, T>),
    Method(Method<T>),
    Call(Call<T>),
    Code(Code<T>),
    Assign(Assign<L, T>),
    Ret(Ret<L, T>),
}

#[derive(Debug, Clone)]
pub enum IrScope<const L: usize, T: TypeMarker> {
    If(Scope<L, T>),
    IfElse(IfElseScope<L, T>),
    Loop(Scope<L, T>),
    Closure(Scope<L, T>),
}

#[derive(Debug, Clone)]
pub struct IfElseScope<const L: usize, T: TypeMarker> {
    pub if_scope: Scope<L, T>,
    pub else_scope: Scope<L, T>,
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
pub struct Ret<const L: usize, T: TypeMarker> {
    pub var: [Var<T>; L],
    pub scope: ScopeId,
}

#[derive(Debug, Clone)]
pub struct Assign<const L: usize, T: TypeMarker> {
    pub vars: T::Assign,
    pub value: Box<Ir<L, T>>,
}

#[derive(Debug, Clone)]
pub struct Lit<T: TypeMarker>(pub T::Literal);

#[derive(Debug, Clone)]
pub struct Code<T: TypeMarker>(pub T::Code);
