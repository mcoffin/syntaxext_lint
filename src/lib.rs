//! Provides some common lints for syntax extensions.
#![feature(plugin_registrar)]
#![feature(rustc_private)]

extern crate syntax;
#[macro_use]
extern crate rustc;
extern crate rustc_front;

use rustc_front::hir as ast;
use rustc::lint::{Context, LintPass, LintPassObject, LintArray};
use rustc::plugin;
use std::ops::Deref;

declare_lint!(DUMMY_SPAN,
              Warn,
              "detects uses of DUMMY_SP");

struct Pass;

impl Pass {
    fn new() -> Pass {
        Pass
    }
}

impl LintPass for Pass {
    fn get_lints(&self) -> LintArray {
        lint_array!(DUMMY_SPAN)
    }

    fn check_expr(&mut self, cx: &Context, expr: &ast::Expr) {
        match expr.node {
            ast::ExprPath(None, ref path) => {
                let name = path.segments.last().unwrap().identifier.name.as_str();
                if name.deref() == "DUMMY_SP" {
                    cx.span_lint(DUMMY_SPAN, expr.span,
                                 "usage of 'DUMMY_SP' is discouraged");
                }
            },
            _ => {},
        }
    }
}

#[plugin_registrar]
pub fn register_plugins(reg: &mut plugin::Registry) {
    reg.register_lint_pass(Box::new(Pass::new()) as LintPassObject);
}
