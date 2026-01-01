pub enum Lexeme {
/*  Bin Op's        */
    LexPlus,
    LexMinus,
    LexDiv,
    LexMulti,
    LexRem,
    LexBitXor,
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

/*  Spec Words      */
    LexFunction,
    LexStruct,
    LexVar,
    LexArr,
    LexDecl,
    LexImpl,
    LexReturn,
    LexCall,
    LexWhile,
    LexIf,
    LexElIf,
    LexElse,
    LexInclude,
    LexConst,
    LexTrue,
    LexFalse,
    LexNothing,

/*  Types           */
    LexS8,
    LexS16,
    LexS32,
    LexS64,
    LexU8,
    LexU16,
    LexU32,
    LexU64,
    LexFloat,
    LexBool,

/*  Other           */
    LexObjName,
    LexNumber,
    LexString,
    LexUndef,
}