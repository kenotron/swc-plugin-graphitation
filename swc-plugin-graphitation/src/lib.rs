use graphql_parser::query::{parse_query};
use swc_plugin::{ast::*, plugin_transform};

pub struct TransformVisitor;

impl VisitMut for TransformVisitor {
    // VisitMut fns: https://rustdoc.swc.rs/swc_ecma_visit/trait.VisitMut.html
    // AST: https://rustdoc.swc.rs/swc_ecma_ast/index.html
    fn visit_mut_tpl(&mut self, n: &mut Tpl) {
        if n.quasis.len() > 0 {
            let query_str = n.quasis[0].raw.to_string();
            let ast = parse_query::<String>(&query_str).unwrap();
            for def in ast.definitions {
                println!("value: {}", def);
            }
        }
    }
}

/// Refer swc_plugin_macro how plugin_transform macro works internally.
#[plugin_transform]
pub fn process_transform(program: Program, _plugin_config: String, _context: String) -> Program {
    program.fold_with(&mut as_folder(TransformVisitor))
}
