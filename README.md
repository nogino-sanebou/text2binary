# Overview
Converts a string representation of a hexadecimal number to a Vec&lt;u8&gt;.

Supported are String, Vec&lt;String&gt; and Files.

The string can include spaces and line breaks.

Case insensitive.

# Errors
It was not a hexadecimal representation.

# Example

## String pattern

```rust
let text: String = "001234Ab".to_string();
let binary: Vec<u8> = text2binary::convert(&text).unwrap();

// [0x00, 0x12, 0x34, 0xAB]

```
```rust
let text: String = r#"00 12
34 56"#.to_string();
let binary: Vec<u8> = text2binary::convert(&text).unwrap();

// [0x00, 0x12, 0x34, 0x56]
```

## Vec&lt;String&gt; pattern

```rust
let lines: Vec<String> = vec!["1234".to_string(), r#"AB
cd"#.to_string(), "".to_string(), "00 00".to_string()];
let binary: Vec<u8> = text2binary::convert_line(&lines).unwrap();

// [0x12, 0x34, 0xAB, 0xCD, 0x00, 0x00]
```

## File pattern

```rust
use std::fs::File;

// target.txt
// 00 12 34
// 56 78 AB
// cd Ef 00
let file: File = File::open("target.txt").unwrap();
let binary: Vec<u8> = text2binary::convert_file(&file).unwrap();

// [0x00, 0x12, 0x34, 0x56, 0x78, 0xAB, 0xCD, 0xEF, 0x00]
```

```rust
// target.txt
// cd Ef 00
//
// 56 78 AB
//
// 00 12 34
//
//
let file: File = File::open("target.txt").unwrap();
let binary: Vec<u8> = text2binary::convert_file(&file).unwrap();

// [0xCD, 0xEF, 0x00, 0x56, 0x78, 0xAB, 0x00, 0x12, 0x34]
```

----

# 日本語概要
16進数の文字列表現をバイナリデータ(Vec<u8>)に変換します。

String、Vec&lt;String&gt;、Fileに対応しています。

文字列には空白、改行コードも含められます。

大文字と小文字を区別しません。

# エラー

16進数表現ではなかった。