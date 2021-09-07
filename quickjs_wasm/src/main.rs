mod hand_written;

use hand_written::NativeModule;

quickrs::module_init!(NativeModule);
use quickrs::{
    BuiltinLoader, BuiltinResolver, Context, FileResolver, Func, ModuleLoader, Runtime, ScriptLoader,
};

fn print(msg: String) {
    println!("{}", msg);
}

#[repr(C)]
pub enum Message {
    RunJS(String)
}


pub struct Node {
    rt: Runtime
}

impl Node {
    pub fn new() -> Self {

        let resolver = (
            BuiltinResolver::default()
                .with_module("react")
                .with_module("bundle/native_module"),
            FileResolver::default()
                .with_path("./react-dom/")
                .with_path("../../target/debug")
                .with_native(),
        );
        let loader = (
            BuiltinLoader::default().with_module("react", include_str!("react.js")),
            ModuleLoader::default().with_module("bundle/native_module", NativeModule),
            ScriptLoader::default(),
        );
        let mut rt = Runtime::new().unwrap();
        rt.set_loader(resolver, loader);
        // 
        let node = Node {
            rt
        };
        node
    }

    pub fn execute_javascript(&mut self, code: &str) -> i64 {
        let ctx = Context::full(&self.rt).unwrap();
        ctx.with(|ctx| {
            let global = ctx.globals();
            global.set("print", Func::new("print", print)).unwrap();
            match ctx.compile("handler", code,
                ) {
                    Ok(_) => println!("Compiled with no errors"),
                    Err(err) => println!("ERROR: {}", err),
                }
        });
        0
    }
}

#[export_name = "execute_javascript"]
pub extern "C" fn execute_javascript(ptr: *const u8, len: usize) -> i64 {
    let mut node = Node::new();
    let slice = unsafe {
        std::slice::from_raw_parts(ptr, len)
    };
    let s = String::from_utf8(slice.to_vec()).unwrap();
    node.execute_javascript(&s);
    0
}

pub fn main() {
    for entry in std::fs::read_dir(".").unwrap() {
        println!("Entry: {:?}", entry);
    }
    let p = std::path::Path::new("./react-dom/module.js");
    println!("Exists: {}", p.exists());
    let s = r#"
    import * as m from "react-dom.development.js"
    print(String(1+1))
    "#;
    execute_javascript(s.as_ptr(), s.len());
    // println!("Messages: {}", unsafe { get_messages() });
   
}
