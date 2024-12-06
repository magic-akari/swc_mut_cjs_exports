use std::path::PathBuf;
use swc_core::{
    common::Mark,
    ecma::{
        ast::Pass,
        transforms::{
            base::resolver,
            testing::{test, test_fixture},
        },
        visit::visit_mut_pass,
    },
};
use swc_mut_cjs_exports::TransformVisitor;

fn tr() -> impl Pass {
    let unresolved_mark = Mark::new();
    let top_level_mark = Mark::new();

    (
        resolver(unresolved_mark, top_level_mark, false),
        visit_mut_pass(TransformVisitor::new(unresolved_mark)),
    )
}

#[testing::fixture("tests/fixture/**/input.js")]
fn test(input: PathBuf) {
    let dir = input.parent().unwrap().to_path_buf();
    let output = dir.join("output.js");

    test_fixture(
        Default::default(),
        &|_| tr(),
        &input,
        &output,
        Default::default(),
    );
}
