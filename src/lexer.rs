use super::parser::{Diagnostic, Span};
use codespan_reporting::diagnostic::Label;
use logos::{Lexer, Logos};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum LexerError {
    #[default]
    Invalid,
    UnterminatedString,
    InvalidSymbolFirstCharacter(char),
    UnterminatedDiscard,
}

impl LexerError {
    pub fn into_diagnostic(self, span: Span) -> Diagnostic {
        match self {
            Self::Invalid => Diagnostic::error()
                        .with_message("invalid token")
                        .with_labels(vec![Label::primary((), span)]),
            Self::UnterminatedString => Diagnostic::error()
                        .with_message("unterminated string")
                        .with_labels(vec![Label::primary((), span)]),
            Self::InvalidSymbolFirstCharacter(c) => Diagnostic::error()
                        .with_message(format!("invalid symbol/keyword first character: {c}"))
                        .with_labels(vec![Label::primary((), span)]),
            Self::UnterminatedDiscard => Diagnostic::error()
                        .with_message("unterminated discard sequence")
                        .with_labels(vec![Label::primary((), span)]),
        }
    }
}

#[allow(clippy::upper_case_acronyms, missing_docs)]
#[derive(Logos, Debug, PartialEq, Eq, Copy, Clone)]
#[logos(error = LexerError)]
pub enum Token {
    EOF,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("nil")]
    Null,
    #[token("#inst")]
    Inst,
    #[token("#uuid")]
    Uuid,
    #[token("\\")]
    CharEscape,
    #[token("/")]
    Slash,
    #[token("#{")]
    LSetBrace,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("[")]
    LBrak,
    #[token("]")]
    RBrak,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token(",")]
    Comma,
    #[token(":")]
    Colon,
    #[token("#")]
    Hash,
    #[regex("[\\+\\-\\.]?[A-Za-z]{1}[A-Za-z0-9-_\\?\\!\\.\\*\\+\\/]+|[\\+\\-\\.]|[^0-9]", priority = 0,)]
    Name,
    #[regex("\"", parse_string)]
    String,
    #[regex("\\\\([A-Za-z]{1}|[uU][0-9a-fA-F]{4}|newline|return|space|tab)")]
    Chars,
    #[regex(r"-?(0|[1-9][0-9]*)(\.[0-9]+)?([eE][+-]?[0-9]+)?")]
    Number,
    #[regex(r"-?(0|[1-9][0-9]*)\/([1-9][0-9]*)")]
    Rational,
    #[regex(
        "\"([0-9]{4}-[0-9]{2}-[0-9]{2})T([0-9]{2}:[0-9]{2}:[0-9]{2})((.[0-9]+)?Z|[+-][0-9]{2}:[0-9]{2})\""
    )]
    Timestamp,
    #[regex("\"[A-Za-z0-9]{8}-[A-Za-z0-9]{4}-[A-Za-z0-9]{4}-[A-Za-z0-9]{4}-[A-Za-z0-9]{12}\"")]
    Id,
    #[regex("[\u{0020}\u{0009}]+")]
    Whitespace,
    #[regex("\r?\n")]
    Newline,
    #[regex("[;]+[\x09\x20-\x7E\u{0080}-\u{D7FF}\u{E000}-\u{10FFFF}]*")]
    Comment,
    #[regex("#_", parse_discard)]
    Discard,
    Error,
}

// const ALLOWED_CHARS: [char;13] = ['.', '*', '+', '!', '-', '_', '?', '$', '%', '&', '=', '<', '>'];
// const ALLOWED_FIRST_CHARS: [char;3] = ['.', '+', '-'];

fn parse_string(lexer: &mut Lexer<'_, Token>) -> Result<(), LexerError> {
    let mut it = lexer.remainder().chars();
    while let Some(c) = it.next() {
        match c {
            '"' => {
                lexer.bump(1);
                return Ok(());
            }
            '\\' => {
                lexer.bump(1);
                if let Some(c) = it.next() {
                    lexer.bump(c.len_utf8());
                }
            }
            c => {
                lexer.bump(c.len_utf8());
            }
        }
    }
    Err(LexerError::UnterminatedString)
}

fn parse_discard(lexer: &mut Lexer<'_, Token>) -> Result<(), LexerError> {
    let mut it = lexer.remainder().chars().peekable();
    while let Some(c) = it.peek() {
        match c {
            ' ' | '\n' | '\r' => {
                return Ok(());
            }
            c => {
                lexer.bump(c.len_utf8());
            }
        }
        it.next();
    }
    Err(LexerError::UnterminatedDiscard)
}

fn check_string(value: &str, span: &Span, diags: &mut Vec<Diagnostic>) {
    let mut it = value.chars().enumerate();
    while let Some((i, c)) = it.next() {
        match c {
            '\\' => match it.next() {
                Some((_, '"' | '\\' | '/' | 'b' | 'f' | 'n' | 'r' | 't')) => {}
                Some((i, 'u')) => {
                    for j in 0..4 {
                        if !it.next().is_some_and(|(_, c)| c.is_ascii_hexdigit()) {
                            diags.push(
                                Diagnostic::error()
                                    .with_message("invalid unicode escape sequence")
                                    .with_labels(vec![Label::primary(
                                        (),
                                        span.start + i - 1..span.start + i + j + 1,
                                    )]),
                            );
                            break;
                        }
                    }
                }
                Some((j, _)) => {
                    diags.push(
                        Diagnostic::error()
                            .with_message("invalid escape sequence")
                            .with_labels(vec![Label::primary(
                                (),
                                span.start + j - 1..span.start + j + 1,
                            )]),
                    );
                }
                _ => unreachable!(),
            },
            '\u{0020}'..='\u{10FFFF}' => {}
            c => {
                diags.push(
                    Diagnostic::error()
                        .with_message(format!("string contains invalid character {c:?}"))
                        .with_labels(vec![
                            Label::primary((), span.start + i..span.start + i + 1)
                                .with_message("after this character"),
                        ]),
                );
            }
        }
    }
}

pub fn tokenize(source: &str, diags: &mut Vec<Diagnostic>) -> (Vec<Token>, Vec<Span>) {
    let lexer = Token::lexer(source);
    let mut tokens = vec![];
    let mut spans = vec![];
    let source = lexer.source();

    let mut count_brace = 0;
    let mut count_brak = 0;
    let mut count_paren = 0;
    for (token, span) in lexer.spanned() {
        match token {
            Ok(token) => {
                match token {
                    Token::String => {
                        #[allow(clippy::unnecessary_struct_initialization)]
                        check_string(&source[span.start..span.end], &span, diags);
                    }
                    Token::LSetBrace | Token::LBrace => count_brace += 1,
                    Token::RBrace => count_brace -= 1,
                    Token::LBrak => count_brak += 1,
                    Token::RBrak => count_brak -= 1,
                    Token::LParen => count_paren += 1,
                    Token::RParen => count_paren -= 1,
                    _ => {}
                }
                if count_brace + count_brak + count_paren > 256 {
                    diags.push(
                        Diagnostic::error()
                            .with_message("bracket nesting level exceeded maximum of 256")
                            .with_labels(vec![Label::primary((), span)]),
                    );
                    break;
                }
                tokens.push(token);
            }
            Err(err) => {
                diags.push(err.into_diagnostic(span.clone()));
                tokens.push(Token::Error);
            }
        }
        spans.push(span);
    }
    (tokens, spans)
}
