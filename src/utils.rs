use swc_core::{
    common::{Span, Spanned, DUMMY_SP},
    ecma::{
        ast::*,
        atoms::JsWord,
        utils::{member_expr, private_ident, quote_ident, quote_str, ExprFactory, IdentExt},
    },
};

/// {
///     "key": ident,
/// }
pub(crate) struct ObjPropKeyIdent(JsWord, Span, Ident);

impl From<((JsWord, Span), Ident)> for ObjPropKeyIdent {
    fn from(((key, span), ident): ((JsWord, Span), Ident)) -> Self {
        Self(key, span, ident)
    }
}

impl From<(JsWord, Span, Ident)> for ObjPropKeyIdent {
    fn from((key, span, ident): (JsWord, Span, Ident)) -> Self {
        Self(key, span, ident)
    }
}

impl Spanned for ObjPropKeyIdent {
    fn span(&self) -> Span {
        self.1
    }
}

impl ObjPropKeyIdent {
    pub fn key(&self) -> &JsWord {
        &self.0
    }

    pub fn ident(&self) -> &Ident {
        &self.2
    }
}

/// ```javascript
/// {
///     get() { return ident; }
/// }
/// ```
pub(crate) fn prop_method_getter(ident: Ident) -> Prop {
    let key = quote_ident!("get").into();

    MethodProp {
        key,
        function: ident.into_lazy_fn(Default::default()).into(),
    }
    .into()
}

/// ```javascript
/// {
///     set(v) { ident = v; }
/// }
/// ```
pub(crate) fn prop_method_setter(ident: Ident) -> Prop {
    let key = quote_ident!("set").into();

    let setter_param = private_ident!("v");
    let params = vec![setter_param.clone().into()];

    let body = BlockStmt {
        span: DUMMY_SP,
        stmts: vec![setter_param
            .make_assign_to(op!("="), Pat::Ident(ident.clone().into()).into())
            .into_stmt()],
    };

    MethodProp {
        key,
        function: Function {
            params,
            decorators: Default::default(),
            span: DUMMY_SP,
            body: Some(body),
            is_generator: false,
            is_async: false,
            type_params: None,
            return_type: None,
        }
        .into(),
    }
    .into()
}

/// Creates
///
///```js
///
///  Object.defineProperty(target, prop_name, {
///      ...props
///  });
/// ```
pub(super) fn object_define_property(
    target: ExprOrSpread,
    prop_name: ExprOrSpread,
    descriptor: ExprOrSpread,
) -> Expr {
    member_expr!(DUMMY_SP, Object.defineProperty)
        .as_call(DUMMY_SP, vec![target, prop_name, descriptor])
}

pub(crate) fn object_define_enumerable_configurable(
    target: ExprOrSpread,
    prop_name: ExprOrSpread,
    getter: PropOrSpread,
    setter: PropOrSpread,
) -> Expr {
    object_define_property(
        target,
        prop_name,
        ObjectLit {
            span: DUMMY_SP,
            props: vec![
                PropOrSpread::Prop(Box::new(
                    KeyValueProp {
                        key: quote_ident!("enumerable").into(),
                        value: Box::new(true.into()),
                    }
                    .into(),
                )),
                getter,
                setter,
                PropOrSpread::Prop(Box::new(
                    KeyValueProp {
                        key: quote_ident!("configurable").into(),
                        value: Box::new(true.into()),
                    }
                    .into(),
                )),
            ],
        }
        .as_arg(),
    )
}

pub(crate) fn emit_export_stmts(exports: Ident, prop_list: Vec<ObjPropKeyIdent>) -> Vec<Stmt> {
    prop_list
        .into_iter()
        .map(|obj_prop| {
            object_define_enumerable_configurable(
                exports.clone().as_arg(),
                quote_str!(obj_prop.span(), obj_prop.key()).as_arg(),
                prop_method_getter(obj_prop.ident().clone()).into(),
                prop_method_setter(obj_prop.ident().clone()).into(),
            )
            .into_stmt()
        })
        .collect()
}

pub(crate) fn key_from_export_name(n: &ModuleExportName) -> (JsWord, Span) {
    match n {
        ModuleExportName::Ident(ident) => (ident.sym.clone(), ident.span),
        ModuleExportName::Str(str) => (str.value.clone(), str.span),
    }
}

pub(crate) fn local_ident_from_export_name(n: ModuleExportName) -> Ident {
    match n {
        ModuleExportName::Ident(ident) => ident.private(),
        ModuleExportName::Str(str) => match Ident::verify_symbol(&str.value) {
            Ok(_) => private_ident!(str.value),
            Err(s) => private_ident!(s),
        },
    }
}
