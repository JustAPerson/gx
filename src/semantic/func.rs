// gx language implementation
// Copyright (C) 2015 Alex Iadicicco <http://ajitek.net>
//
// For licensing information, refer to the COPYING file
// in the project root

//! Semantic information for units

use dump::*;
use frontend::tree;
use semantic::*;
use semantic::types::TypeRef;

pub struct Func<'a> {
    pub name:  String,
    pub proto: FuncPrototype<'a>,
}

pub struct FuncPrototype<'a> {
    pub ret:    Option<TypeRef<'a>>,
    pub params: Vec<FuncParam<'a>>,
}

pub struct FuncParam<'a> {
    pub name: Option<String>,
    pub typ:  TypeRef<'a>,
}


impl<'a> Func<'a> {
    pub fn from_tree(f: &tree::FuncDecl) -> SemResult<Func<'a>> {
        let mut params = Vec::with_capacity(f.params.len());
        for p in &f.params {
            let typ = try!(TypeRef::from_tree(&p.typ));
            for id in &p.ids {
                params.push(FuncParam { name: Some(id.clone()), typ: typ.clone()  });
            }
        }
        let ret = match f.ret {
            Some(ref t) => Some(try!(TypeRef::from_tree(t))),
            None => None,
        };

        Ok(Func{
            name: f.name.clone(),
            proto: FuncPrototype {
                ret: ret,
                params: params,
            },
        })
    }
}


// TODO aji pls fix
impl<'a> Dumpable for Func<'a> {
    fn dump(&self, d: &mut DumpContext) {
        let mut s = String::new();
        for v in &self.proto.params {
            match v.name {
                Some(ref name) => { s = s + &format!("{}:{:?},", name, v.typ) },
                None => { s = s + &format!("{:?},", v.typ) },
            }
        }
        match self.proto.ret {
            Some(ref ret) => d.push(format!("fn {}({}): {:?}", self.name, s, ret)),
            None => d.push(format!("fn {}({})", self.name, s)),
        }
        d.pop();
    }
}
