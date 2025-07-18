use std::string::ToString;

pub fn convert(text: &String) -> Result<Vec<u8>, String> {
    // スペースと改行を除いたテキストに整形
    let mut target = format_text(&text).chars().collect::<Vec<char>>();

    let mut byte_vec = vec![];
    while target.len() > 0 {
        // 先頭2文字をバイトに変換してVecに追加する
        let byte = str_2_binary(target[..2].to_vec())?;
        byte_vec.push(byte);

        // テキストの次の2文字を先頭にする
        target = target[2..].to_vec();
    }

    Ok(byte_vec.clone())
}

fn format_text(text: &String) -> String {
    let mut format_text = "".to_string();

    // 改行コードとスペースを除く
    for c in text.chars() {
        match c {
            ' ' | '\n' | '\r' => continue,
            _ => format_text.push(c),
        }
    }

    // 文字数が奇数の場合、末尾に0を加える
    if format_text.len() % 2 != 0 {
        format_text.push('0');
    }

    format_text
}

fn str_2_binary(target: Vec<char>) -> Result<u8, String> {
    // 文字が2文字以外の場合はパニック
    if target.len() != 2 {
        return Err(format!("Target is not two characters. target={:?}", target));
    }

    // 上位バイトと下位バイトにわける
    let upper = char_2_hex(target[0])?;
    let lower = char_2_hex(target[1])?;

    // 上位バイトは4桁左にシフトし、下位バイトと加算する
    Ok((upper << 4) + lower)
}

fn char_2_hex(target: char) -> Result<u8, String> {
    match target {
        '0' => Ok(0),
        '1' => Ok(1),
        '2' => Ok(2),
        '3' => Ok(3),
        '4' => Ok(4),
        '5' => Ok(5),
        '6' => Ok(6),
        '7' => Ok(7),
        '8' => Ok(8),
        '9' => Ok(9),
        'A' | 'a' => Ok(10),
        'B' | 'b' => Ok(11),
        'C' | 'c' => Ok(12),
        'D' | 'd' => Ok(13),
        'E' | 'e' => Ok(14),
        'F' | 'f' => Ok(15),

        // 16進数表現以外が含まれていた場合はパニック
        _ => return Err(format!("It is not a hexadecimal representation. target={}", target)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // 全て除外しない文字列だった場合のテスト
    fn test_format_text_normal() {
        let expect = "1234567890";
        assert_eq!(expect, format_text(&"123456789".to_string()))
    }

    #[test]
    // 全て除外しない文字列(全角文字含む)だった場合のテスト
    fn test_format_text_wide() {
        let expect = "1２３4";
        assert_eq!(expect, format_text(&"1２３4".to_string()))
    }

    #[test]
    // 末尾に0が付与される場合のテスト
    fn test_format_text_append_zero() {
        let expect = "1234xyz0";
        assert_eq!(expect, format_text(&"1234xyz".to_string()))
    }

    #[test]
    // スペースが含まれていた場合のテスト
    fn test_format_text_remove_space() {
        let expect = "001122334455";
        assert_eq!(expect, format_text(&" 00 11 22 33  44 55 ".to_string()))
    }

    #[test]
    // \rが含まれていた場合のテスト
    fn test_format_text_remove_n() {
        let expect = "abcd0123";
        assert_eq!(expect, format_text(&" abcd\n0123".to_string()))
    }

    #[test]
    // \nが含まれていた場合のテスト
    fn test_format_text_remove_r() {
        let expect = "xyz56789";
        assert_eq!(expect, format_text(&" xyz\r56789".to_string()))
    }

    #[test]
    // スペース、\r\nが含まれていた場合のテスト
    fn test_format_text_remove_composite() {
        let expect = "xyz56789";
        assert_eq!(expect, format_text(&" xyz 56\r78\n9".to_string()))
    }

    #[test]
    // 文字列"08"のテスト
    fn test_str_2_binary_08() {
        match str_2_binary(vec!['0', '8']) {
            Ok(ok) => assert_eq!(0x08, ok),
            Err(err) => panic!("{}", err),
        }
    }

    #[test]
    // 文字列"0a"、"0A"のテスト
    fn test_str_2_binary_0a() {
        match str_2_binary(vec!['0', 'a']) {
            Ok(ok) => assert_eq!(0x0a, ok),
            Err(err) => panic!("{}", err),
        }
        match str_2_binary(vec!['0', 'A']) {
            Ok(ok) => assert_eq!(0x0a, ok),
            Err(err) => panic!("{}", err),
        }
    }

    #[test]
    // 文字列"f1"、"F0"のテスト
    fn test_str_2_binary_f1() {
        match str_2_binary(vec!['f', '1']) {
            Ok(ok) => assert_eq!(0xf1, ok),
            Err(err) => panic!("{}", err),
        }
        match str_2_binary(vec!['F', '1']) {
            Ok(ok) => assert_eq!(0xf1, ok),
            Err(err) => panic!("{}", err),
        }
    }

    #[test]
    // 文字列の長さが3桁の場合のエラーテスト
    fn test_str_2_binary_err_000() {
        let err_msg = "Target is not two characters. target=['0', '0', '0']";
        match str_2_binary(vec!['0', '0', '0']) {
            Ok(ok) => panic!("エラーが発生しませんでした。Ok={}", ok),
            Err(err) => assert_eq!(err_msg, err),
        }
    }

    #[test]
    // 文字列の長さが1桁の場合のエラーテスト
    fn test_str_2_binary_err_1() {
        let err_msg = "Target is not two characters. target=['1']";
        match str_2_binary(vec!['1']) {
            Ok(ok) => panic!("エラーが発生しませんでした。Ok={}", ok),
            Err(err) => assert_eq!(err_msg, err),
        }
    }

    #[test]
    // 文字列に16進数表現以外の文字(0)が出現した場合のエラーテスト
    fn test_str_2_binary_err_0z() {
        let err_msg = "It is not a hexadecimal representation. target=z";
        match str_2_binary(vec!['0', 'z']) {
            Ok(ok) => panic!("エラーが発生しませんでした。Ok={}", ok),
            Err(err) => assert_eq!(err_msg, err),
        }
    }

    #[test]
    // 文字列に全角文字(１)が出現した場合のエラーテスト
    fn test_str_2_binary_err_wide() {
        let err_msg = "It is not a hexadecimal representation. target=１";
        match str_2_binary(vec!['１', '0']) {
            Ok(ok) => panic!("エラーが発生しませんでした。Ok={}", ok),
            Err(err) => assert_eq!(err_msg, err),
        }
    }

    #[test]
    // 文字列"001123456789abcdEF00"のテスト
    fn test_convert() {
        let expect = vec![0x00u8, 0x11u8, 0x23u8, 0x45u8, 0x67u8, 0x89u8, 0xABu8, 0xCDu8, 0xEFu8, 0x00u8];
        let target = "001123456789abcdEF00".to_string();

        match convert(&target) {
            Ok(ok) => assert_eq!(expect, ok),
            Err(err) => panic!("{}", err),
        }
    }

    #[test]
    // 文字列"aBcDE"のテスト
    fn test_convert_complement_zero() {
        let expect = vec![0xabu8, 0xcdu8, 0xe0u8];
        let target = "aBcDE".to_string();

        match convert(&target) {
            Ok(ok) => assert_eq!(expect, ok),
            Err(err) => panic!("{}", err),
        }
    }

    #[test]
    // 文字列に空白や改行コードが含まれていた場合のテスト
    fn test_convert_space_and_break() {
        let expect = vec![0x12u8, 0x34u8, 0x56u8, 0x78u8, 0x9au8];
        let target = "  12  34\r\n 56 78  9 a ".to_string();

        match convert(&target) {
            Ok(ok) => assert_eq!(expect, ok),
            Err(err) => panic!("{}", err),
        }
    }

    #[test]
    // 文字列に16進数表現以外の文字(む)が出現した場合のエラーテスト
    fn test_convert_err() {
        match convert(&"01む345".to_string()) {
            Ok(ok) => panic!("エラーが発生しませんでした。Ok={:?}", ok),
            Err(err) => assert_eq!("It is not a hexadecimal representation. target=む", err),
        }
    }
}
