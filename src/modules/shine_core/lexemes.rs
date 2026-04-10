use std::collections::HashMap;
use std::ops::Deref;

pub enum Lexeme {
/*  Shine compiler  */
    Undefined,
    End,

/*  Bin Op's        */
    LexPlus,
    LexMinus,
    LexDiv,
    LexMulti,
    LexRem,
    LexBitLeftRot,          // <<
    LexBitRightRot,         // >>
    LexLogicAnd,
    LexLogicOr,
    LexLogicEq,
    LexLogicNEq,
    LexLogicG,              // >
    LexLogicL,              // <
    LexLogicGE,             // >=
    LexLogicLE,             // <=
    LexAssign,
    LexAssignPlus,
    LexAssignMinus,
    LexAssignDiv,
    LexAssignMulti,
    LexAssignRem,
    LexAssignBitLeftRot,    // <<=
    LexAssignBitRightRot,   // >>=
    LexAssignBitOr,         // |=
    LexAssignBitAnd,        // &=
    LexAssignBitXor,        // ^=

/*  Un Op's         */
    LexInc,
    LexDec,

/*  Spec Symbols    */
    LexAmpersand,           // &
    LexVertBar,             // |
    LexCaret,               // ^
    LexLParen,              // (
    LexRParen,              // )
    LexLCParen,             // {
    LexRCParen,             // }
    LexLSParen,             // [
    LexRSParen,             // ]
    LexTwoDots,             // :
    LexDblTwoDots,          // ::
    LexDblQuote,            // "
    LexSingleQuote,         // '
    LexDot,                 // .
    LexComma,               // ,
    LexSemic,               // ;
    LexArrow,               // ->
    LexStrongArrow,         // =>
    LexExclam,              // !

/*  Spec Words      */
    LexFunction,            // fun
    LexStruct,
    LexSt,
    LexVar,
    LexArr,
    LexDecl,
    LexImpl,
    LexReturn,
    LexWhile,
    LexFor,
    LexIf,
    LexElIf,
    LexElse,
    LexInclude,
    LexConst,
    LexTrue,
    LexFalse,
    LexNothing,
    LexUnsigned,
    LexPublic,
    LexPrivate,

/*  Types           */
    LexInt,
    LexChar,
    LexStr,
    LexBool,
    LexFloat,
    LexVoid,            // ...

/*  Other           */
    LexObjName,
    LexNumber,
    LexString,
}

pub fn get_lexeme_map() -> HashMap<String, &'static Lexeme> {
    HashMap::from([
        ("+".to_string(), &Lexeme::LexPlus),
        ("-".to_string(), &Lexeme::LexMinus),
        ("*".to_string(), &Lexeme::LexMulti),
        ("/".to_string(), &Lexeme::LexDiv),
        ("%".to_string(), &Lexeme::LexRem),
        ("<".to_string(), &Lexeme::LexLogicL),
        (">".to_string(), &Lexeme::LexLogicG),
        ("=".to_string(), &Lexeme::LexAssign),
        ("&".to_string(), &Lexeme::LexAmpersand),
        ("|".to_string(), &Lexeme::LexVertBar),
        ("^".to_string(), &Lexeme::LexCaret),
        (":".to_string(), &Lexeme::LexTwoDots),
        ("(".to_string(), &Lexeme::LexLParen),
        (")".to_string(), &Lexeme::LexRParen),
        ("{".to_string(), &Lexeme::LexLCParen),
        ("}".to_string(), &Lexeme::LexRCParen),
        ("[".to_string(), &Lexeme::LexLSParen),
        ("]".to_string(), &Lexeme::LexRSParen),
        (",".to_string(), &Lexeme::LexComma),
        (";".to_string(), &Lexeme::LexSemic),
        ("!".to_string(), &Lexeme::LexExclam),
        (".".to_string(), &Lexeme::LexDot),
        ("'".to_string(), &Lexeme::LexSingleQuote),
        ("\"".to_string(), &Lexeme::LexDblQuote),
        ("fun".to_string(), &Lexeme::LexFunction),
        ("struct".to_string(), &Lexeme::LexStruct),
        ("st".to_string(), &Lexeme::LexSt),
        ("var".to_string(), &Lexeme::LexVar),
        ("arr".to_string(), &Lexeme::LexArr),
        ("decl".to_string(), &Lexeme::LexDecl),
        ("impl".to_string(), &Lexeme::LexImpl),
        ("return".to_string(), &Lexeme::LexReturn),
        ("while".to_string(), &Lexeme::LexWhile),
        ("if".to_string(), &Lexeme::LexIf),
        ("else".to_string(), &Lexeme::LexElse),
        ("elif".to_string(), &Lexeme::LexElIf),
        ("for".to_string(), &Lexeme::LexFor),
        ("include".to_string(), &Lexeme::LexInclude),
        ("true".to_string(), &Lexeme::LexTrue),
        ("false".to_string(), &Lexeme::LexFalse),
        ("const".to_string(), &Lexeme::LexConst),
        ("unsigned".to_string(), &Lexeme::LexUnsigned),
        ("Nothing".to_string(), &Lexeme::LexNothing),
        ("pub".to_string(), &Lexeme::LexPublic),
        ("priv".to_string(), &Lexeme::LexPrivate),
        ("int".to_string(), &Lexeme::LexInt),
        ("char".to_string(), &Lexeme::LexChar),
        ("str".to_string(), &Lexeme::LexStr),
        ("bool".to_string(), &Lexeme::LexBool),
        ("flt".to_string(), &Lexeme::LexFloat),
        ("...".to_string(), &Lexeme::LexVoid),
    ])
}

pub fn define_lexeme(value: &String) -> &'static Lexeme {
    match get_lexeme_map().get(value) {
        None => &Lexeme::Undefined,
        Some(lexeme) => lexeme
    }
}