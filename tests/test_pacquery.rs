use rstest::rstest;
use serde_json::Value;
use tempfile::TempDir;

pub mod fixtures;

use fixtures::{invalid_dbpath, no_reverse_deps, reverse_deps, Package};

#[rstest]
#[should_panic]
fn test_invalid_dbpath(invalid_dbpath: (Vec<String>, Option<String>)) {
    let pkgnames = invalid_dbpath.0;
    let dbpath = invalid_dbpath.1;
    pacquery::run(pkgnames, dbpath, vec![]).unwrap();
}

#[rstest]
fn test_reverse_deps(reverse_deps: (Vec<String>, Option<String>, Vec<String>, TempDir)) {
    let pkgnames = reverse_deps.0;
    let pkgname = &pkgnames[0];

    let res = pacquery::run(vec![pkgname.to_string()], reverse_deps.1, reverse_deps.2).unwrap();

    let v: Value = serde_json::from_str(res.trim()).unwrap();
    dbg!(&v);
    assert_eq!(pkgname.to_string(), v[0]["name"]);
    assert_eq!(serde_json::json!([pkgnames[1]]), v[0]["required_by"]);
}

#[rstest]
fn test_no_reverse_deps(no_reverse_deps: (Vec<Package>, Option<String>, Vec<String>, TempDir)) {
    let packages = no_reverse_deps.0;

    let res = pacquery::run(
        vec![packages[0].name.clone()],
        no_reverse_deps.1,
        no_reverse_deps.2,
    )
    .unwrap();
    let v: Value = serde_json::from_str(res.trim()).unwrap();

    assert_eq!(packages[0].name, v[0]["name"]);
    assert_eq!(packages[0].version, v[0]["version"]);

    // No packages required this one
    let required_by = &v[0]["required_by"];
    dbg!(required_by);
    assert_eq!(serde_json::json!([]), v[0]["required_by"]);
}
