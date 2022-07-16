use jest_workaround::TransformVisitor;
use std::path::PathBuf;
use swc_common::{chain, Mark};
use swc_ecma_transforms_base::resolver;
use swc_ecma_transforms_testing::{test, test_fixture};
use swc_plugin::ast::{as_folder, Fold};

fn tr() -> impl Fold {
    let unresolved_mark = Mark::new();
    let top_level_mark = Mark::new();

    chain!(
        resolver(unresolved_mark, top_level_mark, false),
        as_folder(TransformVisitor::new(unresolved_mark)),
    )
}

#[testing::fixture("tests/fixture/**/input.js")]
fn test(input: PathBuf) {
    let dir = input.parent().unwrap().to_path_buf();
    let output = dir.join("output.js");

    test_fixture(Default::default(), &|_| tr(), &input, &output);
}
