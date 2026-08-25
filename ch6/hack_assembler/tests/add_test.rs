use std::{fs::read_to_string, path::PathBuf};

use hack_assembler::hack::assembler;

#[test]
fn add_asm() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/Add.asm");
    let result = assembler(path.to_str().unwrap()).unwrap();

    let expected_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/Add.hack");
    assert_eq!(result, read_to_string(expected_path).unwrap())
}
