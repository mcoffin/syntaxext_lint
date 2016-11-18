//! Provides some common lints for syntax extensions.
#![feature(plugin_registrar)]
#![feature(rustc_private)]

extern crate syntax;
#[macro_use]
extern crate rustc;
extern crate rustc_plugin;

use rustc::lint::*;
use rustc::hir as ast;
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
}

impl LateLintPass for Pass {
    fn check_expr(&mut self, cx: &LateContext, expr: &ast::Expr) {
        match expr.node {
            ast::ExprPath(None, ref path) => {
                let name = path.segments.last().unwrap().name.as_str();
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
pub fn register_plugins(reg: &mut rustc_plugin::Registry) {
    reg.register_late_lint_pass(Box::new(Pass::new()) as LateLintPassObject);
}
