extern crate text2binary;

use std::fs::File;

#[test]
// 文字列"001123456789abcdEF00"のテスト
fn test_convert() {
    let expect = vec![0x00, 0x11, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x00];
    let target = "001123456789abcdEF00".to_string();

    match text2binary::convert(&target) {
        Ok(ok) => assert_eq!(expect, ok),
        Err(err) => panic!("{}", err),
    }
}

#[test]
// 文字列"aBcDE"(末尾に0が付く場合)のテスト
fn test_convert_append_zero() {
    let expect = vec![0xAB, 0xCD, 0xE0];
    let target = "aBcDE".to_string();

    match text2binary::convert(&target) {
        Ok(ok) => assert_eq!(expect, ok),
        Err(err) => panic!("{}", err),
    }
}

#[test]
// 文字列に空白や改行コードが含まれていた場合のテスト
fn test_convert_space_and_break() {
    let expect = vec![0x12, 0x34, 0x56, 0x78, 0x9a];
    let target = r#"  12  34
    56 78  9 a "#.to_string();

    match text2binary::convert(&target) {
        Ok(ok) => assert_eq!(expect, ok),
        Err(err) => panic!("{}", err),
    }
}

#[test]
// 文字列が空であった場合のテスト
fn test_convert_empty() {
    let expect: Vec<u8> = vec![];

    match text2binary::convert(&"".to_string()) {
        Ok(ok) => assert_eq!(expect, ok),
        Err(err) => panic!("{}", err),
    }
}

#[test]
// 文字列に16進数表現以外の文字(む)が出現した場合のエラーテスト
fn test_convert_err() {
    match text2binary::convert(&"01む345".to_string()) {
        Ok(ok) => panic!("エラーが発生しませんでした。Ok={:?}", ok),
        Err(err) => assert_eq!("It is not a hexadecimal representation. target=む", err),
    }
}

#[test]
// 文字列Vecの場合のテスト
fn test_convert_line() {
    let line = vec!["123456".to_string(), "abc\r\ndef".to_string(), "012 fde".to_string(),];
    let expect = vec![0x12, 0x34, 0x56, 0xAB, 0xCD, 0xEF, 0x01, 0x2F, 0xDE,];

    match text2binary::convert_line(&line) {
        Ok(ok) => assert_eq!(expect, ok),
        Err(err) => panic!("{}", err),
    }
}

#[test]
// 文字列Vecのサイズが0の場合のテスト
fn test_convert_line_zero() {
    let line: Vec<String> = vec![];
    let expect: Vec<u8> = vec![];

    match text2binary::convert_line(&line) {
        Ok(ok) => assert_eq!(expect, ok),
        Err(err) => panic!("{}", err),
    }
}

#[test]
// テキストファイルの場合のテスト
fn test_convert_file() {
    let file = File::open("./tests/target.txt");
    let expect = vec![0x00, 0x12, 0x34, 0x56, 0x78, 0xAB, 0xCD, 0xEF, 0x00];

    match file {
        Ok(file) => {
            match text2binary::convert_file(&file) {
                Ok(ok) => assert_eq!(expect, ok),
                Err(err) => panic!("{}", err),
            }
        },
        Err(err) => panic!("{}", err),
    }
}

#[test]
// テキストファイル(空行含む)の場合のテスト
fn test_convert_file_blank_line() {
    let file = File::open("./tests/target2.txt");
    let expect = vec![0xCD, 0xEF, 0x00, 0x56, 0x78, 0xAB, 0x00, 0x12, 0x34];

    match file {
        Ok(file) => {
            match text2binary::convert_file(&file) {
                Ok(ok) => assert_eq!(expect, ok),
                Err(err) => panic!("{}", err),
            }
        },
        Err(err) => panic!("{}", err),
    }
}