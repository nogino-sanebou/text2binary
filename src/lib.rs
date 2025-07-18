use std::string::ToString;

pub fn convert(text: &String) -> Vec<u8> {
    // スペースと改行を除いたテキストに整形
    let mut target = format_text(&text).chars().collect::<Vec<char>>();

    let mut byte_vec = vec![];
    while target.len() > 0 {
        // 先頭2文字をバイトに変換してVecに追加する
        let byte = str_2_binary(target[..2].to_vec());
        byte_vec.push(byte);

        // テキストの次の2文字を先頭にする
        target = target[2..].to_vec();
    }

    byte_vec.clone()
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

fn str_2_binary(target: Vec<char>) -> u8 {
    // 文字が2文字以外の場合はパニック
    if target.len() != 2 {
        panic!("Target is not two characters. target={:?}", target);
    }

    // 上位バイトと下位バイトにわける
    let upper = char_2_hex(target[0]);
    let lower = char_2_hex(target[1]);

    // 上位バイトは4桁左にシフトし、下位バイトと加算する
    (upper << 4) + lower
}

fn char_2_hex(target: char) -> u8 {
    match target {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'A' | 'a' => 10,
        'B' | 'b' => 11,
        'C' | 'c' => 12,
        'D' | 'd' => 13,
        'E' | 'e' => 14,
        'F' | 'f' => 15,

        // 16進数表現以外が含まれていた場合はパニック
        _ => panic!("It is not a hexadecimal representation. target={}", target),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_text_append_zero() {
        let expect = "1234567890";
        assert_eq!(expect, format_text(&"123456789".to_string()))
    }

    #[test]
    fn test_format_text_remove_space() {
        let expect = "001122334455";
        assert_eq!(expect, format_text(&" 00 11 22 33  44 55 ".to_string()))
    }

    #[test]
    fn test_format_text_remove_n() {
        let expect = "abcd0123";
        assert_eq!(expect, format_text(&" abcd\n0123".to_string()))
    }

    #[test]
    fn test_format_text_remove_r() {
        let expect = "xyz56789";
        assert_eq!(expect, format_text(&" xyz\r56789".to_string()))
    }

    #[test]
    fn test_str_2_binary_08() {
        assert_eq!(0x08, str_2_binary(vec!['0', '8']));
    }

    #[test]
    fn test_str_2_binary_0a() {
        assert_eq!(0x0a, str_2_binary(vec!['0', 'a']));
        assert_eq!(0x0a, str_2_binary(vec!['0', 'A']));
    }

    #[test]
    fn test_str_2_binary_f1() {
        assert_eq!(0xf1, str_2_binary(vec!['f', '1']));
        assert_eq!(0xf1, str_2_binary(vec!['F', '1']));
    }

    #[test]
    #[should_panic(expected = "Target is not two characters. target=['0', '0', '0']")]
    fn test_str_2_binary_panic_000() {
        str_2_binary(vec!['0', '0', '0']);
    }

    #[test]
    #[should_panic(expected = "Target is not two characters. target=['1']")]
    fn test_str_2_binary_panic_1() {
        str_2_binary(vec!['1']);
    }

    #[test]
    #[should_panic(expected = "It is not a hexadecimal representation. target=z")]
    fn test_str_2_binary_panic_0z() {
        str_2_binary(vec!['0', 'z']);
    }

    #[test]
    #[should_panic(expected = "It is not a hexadecimal representation. target=あ")]
    fn test_str_2_binary_panic_wide() {
        str_2_binary(vec!['あ', '0']);
    }

    #[test]
    fn test_convert() {
        let expect = vec![0x00u8, 0x11u8, 0x23u8, 0x45u8, 0x67u8, 0x89u8, 0xABu8, 0xCDu8, 0xEFu8, 0x00u8];
        let target = "001123456789abcdEF00".to_string();

        assert_eq!(expect, convert(&target));
    }

    #[test]
    fn test_convert_complement_zero() {
        let expect = vec![0xabu8, 0xcdu8, 0xe0u8];
        let target = "aBcDE".to_string();

        assert_eq!(expect, convert(&target));
    }

    #[test]
    fn test_convert_space_and_break() {
        let expect = vec![0x12u8, 0x34u8, 0x56u8, 0x78u8, 0x9au8];
        let target = "  12  34\r\n 56 78  9 a ".to_string();

        assert_eq!(expect, convert(&target));
    }

    #[test]
    #[should_panic(expected = "It is not a hexadecimal representation. target=む")]
    fn test_convert_panic() {
        convert(&"01む345".to_string());
    }
}
