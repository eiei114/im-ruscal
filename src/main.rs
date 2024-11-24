fn main() {
    let s = "(123  456  world)";
    println!("source: {:?}, parsed:\n {:?}", s, source(s));

    let s = "((car cdr) cdr)";
    println!("source: {:?}, parsed:\n {:?}", s, source(s));

    let s = "()())))((()))";
    println!("source: {:?}, parsed:\n {:?}", s, source(s));
}

/// 次の文字を進める関数
///
/// # 引数
/// * `input` - 進める対象の文字列
///
/// # 戻り値
/// * `&str` - 進めた後の文字列
fn advance_char(input: &str) -> &str {
    let mut chars = input.chars();
    chars.next();
    chars.as_str()
}

/// 次の文字を見る関数
///
/// # 引数
/// * `input` - 見る対象の文字列
///
/// # 戻り値
/// * `Option<char>` - 次の文字
fn peek_char(input: &str) -> Option<char> {
    input.chars().next()
}

/// ソースコードを解析してトークンのリストを返す関数
///
/// # 引数
/// * `input` - 解析対象の文字列
///
/// # 戻り値
/// * `Vec<Token>` - 解析結果のトークンのリスト
fn source(mut input: &str) -> Vec<Token> {
    let mut tokens = vec![];

    while !input.is_empty() {
        input = if let (next_input, Some(token)) = token(input) {
            tokens.push(token);
            next_input
        } else {
            break;
        }
    }
    tokens
}

#[derive(Debug, PartialEq, Eq)]
enum Token {
    Indent,
    Number,
    LParen,
    RParen,
}

fn token(i: &str) -> (&str, Option<Token>) {
    if let (i, Some(ident_res)) = ident(whitespace(i)) {
        return (i, Some(ident_res));
    }

    if let (i, Some(number_res)) = number(whitespace(i)) {
        return (i, Some(number_res));
    }

    if let (i, Some(lparen_res)) = lparen(whitespace(i)) {
        return (i, Some(lparen_res));
    }

    if let (i, Some(rparen_res)) = rparen(whitespace(i)) {
        return (i, Some(rparen_res));
    }

    (whitespace(i), None)
}

fn whitespace(mut input: &str) -> &str {
    while matches!(peek_char(input), Some(' ')) {
        input = advance_char(input);
    }
    input
}

/// 識別子（アルファベットで始まり、その後にアルファベットまたは数字が続く文字列）を解析する関数
///
/// # 引数
/// * `input` - 解析対象の文字列
///
/// # 戻り値
/// * `(&str, Option<Token>)` - (残りの入力文字列, 解析結果のトークン)のタプル
///   - 識別子として解析できた場合は `Some(Token::Indent)` を返す
///   - 解析できなかった場合は `None` を返す
fn ident(mut input: &str) -> (&str, Option<Token>) {
    if matches!(peek_char(input), Some(_x @ ('a'..='z' | 'A'..='Z'))) {
        while matches!(
            peek_char(input),
            Some(_x @ ('a'..='z' | 'A'..='Z' | '0'..='9'))
        ) {
            input = advance_char(input);
        }
        (input, Some(Token::Indent))
    } else {
        (input, None)
    }
}

/// 数値を解析する関数
///
/// # 引数
/// * `input` - 解析対象の文字列
///
/// # 戻り値
/// * `(&str, Option<Token>)` - (残りの入力文字列, 解析結果のトークン)のタプル
fn number(mut input: &str) -> (&str, Option<Token>) {
    if matches!(peek_char(input), Some(_x @ ('-' | '+' | '.' | '0'..='9'))) {
        input = advance_char(input);
        while matches!(peek_char(input), Some(_x @ ('.' | '0'..='9'))) {
            input = advance_char(input);
        }
        (input, Some(Token::Number))
    } else {
        (input, None)
    }
}

/// 左括弧を解析する関数
///
/// # 引数
/// * `input` - 解析対象の文字列
///
/// # 戻り値
/// * `(&str, Option<Token>)` - (残りの入力文字列, 解析結果のトークン)のタプル
fn lparen(mut input: &str) -> (&str, Option<Token>) {
    if matches!(peek_char(input), Some('(')) {
        input = advance_char(input);
        (input, Some(Token::LParen))
    } else {
        (input, None)
    }
}

/// 右括弧を解析する関数
///
/// # 引数
/// * `input` - 解析対象の文字列
///
/// # 戻り値
/// * `(&str, Option<Token>)` - (残りの入力文字列, 解析結果のトークン)のタプル
fn rparen(mut input: &str) -> (&str, Option<Token>) {
    if matches!(peek_char(input), Some(')')) {
        input = advance_char(input);
        (input, Some(Token::RParen))
    } else {
        (input, None)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_whitespace() {
        assert_eq!(whitespace(" "), "");
    }

    #[test]
    fn test_ident() {
        assert_eq!(ident("Adam"), "");
    }

    #[test]
    fn test_number() {
        assert_eq!(number("123.45"), "");
    }
}
