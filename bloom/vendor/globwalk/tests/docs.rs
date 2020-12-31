extern crate docmatic;

#[test]
fn readme_test() {
    let readme = std::path::Path::new(file!()).canonicalize().unwrap();
    let readme = readme.parent().unwrap().parent().unwrap().join("README.md");
    let readme = readme.to_str().unwrap();

    docmatic::assert_file(readme);
}
