use lexer::{util, ByteHandler};
use lexer::token::Token::*;
use lexer::ReservedKind::*;

use ast::Value;
use ast::OperatorKind::*;
use ast::DeclarationKind::*;

macro_rules! match_label {
    ($lex:ident [$( $byte:expr )* => $token:expr]) => {
        if $(
            $lex.next_byte() == $byte &&
        )* {$lex.bump(); !util::legal_in_label($lex.read_byte())} {
            return $lex.token = $token;
        }
    };

    ($lex:ident { [=> $token:expr] $( $match:tt $cont:tt )+ }) => {
        match $lex.next_byte() {
            $(
                $match => match_label!($lex $cont),
            )*
            ch if !util::legal_in_label(ch) => return $lex.token = $token,
            _ => {}
        }
    };

    ($lex:ident { $( $match:tt $cont:tt )+ }) => {
        match $lex.next_byte() {
            $(
                $match => match_label!($lex $cont),
            )*
            _ => {}
        }
    }
}

// Non-keyword Identifier: starting with a letter, _ or $
pub const IDT: ByteHandler = Some(|lex| {
    lex.bump();
    lex.token = Identifier(lex.read_label());
});

// Identifier or keyword starting with a letter `b`
pub const L_B: ByteHandler = Some(|lex| {
    match_label!(lex [b'r' b'e' b'a' b'k' => Break]);

    lex.token = Identifier(lex.read_label());
});

// Identifier or keyword starting with a letter `c`
pub const L_C: ByteHandler = Some(|lex| {
    match_label!(lex {
        b'o'{
            b'n'{
                b's'[b't'                => Declaration(Const)]
                b't'[b'i' b'n' b'u' b'e' => Continue]
            }
        }
        b'a'{
            b's'[b'e'       => Case]
            b't'[b'c' b'h'  => Catch]
        }
        b'l'[b'a' b's' b's' => Class]
    });

    lex.token = Identifier(lex.read_label());
});

// Identifier or keyword starting with a letter `d`
pub const L_D: ByteHandler = Some(|lex| {
    match_label!(lex {
        b'o'[                             => Do]
        b'e'{
            b'l'[b'e' b't' b'e'           => Operator(Delete)]
            b'f'[b'a' b'u' b'l' b't'      => Default]
            b'b'[b'u' b'g' b'g' b'e' b'r' => Debugger]
        }
    });

    lex.token = Identifier(lex.read_label());
});

// Identifier or keyword starting with a letter `e`
pub const L_E: ByteHandler = Some(|lex| {
    match_label!(lex {
        b'l'[b's' b'e'               => Else]
        b'x'{
            b'p'[b'o' b'r' b't'      => Export]
            b't'[b'e' b'n' b'd' b's' => Extends]
        }
        b'n'[b'u' b'm'               => Reserved(Enum)]
    });

    lex.token = Identifier(lex.read_label());

});

// Identifier or keyword starting with a letter `f`
pub const L_F: ByteHandler = Some(|lex| {
    match_label!(lex {
        b'i'[b'n' b'a' b'l' b'l' b'y'      => Finally]
        b'o'[b'r'                          => For]
        b'u'[b'n' b'c' b't' b'i' b'o' b'n' => Function]
        b'a'[b'l' b's' b'e'                => Literal(Value::False)]
    });

    lex.token = Identifier(lex.read_label());
});

// Identifier or keyword starting with a letter `i`
pub const L_I: ByteHandler = Some(|lex| {
    match_label!(lex {
        b'n'{
            [                                       => Operator(In)]
            b's'[b't' b'a' b'n' b'c' b'e' b'o' b'f' => Operator(Instanceof)]
            b't'[b'e' b'r' b'f' b'a' b'c' b'e'      => Reserved(Interface)]
        }
        b'f'[                                       => If]
        b'm'{
            b'p'{
                b'o'[b'r' b't'                      => Import]
                b'l'[b'e' b'm' b'e' b'n' b't' b's'  => Reserved(Implements)]
            }
        }
    });

    lex.token = Identifier(lex.read_label());
});

// Identifier or keyword starting with a letter `l`
pub const L_L: ByteHandler = Some(|lex| {
    match_label!(lex [b'e' b't' => Declaration(Let)]);

    lex.token = Identifier(lex.read_label());
});

// Identifier or keyword starting with a letter `n`
pub const L_N: ByteHandler = Some(|lex| {
    match_label!(lex {
        b'e'[b'w'      => Operator(New)]
        b'u'[b'l' b'l' => Literal(Value::Null)]
    });

    lex.token = Identifier(lex.read_label());
});

// Identifier or keyword starting with a letter `p`
pub const L_P: ByteHandler = Some(|lex| {
    match_label!(lex {
        b'a'[b'c' b'k' b'a' b'g' b'e'          => Reserved(Package)]
        b'u'[b'b' b'l' b'i' b'c'               => Reserved(Public)]
        b'r'{
            b'o'[b't' b'e' b'c' b't' b'e' b'd' => Reserved(Protected)]
            b'i'[b'v' b'a' b't' b'e'           => Reserved(Private)]
        }
    });

    lex.token = Identifier(lex.read_label());
});

// Identifier or keyword starting with a letter `r`
pub const L_R: ByteHandler = Some(|lex| {
    match_label!(lex [b'e' b't' b'u' b'r' b'n' => Return]);

    lex.token = Identifier(lex.read_label());
});

// Identifier or keyword starting with a letter `s`
pub const L_S: ByteHandler = Some(|lex| {
    match_label!(lex {
        b'u'[b'p' b'e' b'r'      => Super]
        b'w'[b'i' b't' b'c' b'h' => Switch]
        b't'[b'a' b't' b'i' b'c' => Static]
    });

    lex.token = Identifier(lex.read_label());
});

// Identifier or keyword starting with a letter `t`
pub const L_T: ByteHandler = Some(|lex| {
    match_label!(lex {
        b'y'[b'p' b'e' b'o' b'f' => Operator(Typeof)]
        b'h'{
            b'i'[b's'            => This]
            b'r'[b'o' b'w'       => Throw]
        }
        b'r'{
            b'y'[                => Try]
            b'u'[b'e'            => Literal(Value::True)]
        }
    });

    lex.token = Identifier(lex.read_label());
});

// Identifier or keyword starting with a letter `u`
pub const L_U: ByteHandler = Some(|lex| {
    match_label!(lex [b'n' b'd' b'e' b'f' b'i' b'n' b'e' b'd' => Literal(Value::Undefined)]);

    lex.token = Identifier(lex.read_label());
});

// Identifier or keyword starting with a letter `v`
pub const L_V: ByteHandler = Some(|lex| {
    match_label!(lex {
        b'a'[b'r'      => Declaration(Var)]
        b'o'[b'i' b'd' => Operator(Void)]
    });

    lex.token = Identifier(lex.read_label());
});

// Identifier or keyword starting with a letter `w`
pub const L_W: ByteHandler = Some(|lex| {
    match_label!(lex {
        b'h'[b'i' b'l' b'e' => While]
        b'i'[b't' b'h'      => With]
    });

    lex.token = Identifier(lex.read_label());
});

// Identifier or keyword starting with a letter `y`
pub const L_Y: ByteHandler = Some(|lex| {
    match_label!(lex [b'i' b'e' b'l' b'd' => Yield]);

    lex.token = Identifier(lex.read_label());
});
