use quote::{ToTokens, Tokens};
use syn::punctuated::Pair;
use syn::synom::Synom;
use syn::{Expr, ExprStruct, Ident, Lit, Member};

pub struct Metadata {
    pub description: String,
    pub method: String,
    pub name: String,
    pub path: String,
    pub rate_limited: bool,
    pub requires_authentication: bool,
}

impl From<ExprStruct> for Metadata {
    fn from(expr: ExprStruct) -> Self {
        let mut description = None;
        let mut method = None;
        let mut name = None;
        let mut path = None;
        let mut rate_limited = None;
        let mut requires_authentication = None;

        for field in expr.fields {
            let Member::Named(identifier) = field.member;

            match identifier.as_ref() {
                "description" => {
                    let Expr::Lit(expr_lit) = field.expr;
                    let Lit::Str(lit_str) = expr_lit.lit;
                    description = Some(lit_str.value());
                }
                "method" => {
                    let Expr::Path(expr_path) = field.expr;
                    let path = expr_path.path;
                    let segments = path.segments;
                    if segments.len() != 1 {
                        panic!("ruma_api! expects a one component path for `metadata` `method`");
                    }
                    let pair = segments.first().unwrap(); // safe because we just checked
                    let Pair::End(method_name) = pair;
                    method = Some(method_name.ident.to_string());
                }
                "name" => {
                    let Expr::Lit(expr_lit) = field.expr;
                    let Lit::Str(lit_str) = expr_lit.lit;
                    name = Some(lit_str.value());
                }
                "path" => {
                    let Expr::Lit(expr_lit) = field.expr;
                    let Lit::Str(lit_str) = expr_lit.lit;
                    path = Some(lit_str.value());
                }
                "rate_limited" => {
                    let Expr::Lit(expr_lit) = field.expr;
                    let Lit::Bool(lit_bool) = expr_lit.lit;
                    rate_limited = Some(lit_bool.value)
                }
                "requires_authentication" => {
                    let Expr::Lit(expr_lit) = field.expr;
                    let Lit::Bool(lit_bool) = expr_lit.lit;
                    requires_authentication = Some(lit_bool.value)
                }
                _ => panic!("ruma_api! metadata included unexpected field"),
            }
        }

        Metadata {
            description: description.expect("ruma_api! `metadata` is missing `description`"),
            method: method.expect("ruma_api! `metadata` is missing `method`"),
            name: name.expect("ruma_api! `metadata` is missing `name`"),
            path: path.expect("ruma_api! `metadata` is missing `path`"),
            rate_limited: rate_limited.expect("ruma_api! `metadata` is missing `rate_limited`"),
            requires_authentication: requires_authentication
                .expect("ruma_api! `metadata` is missing `requires_authentication`"),
        }
    }
}
