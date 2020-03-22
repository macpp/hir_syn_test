use std::path::Path;
use std::process::Command;
use std::io::prelude::*;
use syn::{File,parse_str,Attribute};
fn main() {
    let mut entries = vec![];
    for entry in std::fs::read_dir(std::env::current_dir().unwrap()).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            let last = path.components().last().unwrap().as_os_str().to_str().unwrap().to_owned();
            if last.starts_with("test_") {
                entries.push((path,last));
            }
        } 
    }
    entries.sort();
    for (path,last) in entries.into_iter() {
        println!("==============================");
        println!("TESTING: {:?}", path);
        let res = cwd_sandbox(|| test_crate(&path));

        println!("BASIC TEST RESULT: {}", res.is_some());
        if let Some(f) = res {
            match last.as_str() {
                "test_4_lint" => test_4_lint(&f),
                _ => println!("no additional tests")
            }
        }
    }
}

fn test_crate(dir: &Path) -> Option<File> {
    std::env::set_current_dir(&dir).unwrap();
    let out = Command::new("cargo")
            .args(&["rustc", "--", "-Zunpretty=hir-syn"]) //, ">", "hir_syn.rs"
            .output()
            .expect("failed to execute cargo rustc");
    let out = if !out.status.success() {
        println!("CARGO FAILED!!");
        let err = String::from_utf8_lossy(&out.stderr);
        println!("{}", err);
        return None;
    } else {
        String::from_utf8(out.stdout).expect("failed to convert output to utf8")
    };
    let mut file = std::fs::File::create("hir_syn.rs").unwrap();
    file.write_all(out.as_bytes()).unwrap();
    let f : File = match parse_str(&out) {
        Ok(x) => x,
        Err(x) => {
            println!("SYN PARSING ERROR: {}",x);
            
            return None;
        }
    };
    Some(f)
}

fn cwd_sandbox<T, F: Fn()-> T>(f: F) -> T{
    let cwd = std::env::current_dir().unwrap();
    let x : T = f();
    std::env::set_current_dir(&cwd).unwrap();
    x
}



#[derive(serde::Serialize,serde::Deserialize,Debug,Default)]
#[serde(default)]
struct HirSynData {
    ty: String,
    fname: String,
    line: usize,
    col: usize,
}

fn test_4_lint(file: &File) {
    let mut visitor = NoArcStringLint::default();
    syn::visit::visit_file(&mut visitor, file);
    if visitor.errors.len() > 0 {
        println!("found bad let statements: ");
        for x in visitor.errors.iter() {
            println!("{}",x);
        }
    }else {
        println!("no bad 'let' statements found");
    }
}

#[derive(Default,Debug)]
struct NoArcStringLint {
    errors: Vec<String>
}
impl<'ast> syn::visit::Visit<'ast> for NoArcStringLint {
    fn visit_local(&mut self, i: &'ast syn::Local)
    {
        if let Some((_eq,expr)) = i.init.as_ref() {
            match get_hir_data(expr.as_ref()) {
                Some(x) => {
                    let ty = x.ty.clone().replace(" ", "");
                    if ty == "std::sync::Arc<std::string::String>" {
                        self.errors.push(format!("found Arc<String>  {} line: {} column: {}", x.fname, x.line,x.col));
                    }
                },
                None => panic!("NO HIR DATA FOR LET EXPR")
            }
        }
        syn::visit::visit_local(self,i);
    }
}

trait HasAttrs {
    fn get_attrs(&self) -> &[Attribute];
}

fn get_hir_data<T: HasAttrs>(elem: &T) -> Option<HirSynData> {
    for attr in elem.get_attrs().iter() {
        if attr.path.is_ident("hir_syn") {
            let mut s = attr.tokens.to_string();
            if s.starts_with("("){
                s.remove(0);
            }
            if s.ends_with(")") {
                s.pop();
            }
            return Some(serde_json::from_str(&s).unwrap())
        }
    }
    None
}

macro_rules! impl_has_attrs {
    ($($x:ident),*) => {
        $ (
          impl HasAttrs for syn::$x {
            fn get_attrs(&self) -> &[syn::Attribute] {
                self.attrs.as_slice()
            }
          }
        )*
    }
}

impl_has_attrs!{
    ItemConst,
    ItemEnum,
    ItemExternCrate,
    ItemFn,
    ItemForeignMod,
    ItemImpl,
    ItemMacro,
    ItemMacro2,
    ItemMod,
    ItemStatic,
    ItemStruct,
    ItemTrait,
    ItemTraitAlias,
    ItemType,
    ItemUnion,
    ItemUse,
    ExprArray,
    ExprAssign,
    ExprAssignOp,
    ExprAsync,
    ExprAwait,
    ExprBinary,
    ExprBlock,
    ExprBox,
    ExprBreak,
    ExprCall,
    ExprCast,
    ExprClosure,
    ExprContinue,
    ExprField,
    ExprForLoop,
    ExprGroup,
    ExprIf,
    ExprIndex,
    ExprLet,
    ExprLit,
    ExprLoop,
    ExprMacro,
    ExprMatch,
    ExprMethodCall,
    ExprParen,
    ExprPath,
    ExprRange,
    ExprReference,
    ExprRepeat,
    ExprReturn,
    ExprStruct,
    ExprTry	,
    ExprTryBlock,
    ExprTuple,
    ExprType,
    ExprUnary,
    ExprUnsafe,
    ExprWhile,
    ExprYield,
    Local
}

macro_rules! impl_has_attrs_for_syn_enum {
    ($name:ident -> $($x:ident),*) => {
        impl HasAttrs for syn::$name {
            fn get_attrs(&self) -> &[syn::Attribute] {
                match self {
                    $ (syn::$name::$x(x) => x.get_attrs()), * ,
                    _ => unimplemented!()
                }
            }
          }
    }
}

impl_has_attrs_for_syn_enum !{
    Expr ->
    Array,
    Assign,
    AssignOp,
    Async,
    Await,
    Binary,
    Block,
    Box,
    Break,
    Call,
    Cast,
    Closure,
    Continue,
    Field,
    ForLoop,
    Group,
    If,
    Index,
    Let,
    Lit,
    Loop,
    Macro,
    Match,
    MethodCall,
    Paren,
    Path,
    Range,
    Reference,
    Repeat,
    Return,
    Struct,
    Try	,
    TryBlock,
    Tuple,
    Type,
    Unary,
    Unsafe,
    While,
    Yield
}

impl_has_attrs_for_syn_enum! {
    Item -> 
    Const,
    Enum,
    ExternCrate,
    Fn,
    ForeignMod,
    Impl,
    Macro,
    Macro2,
    Mod,
    Static,
    Struct,
    Trait,
    TraitAlias,
    Type,
    Union,
    Use
}

/*
usefull:
dump:
cargo rustc -- -Zunpretty=hir-json > hir.json

build rustc: 
 ./x.py check
 ./x.py build -i --stage 1 src/libstd


*/