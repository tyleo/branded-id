use crate::internal::split_type_str;

#[test]
fn unqualified_name_test() {
    let actual = split_type_str("MTest");
    let expected = "MTest";
    assert_eq!(actual, expected);
}

#[test]
fn qualified_path_test() {
    let actual = split_type_str("branded_id::tests::util::m_test::MTest");
    let expected = "MTest";
    assert_eq!(actual, expected);
}

#[test]
fn single_generic_test() {
    let actual = split_type_str("a::b::Foo<c::d::Bar>");
    let expected = "Foo<Bar>";
    assert_eq!(actual, expected);
}

#[test]
fn multi_generic_test() {
    let actual = split_type_str("a::Foo<b::Bar, c::Baz>");
    let expected = "Foo<Bar, Baz>";
    assert_eq!(actual, expected);
}

#[test]
fn nested_generic_test() {
    let actual = split_type_str("a::Foo<b::Bar<c::Inner>>");
    let expected = "Foo<Bar<Inner>>";
    assert_eq!(actual, expected);
}

#[test]
fn raw_pointer_test() {
    let actual = split_type_str("*const a::b::T");
    let expected = "*const T";
    assert_eq!(actual, expected);
}

#[test]
fn empty_test() {
    let actual = split_type_str("");
    let expected = "";
    assert_eq!(actual, expected);
}
