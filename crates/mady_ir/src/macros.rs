#[macro_export]
macro_rules! ir {

    {__@scope $id:ident $($ir:tt)* } => {
        {
            let mut lk = std::collections::LinkedList::new();
            ir! {
                lk
                $($ir)*
            }
            $crate::ir::Scope {
                id: $id,
                ir: lk,
            }
        }
    };

    // block
    {$ll:ident block $id:ident => {$($ir:tt)*}; $($remain:tt)* } => {
        $ll.push_back($crate::ir::Ir::Scope($crate::ir::IrScope::Block($crate::ir! {
            __@scope $id $($ir)*
        })));
    };

    // if
    {$ll:ident if $cond:ident $id:ident => {$($ir:tt)*}; $($remain:tt)* } => {
        $ll.push_back($crate::ir::Ir::Scope($crate::ir::IrScope::If($crate::ir::IfScope {
            cond: $cond,
            if_scope: $crate::ir! {
                __@scope $id $($ir)*
            },
        })));
        $crate::ir!{
            $ll
            $($remain)*
        }
    };
    
    // if-else
    {$ll:ident if $cond:ident $id:ident => {$($ir:tt)*} else $id_el:ident => {$($ir_el:tt)*}; $($remain:tt)* } => {
        $ll.push_back($crate::ir::Ir::Scope($crate::ir::IrScope::IfElse($crate::ir::IfElseScope {
            cond: $cond,
            if_scope: $crate::ir! {
                __@scope $id $($ir)*
            },
            else_scope: $crate::ir! {
                __@scope $id $($ir_el)*
            }
        })));
        $crate::ir!{
            $ll
            $($remain)*
        }
    };

    // call
    {$ll:ident $path:ident( $($arg:ident),* ); $($remain:tt)* } => {
        $ll.push_back($crate::ir::Ir::Call($crate::ir::Call{
            path: $path,
            args: vec![$($arg),*],
        }));
        $crate::ir!{
            $ll
            $($remain)*
        }
    };

    // method
    {$ll:ident $receiver:ident.$path:ident ($($arg:ident),*); $($remain:tt)* } => {
        $ll.push_back($crate::ir::Ir::Method($crate::ir::Method{
            receiver: $receiver,
            path: $path,
            args: vec![$($arg),*],
        }));
        $crate::ir!{
            $ll
            $($remain)*
        }
    };

    // assign
    {$ll:ident $vars:ident = ( $($ir:tt)+ ); $($remain:tt)* } => {
        
        $ll.push_back($crate::ir::Ir::Assign($crate::ir::Assign{
            vars: $vars,
            value: {
                let mut lk = std::collections::LinkedList::new();
                $crate::ir!{
                    lk
                    $($ir)+;
                }
                if lk.len() != 1 {
                    panic("expect ir, get ir list")
                }
                lk.pop_back()
            },
        }));
        $crate::ir!{
            $ll
            $($remain)*
        }
    };

    // ret
    {$ll:ident ret $scope:ident => $var:ident; $($remain:tt)* } => {
        $ll.push_back($crate::ir::Ir::Ret($crate::ir::Ret{
            var: $var,
            scope: $scope,
        }));
        $crate::ir!{
            $ll
            $($remain)*
        }
    };

    // lit
    {$ll:ident lit $value:ident; $($remain:tt)* } => {
        $ll.push_back($crate::ir::Ir::Lit($crate::ir::Lit($value)));
        $crate::ir!{
            $ll
            $($remain)*
        }
    };

    // code
    {$ll:ident code $value:ident; $($remain:tt)* } => {
        $ll.push_back($crate::ir::Ir::Code($crate::ir::Code($value)));
        $crate::ir!{
            $ll
            $($remain)*
        }
    };

    // empty
    {$ll:ident} => {};
}

fn a() {
    // LinkedList::new();
    // ir! {
    //     lk
    //     if a b => {
    //         abc(a,b,c);
    //         code ccc;
    //         aaa = (a.abc(c));
    //         lit ttt;
    //     };
    //     block c => {};
    // };
}
