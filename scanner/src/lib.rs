use token::{Literal, Position, Token};

type ErrorHandle = Box<dyn Fn(Position, String)>;

pub struct Scanner {
    filename: String,
    src: String,

    ch: u8,
    offset: usize,
    rd_offset: usize,

    line_offset: usize,
    line_no: usize,

    err: ErrorHandle,
}

impl Scanner {
    pub fn new(filename: String, src: String, err: ErrorHandle) -> Self {
        let mut s = Self {
            filename,
            src,
            ch: b' ',
            offset: 0,
            rd_offset: 0,
            line_offset: 0,
            line_no: 1,
            err,
        };
        s.next();
        s
    }

    fn next(&mut self) {
        if let Some(&ch) = self.src.as_bytes().get(self.rd_offset) {
            self.offset = self.rd_offset;
            if self.ch == b'\n' {
                self.line_offset = self.offset;
                self.line_no += 1;
            }

            if self.ch == 0 {
                self.error(format!("invalid character null"));
            }

            self.rd_offset += 1;
            self.ch = ch;
        } else {
            self.offset = self.src.len();
            if self.ch == b'\n' {
                self.line_offset = self.offset;
                self.line_no += 1;
            }
            self.ch = 0;
        }
    }
    fn error(&self, msg: String) {
        (self.err)(self.position(), msg);
    }

    fn peek(&self) -> u8 {
        *self.src.as_bytes().get(self.rd_offset).unwrap_or(&0)
    }

    #[inline]
    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.next();
        }
    }

    #[inline]
    fn position(&self) -> Position {
        Position {
            filename: self.filename.clone(),
            offset: self.offset,
            line: self.line_no,
            column: self.offset - self.line_offset + 1,
        }
    }

    fn advance(&mut self, len: usize) {
        self.offset += len;
        self.rd_offset = self.offset + 1;
        self.ch = *self.src.as_bytes().get(self.offset).unwrap_or(&0);
    }

    #[inline]
    fn scan_ident(&self) -> Literal {
        String::from_iter(
            self.src.as_bytes()[self.offset..]
                .iter()
                .take_while(|&&c| c.is_ascii_alphanumeric() || c == b'_' || c == b'$')
                .map(|c| *c as char),
        )
    }

    #[inline]
    fn scan_integer(&self) -> Literal {
        String::from_iter(
            self.src.as_bytes()[self.offset..]
                .iter()
                .take_while(|&&c| c.is_ascii_digit())
                .map(|c| *c as char),
        )
    }
    /*  #[inline]
    fn scan_binary_integer(&self) -> Literal {
        String::from_iter(
            let len =2;
            self.src.as_bytes()[self.offset..]
                .iter()
                .take_while(|&&c| c == b'0' || c == b'1')
                .for_each(|_| len+=1);


                .map(|c: &u8|  *c as char),
        )
    }*/
    /*  #[inline]
    fn scan_hexadecimal_integer(&self) -> Literal {
        String::from_iter(
            self.src.as_bytes()[self.offset..]
                .iter()
                .take_while(|&&c| c.is_ascii_hexdigit() )
                .map(|c: &u8|  *c as char),
        )
    }*/
    #[inline]
    fn scan_octal_integer(&self) -> Literal {
        String::from_iter(
            self.src.as_bytes()[self.offset..]
                .iter()
                .take_while(|&&c| c >= b'0' && c <= b'7')
                .map(|c: &u8| *c as char),
        )
    }

    pub fn scan(&mut self) -> (Token, Position, &str) {
        todo!()
    }
}

fn is_letter(c: u8) -> bool {
    c >= b'a' && c <= b'z'
        || c >= b'A' && c <= b'Z'
        || c >= b'0' && c <= b'9'
        || c == b'_'
        || c == b'$'
}

fn is_digit(c: u8) -> bool {
    c >= b'0' && c <= b'9'
}
fn is_hex_digit(c: u8) -> bool {
    c >= b'0' && c <= b'9' || c >= b'A' && c <= b'F' || c >= b'a' && c <= b'f'
}
fn is_octal_digit(c: u8) -> bool {
    c >= b'0' && c <= b'7'
}
fn is_binary_digit(c: u8) -> bool {
    c == b'0' || c == b'1'
}

impl IntoIterator for Scanner {
    type Item = (Token, Position, Literal);

    type IntoIter = ScannerIter;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter { scanner: self }
    }
}

pub struct ScannerIter {
    scanner: Scanner,
}

impl Iterator for ScannerIter {
    type Item = (Token, Position, String);

    fn next(&mut self) -> Option<Self::Item> {
        let (tok,pos,lit) = self.scanner.scan();

        if tok == Token::EOF {
            return None;
        }

        Some((tok, pos, lit.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Token::*;

    #[test]
    fn test_scan() {
        let tests = [
            (ILLEGAL, "@"),
            (IDENT, "intIs_32bit"),
            (IDENT, "_Give_me_100$"),
            (IDENT, "$"),
            (INTEGER, "1234567890"),
            // (INTEGER, "01234567"),
            // (INTEGER, "0x123456790abcdefABCDEF"),
            // (INTEGER, "0b1010"),
            // (FLOATING, "0."),
            // (FLOATING, ".1"),
            // (FLOATING, "3.1"),
            // (FLOATING, "9.e10"),
            // (FLOATING, "9.e-10"),
            // (FLOATING, "9.e+10"),
            // (FLOATING, "9.1e10"),
            // (FLOATING, "9.1e-10"),
            // (FLOATING, "9.1e+10"),
            // (FLOATING, ".1e10"),
            // (FLOATING, ".1e-10"),
            // (FLOATING, ".1e+10"),
            // (STRING, "\"crepl\""),
            // (STRING, "\"He said, \\\"I can eat 4 mango\\\".\""),
            (ASSIGN, "="),
            (ADD_ASSIGN, "+="),
            (SUB_ASSIGN, "-="),
            (MUL_ASSIGN, "*="),
            (DIV_ASSIGN, "/="),
            (REM_ASSIGN, "%="),
            (AND_ASSIGN, "&="),
            (OR_ASSIGN, "|="),
            (XOR_ASSIGN, "^="),
            (SHL_ASSIGN, "<<="),
            (SHR_ASSIGN, ">>="),
            (INC, "++"),
            (DEC, "--"),
            (PLUS, "+"),
            (MINUS, "-"),
            (ASTERISK, "*"),
            (SLASH, "/"),
            (REM, "%"),
            (TILDE, "~"),
            (AND, "&"),
            (OR, "|"),
            (XOR, "^"),
            (SHL, "<<"),
            (SHR, ">>"),
            (NOT, "!"),
            (LAND, "&&"),
            (LOR, "||"),
            (TERNARY, "?"),
            (DOT, "."),
            (ARROW, "->"),
            (ELIPSE, "..."),
            (COMMA, ","),
            (SEMICOLON, ";"),
            (COLON, ":"),
            (LPAREN, "("),
            (LBRACE, "{"),
            (LBRACK, "["),
            (RPAREN, ")"),
            (RBRACE, "}"),
            (RBRACK, "]"),
            (AUTO, "auto"),
            (BREAK, "break"),
            (CASE, "case"),
            (CHAR, "char"),
            (CONST, "const"),
            (CONTINUE, "continue"),
            (DEFAULT, "default"),
            (DO, "do"),
            (DOUBLE, "double"),
            (ELSE, "else"),
            (ENUM, "enum"),
            (EXTERN, "extern"),
            (FLOAT, "float"),
            (FOR, "for"),
            (GOTO, "goto"),
            (IF, "if"),
            (INLINE, "inline"),
            (INT, "int"),
            (LONG, "long"),
            (REGISTER, "register"),
            (RESTRICT, "restrict"),
            (RETURN, "return"),
            (SHORT, "short"),
            (SIGNED, "signed"),
            (SIZEOF, "sizeof"),
            (STATIC, "static"),
            (STRUCT, "struct"),
            (SWITCH, "switch"),
            (TYPEDEF, "typedef"),
            (UNION, "union"),
            (UNSIGNED, "unsigned"),
            (VOID, "void"),
            (VOLATILE, "volatile"),
            (WHILE, "while"),
        ];

        let source = tests
            .iter()
            .map(|(_, lit)| lit.to_owned())
            .collect::<Vec<_>>()
            .join(" ");

        let mut s = Scanner::new("repl".to_string(), source.clone(), Box::new(|_, _| {}));

        for (i, t) in tests.iter().enumerate() {
            let (tok, _, lit) = s.scan();

            assert_eq!(
                *t,
                (tok, lit),
                "source: {}\n [{}/{}] test failed.",
                source,
                i + 1,
                tests.len()
            );
        }
    }
}
