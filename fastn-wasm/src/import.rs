pub fn func00(name: &str) -> fastn_wasm::Ast {
    func(name, vec![], None)
}

pub fn func0(name: &str, result: fastn_wasm::Type) -> fastn_wasm::Ast {
    func(name, vec![], Some(result))
}

pub fn func1(name: &str, arg0: fastn_wasm::PL) -> fastn_wasm::Ast {
    func(name, vec![arg0], None)
}

pub fn func2(name: &str, arg0: fastn_wasm::PL, arg1: fastn_wasm::PL) -> fastn_wasm::Ast {
    func(name, vec![arg0, arg1], None)
}

pub fn func3(
    name: &str,
    arg0: fastn_wasm::PL,
    arg1: fastn_wasm::PL,
    arg2: fastn_wasm::PL,
) -> fastn_wasm::Ast {
    func(name, vec![arg0, arg1, arg2], None)
}

pub fn func4(
    name: &str,
    arg0: fastn_wasm::PL,
    arg1: fastn_wasm::PL,
    arg2: fastn_wasm::PL,
    arg3: fastn_wasm::PL,
) -> fastn_wasm::Ast {
    func(name, vec![arg0, arg1, arg2, arg3], None)
}

pub fn func1ret(name: &str, arg0: fastn_wasm::PL, ret: fastn_wasm::Type) -> fastn_wasm::Ast {
    func(name, vec![arg0], Some(ret))
}

pub fn func2ret(
    name: &str,
    arg0: fastn_wasm::PL,
    arg1: fastn_wasm::PL,
    ret: fastn_wasm::Type,
) -> fastn_wasm::Ast {
    func(name, vec![arg0, arg1], Some(ret))
}

pub fn func3ret(
    name: &str,
    arg0: fastn_wasm::PL,
    arg1: fastn_wasm::PL,
    arg2: fastn_wasm::PL,
    ret: fastn_wasm::Type,
) -> fastn_wasm::Ast {
    func(name, vec![arg0, arg1, arg2], Some(ret))
}

pub fn func4ret(
    name: &str,
    arg0: fastn_wasm::PL,
    arg1: fastn_wasm::PL,
    arg2: fastn_wasm::PL,
    arg3: fastn_wasm::PL,
    ret: fastn_wasm::Type,
) -> fastn_wasm::Ast {
    func(name, vec![arg0, arg1, arg2, arg3], Some(ret))
}

pub fn func(
    name: &str,
    params: Vec<fastn_wasm::PL>,
    result: Option<fastn_wasm::Type>,
) -> fastn_wasm::Ast {
    fastn_wasm::Ast::Import(fastn_wasm::Import {
        module: "fastn".to_string(),
        name: name.to_string(),
        desc: fastn_wasm::ImportDesc::Func(fastn_wasm::FuncDecl {
            name: Some(name.to_string()),
            params,
            result,
        }),
    })
}

#[derive(Debug)]
pub struct Import {
    pub module: String,
    pub name: String,
    pub desc: fastn_wasm::ImportDesc,
}

impl Import {
    pub fn to_doc(&self) -> pretty::RcDoc<'static> {
        fastn_wasm::group(
            "import".to_string(),
            Some(pretty::RcDoc::text(format!(
                "\"{}\" \"{}\"",
                self.module, self.name
            ))),
            self.desc.to_doc().group().nest(4),
        )
    }
}

#[derive(Debug)]
pub enum ImportDesc {
    Func(fastn_wasm::FuncDecl),
    Table(fastn_wasm::Table),
    Memory(fastn_wasm::Memory),
}

impl ImportDesc {
    pub fn to_doc(&self) -> pretty::RcDoc<'static> {
        match self {
            ImportDesc::Func(f) => f.to_doc(),
            ImportDesc::Table(t) => t.to_doc(),
            ImportDesc::Memory(m) => m.to_doc(),
        }
    }
}

#[cfg(test)]
mod test {
    use fastn_wasm::Import;

    #[track_caller]
    fn e(f: Import, s: &str) {
        let g = fastn_wasm::encode(&vec![fastn_wasm::Ast::Import(f)]);
        println!("got: {}", g);
        println!("expected: {}", s);
        assert_eq!(g, s);
    }

    #[test]
    fn test() {
        e(
            fastn_wasm::Import {
                module: "fastn".to_string(),
                name: "create_column".to_string(),
                desc: fastn_wasm::ImportDesc::Func(fastn_wasm::FuncDecl {
                    name: Some("create_column".to_string()),
                    params: vec![],
                    result: Some(fastn_wasm::Type::I32),
                }),
            },
            r#"(module (import "fastn" "create_column" (func $create_column (result i32))))"#,
        );
        e(
            fastn_wasm::Import {
                module: "js".to_string(),
                name: "table".to_string(),
                desc: fastn_wasm::ImportDesc::Table(fastn_wasm::Table {
                    ref_type: fastn_wasm::RefType::Func,
                    limits: fastn_wasm::Limits { min: 1, max: None },
                }),
            },
            r#"(module (import "js" "table" (table 1 funcref)))"#,
        );
    }
}
