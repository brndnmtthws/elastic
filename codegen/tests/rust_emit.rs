#![feature(quote, rustc_private)]

extern crate syntax;
extern crate elastic_codegen;

use std::fs;
use std::fs::File;
use syntax::ast::*;
use syntax::parse::ParseSess;
use syntax::feature_gate::GatedCfgAttr;
use syntax::ext::base::ExtCtxt;
use syntax::ext::expand::ExpansionConfig;
use syntax::print::pprust;
use syntax::ext::quote;
use syntax::codemap::DUMMY_SP;
use syntax::parse::token::intern;
use elastic_codegen::emit::*;
use elastic_codegen::emit::rust::*;
use elastic_codegen::gen::rust::*;

#[test]
fn can_emit_rs_to_file() {
	//Build an execution context
	let ps = syntax::parse::ParseSess::new();
	let mut feature_gated_cfgs = vec![];
	let mut cx = syntax::ext::base::ExtCtxt::new(
		&ps, vec![],
		syntax::ext::expand::ExpansionConfig::default("qquote".to_string()),
		&mut feature_gated_cfgs
	);
	cx.bt_push(syntax::codemap::ExpnInfo {
		call_site: DUMMY_SP,
		callee: syntax::codemap::NameAndSpan {
			format: syntax::codemap::MacroBang(intern("")),
			allow_internal_unstable: false,
			span: None,
		}
	});
	
	//Function lifetime
	let lifetime = lifetime("'a");

	//Function signature
	let mut fun = build_fn("my_fun", vec![
		arg_ptr::<i32>("arg1", Mutability::MutMutable, Some(lifetime)),
		build_arg("arg2", build_ty_ptr("str", Mutability::MutImmutable, Some(lifetime)))
	]);

	//Function return type
	fun.set_return::<i32>();

	//Function body
	{
		let cx = &mut cx;
		fun.set_body(quote_block!(cx, {
			let x = 1;
			x
		}));
	}

	//Create an emitter
	let emitter = RustEmitter::new(cx);

	//Get a file ref
	let _ = fs::remove_file("tests/emit_results/can_emit_to_file.rs");
	let mut f = File::create("tests/emit_results/can_emit_to_file.rs").unwrap();

	let mut result = emitter.emit_str(&"//Autogenerated\n", &mut f);
	result = emitter.emit(&fun, &mut f);

	assert!(result.is_ok());
}