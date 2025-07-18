use std::fs::File;
use std::io::{BufRead, BufReader};
use std::string::ToString;

/// Converts a string representation of a hexadecimal number to a Vec&lt;u8&gt;.
///
/// 16進数の文字列表現をVec&lt;u8&gt;に変換します。
///
/// The string can include spaces and line breaks.
///
/// 文字列には空白、改行コードも含められます。
///
/// Case insensitive.
///
/// 大文字と小文字を区別しません。
///
/// # Errors
/// It was not a hexadecimal representation.
///
/// 16進数表現ではなかった。
///
/// # Examples
///
/// ```
/// let text: String = "001234Ab".to_string();
/// let binary: Vec<u8> = text2binary::convert(&text).unwrap();
/// assert_eq!(vec![0x00, 0x12, 0x34, 0xAB], binary);
///
/// let text: String = r#"00 12
/// 34 56"#.to_string();
/// let binary: Vec<u8> = text2binary::convert(&text).unwrap();
/// assert_eq!(vec![0x00, 0x12, 0x34, 0x56], binary);
/// ```
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

/// Converts a vector of hexadecimal string representation to a Vec&lt;u8&gt;.
///
/// 16進数の文字列表現のベクターをVec&lt;u8&gt;に変換します。
///
/// The string can include spaces and line breaks.
///
/// 文字列には空白、改行コードも含められます。
///
/// Case insensitive.
///
/// 大文字と小文字を区別しません。
///
/// # Errors
/// It was not a hexadecimal representation.
///
/// 16進数表現ではなかった。
///
/// # Examples
///
/// ```
/// let lines: Vec<String> = vec!["1234".to_string(), r#"AB
/// cd"#.to_string(), "".to_string(), "00 00".to_string()];
/// let binary: Vec<u8> = text2binary::convert_line(&lines).unwrap();
/// assert_eq!(vec![0x12, 0x34, 0xAB, 0xCD, 0x00, 0x00], binary);
/// ```
pub fn convert_line(line: &Vec<String>) -> Result<Vec<u8>, String> {
    // 文字列Vecを結合してStringに変換
    let join_string = line.join("");
    convert(&join_string)
}

/// Convert a text file containing a hexadecimal string representation to a Vec&lt;u8&gt;.
///
/// 16進数の文字列表現が書かれたテキストファイルをVec&lt;u8&gt;に変換します。
///
/// The string can include spaces and line breaks.
///
/// 文字列には空白、改行コードも含められます。
///
/// Case insensitive.
///
/// 大文字と小文字を区別しません。
///
/// Blank lines are ignored.
///
/// 空行は無視します。
///
/// # Errors
/// It was not a hexadecimal representation.
///
/// 16進数表現ではなかった。
///
/// File did not exist.
///
/// ファイルが存在しなかった。
///
/// # Examples
///
/// ```
/// use std::fs::File;
///
/// // target.txt
/// // 00 12 34
/// // 56 78 AB
/// // cd Ef 00
/// let file: File = File::open("./tests/target.txt").unwrap();
/// let binary: Vec<u8> = text2binary::convert_file(&file).unwrap();
/// assert_eq!(vec![0x00, 0x12, 0x34, 0x56, 0x78, 0xAB, 0xCD, 0xEF, 0x00], binary);
///
/// // target2.txt
/// // cd Ef 00
/// //
/// // 56 78 AB
/// //
/// // 00 12 34
/// //
/// let file: File = File::open("./tests/target2.txt").unwrap();
/// let binary: Vec<u8> = text2binary::convert_file(&file).unwrap();
/// assert_eq!(vec![0xCD, 0xEF, 0x00, 0x56, 0x78, 0xAB, 0x00, 0x12, 0x34], binary);
/// ```
pub fn convert_file(file: &File) -> Result<Vec<u8>, String> {
    let mut string = "".to_string();

    // ファイルを1行ずつ読み込み1つのStringに結合する
    let mut reader = BufReader::new(file);
    loop {
        let mut line = "".to_string();
        let len = match reader.read_line(&mut line) {
            Ok(len) => len,
            Err(err) => return Err(format!("{}", err)),
        };
        if len == 0 {
            break;
        }
        string.push_str(line.as_str());
    }

    convert(&string)
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
}
