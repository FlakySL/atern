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

/*
struct SomeOf_<T>{
    parsers: T,
}

type SomeOf<T> = Ext<SomeOf_<T>>;

pub fn some_of<T>(parsers: T) -> SomeOf<T>{
   SomeOf{parsers} 
}

macro_rules! impl_some_of_for_tuple {
    () => {};
    ($Head:ident $($X:ident)*) => {
        impl_some_of_for_tuple!($($X)*);
        impl_some_of_for_tuple!(@recur $Head $($X)*);
    };
    (@recur $Head:ident) => {
        impl<'src, I, E, $Head, O> ExtParser<'src, I, O, E> for SomeOf_<($Head,)>
        where
            I: Input<'src>,
            E: ParserExtra<'src, I>,
            $Head:  Parser<'src, I, O, E>,
        {
            fn parse(&self, inp: &mut InputRef<'src, '_, I, E>) -> Result<Vec<Box<O>>, E>  {
                let mut result = vec![];
                let before = inp.save();
                
                if let Ok(out) = self.parsers.0.parse(inp) {
                    result.push(Box::new(out));
                }
                result
            }
        }
    };
    
    (@recur $Head:ident $($X:ident)+) => {
        #[allow(unused_variables, non_snake_case)]
        impl<'src, I, E, $Head, $($X),*, O> ExtParser<'src, I, O, E> for SomeOf_<($($X,)*)>
        where
            I: Input<'src>,
            E: ParserExtra<'src, I>,
            $($X: Parser<'src, I, O, E>),*
        {
            fn parse(&self, inp: &mut InputRef<'src, '_, I, E>) -> Result<Vec<Box<O>>, E> {
                let count: u32 = impl_some_of_for_tuple! (@count $($X),*);
                let result = Vec::with_capacity::<Box<O>>(count);
                let mut used = vec![false; count];

                loop{
                    let mut parsed = false;
                    for i in 0..count {
                        if !used[i] {
                            let before = inp.save();
                            match self.parsers.i.parse(inp){
                                Ok(out) => {
                                    result.push(Box::new(out));
                                    used[i] = true;
                                    parsed = true;
                                },
                                Err(()) => inp.rewind(before.clone()),
                            };
                        }
                    }
                    if !parsed || result.size() == result.capacity() {
                        Ok(result)
                    }
                }
            }
        }
    };
    
    (@count $($X:ident),*) => {
        {
            let types_count = 1;
            $(
                let _:$X;
                types_count += 1;
            )*
            types_count
        }
    };
}

impl_some_of_for_tuple!(A_ B_ C_ D_ E_ F_ G_ H_ I_ J_ K_ L_ M_ N_ O_ P_ Q_ R_ S_ T_ U_ V_ W_ X_ Y_ Z_);


fn handle_parsers<'src, I, O, E, M>(input: &mut I, parsers: Vec<PPtr>) -> Vec<O>
where 
    PPtr: RcRa<Box<OrNot<dyn Parser<I,O,E>>>>
    I: Input<'src>,
    E: ParserExtra<'src, I>,
    M: Mode
{
    let mut result: Vec<O> = vec![];
    let mut tmp: Vec<PPtr> = vec![];
    
    loop {
        for parser in parsers {
            let before = input.save();
            match parser.go<M>(input) {
                Ok(Some(r)) => result.push(r),
                Ok(None) => {
                    input.rewind(before);
                    tmp.push(parser);
                },
                _ => panic!("Unreacheable"),
            };
        }
        if tmp.empty() || parsers.empty(){
            break;
        }
        parsers = tmp;
        tmp = vec![];
    }
    result
}
*/