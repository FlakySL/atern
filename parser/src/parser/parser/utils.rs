use crate::parser::ast::nodes::TreeNode;
use crate::parser::ast::syntax_kind::SyntaxKind;
use chumsky::{
    prelude::*,
    input::InputRef,
    container::Container,
    extra::ParserExtra,
    extension::v1::*
};

impl Container<TreeNode> for TreeNode{
    fn push(&mut self, item: TreeNode){
        self.add(item).unwrap();
    }
}

impl Default for TreeNode{
    fn default() -> Self{
        TreeNode::NonTerminal{
            //parent: None,
            kind: SyntaxKind::ROOT,
            children: vec![],
        }
    }
}

#[macro_export]
macro_rules! boxed_vec{
    ($($x:expr),*) => {
        vec![$(Box::new($x),)*]
    }
}


pub struct SomeOf_<T>{
    parsers: T,
}

pub type SomeOf<T> = Ext<SomeOf_<T>>;

macro_rules! impl_some_of_for_tuple {
    () => {};
    ($head:ident $Head:ident $($x:ident $X:ident)*) => {
        impl_some_of_for_tuple!(@recur $head $Head $($x $X)*);
        impl_some_of_for_tuple!($($x $X)*);
    };
    
    (@recur $head:ident $Head:ident $($x:ident $X:ident)*) => {
        #[allow(unused_variables, non_snake_case)]
        impl<'src, I, E, $Head, $($X,)* O> ExtParser<'src, I, Vec<Box<O>>, E> for SomeOf_<($Head, $($X,)*)>
        where
            I: Input<'src>,
            E: ParserExtra<'src, I>,
            $Head: Parser<'src, I, O, E>,
            $($X: Parser<'src, I, O, E>),*
        {
            fn parse(&self, inp: &mut InputRef<'src, '_, I, E>) -> Result<Vec<Box<O>>, E::Error> {
                let count: usize = impl_some_of_for_tuple! (@count $($X),*);
                let mut result: Vec<Box<O>> = Vec::with_capacity(count);
                let mut used: u32 = 0;
                let ($head, $($x),*) = &self.parsers;

                loop{
                    let begin = inp.cursor();
                    let mut i = 1;
                    
                    if used & i == 0 {
                            let before = inp.save();
                            match inp.parse(&$head){
                                Ok(out) => {
                                    result.push(Box::new(out));
                                    used |= i;
                                },
                                Err(_) => inp.rewind(before.clone()),
                            };
                    }
                    i <<= 1;
                    
                    $(
                        if used & i == 0 {
                            let before = inp.save();
                            match inp.parse(&$x){
                                Ok(out) => {
                                    result.push(Box::new(out));
                                    used |= i;
                                },
                                Err(_) => inp.rewind(before.clone()),
                            };
                        }
                        i <<= 1;
                    )*

                    if begin == inp.cursor() {
                        return Ok(result);
                    }
                }
            }
        }
    };
    
    (@count $($X:ident),*) => {
        {
            let mut types_count = 1;
            $(
                let _:$X;
                types_count += 1;
            )*
            types_count
        }
    };
}

impl_some_of_for_tuple!(a_ A_
                        b_ B_
                        c_ C_
                        d_ D_
                        e_ E_
                        f_ F_
                        g_ G_
                        h_ H_
                        i_ I_
                        j_ J_
                        k_ K_
                        l_ L_
                        m_ M_
                        n_ N_
                        o_ O_
                        p_ P_
                        q_ Q_
                        r_ R_
                        s_ S_ 
                        t_ T_
                        u_ U_ 
                        v_ V_
                        w_ W_
                        x_ X_
                        y_ Y_ 
                        z_ Z_ );

pub(crate) fn some_of<T>(parsers: T) -> SomeOf<T>{
   Ext(SomeOf_{parsers}) 
}

macro_rules! boxed_vec{
    ($($x:expr),*) =>{
        vec![$(Box::new($x),)* ]
    }
}