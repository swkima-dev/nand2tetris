use std::{fs::read_to_string, path::PathBuf};

use hack_assembler::hack::assembler;

fn assert_assembles_to(asm: &str, expected_hack: &str) {
    let tests_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests");

    let result = assembler(tests_dir.join(asm).to_str().unwrap()).unwrap();
    let expected = read_to_string(tests_dir.join(expected_hack)).unwrap();

    let result: Vec<&str> = result.lines().collect();
    let expected: Vec<&str> = expected.lines().collect();

    for i in 0..result.len().max(expected.len()) {
        assert_eq!(result.get(i), expected.get(i), "{asm}: line {}", i + 1);
    }
}

#[test]
fn add_asm() {
    assert_assembles_to("Add.asm", "Add.hack");
}

#[test]
fn pongl_asm() {
    assert_assembles_to("PongL.asm", "PongL.hack");
}

#[test]
fn maxl_asm() {
    assert_assembles_to("MaxL.asm", "MaxL.hack");
}

#[test]
fn max_asm() {
    assert_assembles_to("Max.asm", "Max.hack");
}

#[test]
fn rect_asm() {
    assert_assembles_to("Rect.asm", "Rect.hack");
}
