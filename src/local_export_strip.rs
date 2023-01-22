use indexmap::IndexMap;
use swc_core::{
    common::{collections::AHashSet, util::take::Take, Mark, Span, DUMMY_SP},
    ecma::{
        ast::*,
        atoms::{js_word, JsWord},
        utils::{find_pat_ids, private_ident, ExprFactory},
        visit::{noop_visit_mut_type, VisitMut, VisitMutWith},
    },
};

use crate::utils::{ident_from_export_name, re_export};

type Export = IndexMap<(JsWord, Span), Ident>;

#[derive(Debug, Default)]
pub(crate) struct LocalExportStrip {
    pub(crate) has_export_assign: bool,
    pub(crate) export: Export,
    pub(crate) export_decl_id: AHashSet<Id>,
    export_default: Option<Stmt>,
    pub unresolved_mark: Mark,
}

impl VisitMut for LocalExportStrip {
    noop_visit_mut_type!();

    fn visit_mut_module_items(&mut self, n: &mut Vec<ModuleItem>) {
        let mut list = Vec::with_capacity(n.len());

        for item in n.drain(..) {
            match item {
                ModuleItem::Stmt(stmt) => list.push(stmt.into()),

                ModuleItem::ModuleDecl(mut module_decl) => {
                    // collect link meta
                    module_decl.visit_mut_with(self);

                    // emit stmt
                    match module_decl {
                        ModuleDecl::ExportDecl(ExportDecl { decl, .. }) => {
                            list.push(Stmt::Decl(decl).into());
                        }
                        ModuleDecl::ExportNamed(NamedExport { src: None, .. }) => continue,
                        ModuleDecl::ExportNamed(export)
                            if matches!(
                                export.specifiers.first(),
                                Some(ExportSpecifier::Named(_))
                            ) =>
                        {
                            list.push(re_export(export, self.unresolved_mark).into())
                        }
                        ModuleDecl::ExportDefaultDecl(ExportDefaultDecl {
                            decl:
                                decl @ (DefaultDecl::Class(ClassExpr {
                                    ident: Some(..), ..
                                })
                                | DefaultDecl::Fn(FnExpr {
                                    ident: Some(..), ..
                                })),
                            ..
                        }) => match decl {
                            DefaultDecl::Class(class_expr) => list.extend(
                                class_expr
                                    .as_class_decl()
                                    .map(|decl| Stmt::Decl(Decl::Class(decl)))
                                    .map(Into::into),
                            ),
                            DefaultDecl::Fn(fn_expr) => list.extend(
                                fn_expr
                                    .as_fn_decl()
                                    .map(|decl| Stmt::Decl(Decl::Fn(decl)))
                                    .map(Into::into),
                            ),
                            _ => unreachable!(),
                        },
                        ModuleDecl::ExportDefaultExpr(..) => {
                            list.extend(self.export_default.take().map(From::from))
                        }
                        ModuleDecl::TsExportAssignment(..) => {
                            self.has_export_assign = true;
                            list.push(module_decl.into());
                        }
                        _ => list.push(module_decl.into()),
                    };
                }
            };
        }

        *n = list;
    }

    /// ```javascript
    /// export const foo = 1, bar = 2, { baz } = { baz: 3 };
    /// export let a = 1, [b] = [2];
    /// export function x() {}
    /// export class y {}
    /// ```
    /// ->
    /// ```javascript
    /// const foo = 1, bar = 2, { baz } = { baz: 3 };
    /// let a = 1, [b] = [2];
    /// function x() {}
    /// class y {}
    /// ```
    fn visit_mut_export_decl(&mut self, n: &mut ExportDecl) {
        match &n.decl {
            Decl::Class(ClassDecl { ident, .. }) | Decl::Fn(FnDecl { ident, .. }) => {
                let ident = ident.clone();

                self.export.insert((ident.sym.clone(), ident.span), ident);
            }

            Decl::Var(v) => {
                let ids = find_pat_ids::<_, Ident>(&v.decls);

                self.export_decl_id.extend(ids.iter().map(Ident::to_id));

                self.export.extend(ids.into_iter().map(|id| {
                    let ident = id.clone();

                    ((id.sym, id.span), ident)
                }));
            }
            _ => {}
        };
    }

    /// ```javascript
    /// export { foo, foo as bar, foo as "baz" };
    /// export { "foo", foo as bar, "foo" as "baz" } from "mod";
    /// export * as foo from "mod";
    /// export * as "bar" from "mod";
    /// ```
    fn visit_mut_named_export(&mut self, n: &mut NamedExport) {
        if n.type_only {
            return;
        }

        let (re_exported, NamedExport { specifiers, .. }) = match n.src {
            Some(_) => (true, n.clone()),
            None => (false, n.take()),
        };

        self.export.extend(
            specifiers
                .into_iter()
                .filter(|e| match e {
                    ExportSpecifier::Namespace(..) => false,
                    _ => true,
                })
                .map(|e| match e {
                    ExportSpecifier::Namespace(..) => unreachable!(),
                    ExportSpecifier::Default(..) => {
                        unreachable!("`export foo` without src is invalid")
                    }
                    ExportSpecifier::Named(ExportNamedSpecifier { orig, exported, .. }) => {
                        let orig = ident_from_export_name(orig);

                        if let Some(exported) = exported {
                            let exported = match exported {
                                ModuleExportName::Ident(Ident { span, sym, .. }) => (sym, span),
                                ModuleExportName::Str(Str { span, value, .. }) => (value, span),
                            };

                            if re_exported {
                                let sym = exported.0.clone();

                                (
                                    exported,
                                    Ident {
                                        span: DUMMY_SP,
                                        sym,
                                        optional: false,
                                    },
                                )
                            } else {
                                (exported, orig)
                            }
                        } else {
                            let exported = orig.sym.clone();
                            ((exported, orig.span), orig)
                        }
                    }
                }),
        )
    }

    /// ```javascript
    /// export default class foo {};
    /// export default class {};
    /// export default function bar () {};
    /// export default function () {};
    /// ```
    /// ->
    /// ```javascript
    /// class foo {};
    /// class _default {};
    /// function bar () {};
    /// function _default () {};
    /// ```
    fn visit_mut_export_default_decl(&mut self, n: &mut ExportDefaultDecl) {
        match &mut n.decl {
            DefaultDecl::Class(class_expr) => {
                if let Some(ident) = class_expr.ident.clone() {
                    self.export.insert((js_word!("default"), n.span), ident);
                }
            }
            DefaultDecl::Fn(fn_expr) => {
                if let Some(ident) = fn_expr.ident.clone() {
                    self.export.insert((js_word!("default"), n.span), ident);
                }
            }
            DefaultDecl::TsInterfaceDecl(_) => {}
        }
    }

    /// ```javascript
    /// export default foo;
    /// export default 1
    /// ```
    /// ->
    /// ```javascript
    /// var _default = foo;
    /// var _default = 1;
    /// ```
    fn visit_mut_export_default_expr(&mut self, n: &mut ExportDefaultExpr) {
        let ident = private_ident!(n.span, "_default");

        self.export
            .insert((js_word!("default"), n.span), ident.clone());

        self.export_default = Some(Stmt::Decl(
            n.expr
                .take()
                .into_var_decl(VarDeclKind::Const, ident.into())
                .into(),
        ));
    }
}
