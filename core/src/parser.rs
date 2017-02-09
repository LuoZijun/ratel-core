use ast::{Program, Node, Statement, Expression};
use error::{Error, ParseResult};
use tokenizer::Tokenizer;
use lexicon::Token;

/// Peek on the next token. Return with an error if tokenizer fails.
macro_rules! peek {
    ($parser:ident) => {
        match $parser.token {
            Some(token) => token,

            None => {
                let token = try!($parser.tokenizer.get_token());

                $parser.token = Some(token);

                token
            }
        }
    }
}

/// Get the next token. Return with an error if tokenizer fails.
macro_rules! next {
    ($parser:ident) => {
        match $parser.token {
            Some(token) => {
                $parser.consume();

                token
            },
            None => try!($parser.tokenizer.get_token())
        }
    }
}

/// If the next token matches `$p`, consume that token and execute `$eval`.
macro_rules! allow {
    ($parser:ident, $p:pat => $eval:expr) => {
        match peek!($parser) {
            $p => {
                $parser.consume();
                $eval;
            },
            _ => {}
        }
    }
}

/// Return an error if the next token doesn't match $p.
macro_rules! expect {
    ($parser:ident, $p:pat) => {
        match next!($parser) {
            $p => {},
            _  => unexpected_token!($parser)
        }
    }
}

/// Expect the next token to be an Identifier, extracting the OwnedSlice
/// out of it. Returns an error otherwise.
macro_rules! expect_identifier {
    ($parser:ident) => {
        match next!($parser) {
            Identifier(ident) => ident,
            _                 => unexpected_token!($parser)
        }
    }
}

/// Expecta semicolon to terminate a statement. Will assume a semicolon
/// following the ASI rules.
macro_rules! expect_semicolon {
    ($parser:ident) => {
        // TODO: Tokenizer needs to flag when a new line character has been
        //       consumed to satisfy all ASI rules
        match peek!($parser) {
            Semicolon     => $parser.consume(),

            ParenClose    |
            BraceClose    |
            EndOfProgram  => {},

            _             => {
                if !$parser.tokenizer.asi() {
                    unexpected_token!($parser)
                }
            }
        }
    }
}

/// Return an error for current token.
macro_rules! unexpected_token {
    ($parser:ident) => {
        return Err($parser.tokenizer.invalid_token())
    };
}

pub struct Parser<'src> {
    source: &'src str,

    // Tokenizer will produce tokens from the source
    tokenizer: Tokenizer<'src>,

    // Current token, to be used by peek! and next! macros
    token: Option<Token>,

    expressions: Vec<Node<Expression>>,
    statements: Vec<Node<Statement>>,
}

impl<'src> Parser<'src> {
    pub fn new(source: &'src str) -> Self {
        Parser {
            source: source,
            tokenizer: Tokenizer::new(source),
            token: None,
            expressions: Vec::new(),
            statements: Vec::new(),
        }
    }

    #[inline]
    fn consume(&mut self) {
        self.token = None;
    }
}

pub fn parse<'src>(source: &'src str) -> ParseResult<Program<'src>> {
    let mut parser = Parser::new(source);

    Ok(Program {
        source: source,
        expressions: Vec::new(),
        statements: Vec::new(),
    })
}
