use crate::Lexeme as LexemeCMN;

pub fn define_lexeme_by_word(word: &String) -> LexemeCMN {
    if word == "+"          { return LexemeCMN::LexPlus; }
    if word == "-"          { return LexemeCMN::LexMinus; }
    if word == "*"          { return LexemeCMN::LexMulti; }
    if word == "/"          { return LexemeCMN::LexDiv; }
    if word == "%"          { return LexemeCMN::LexRem; }
    if word == "^"          { return LexemeCMN::LexBitXor; }
    if word == "<<"         { return LexemeCMN::LexBitLeftRot; }
    if word == ">>"         { return LexemeCMN::LexBitRightRot; }
    if word == "&&"         { return LexemeCMN::LexLogicAnd; }
    if word == "||"         { return LexemeCMN::LexLogicOr; }
    if word == "=="         { return LexemeCMN::LexLogicEq; }
    if word == "!="         { return LexemeCMN::LexLogicNEq; }
    if word == ">"          { return LexemeCMN::LexLogicG; }
    if word == "<"          { return LexemeCMN::LexLogicL; }
    if word == "<="         { return LexemeCMN::LexLogicLE; }
    if word == ">="         { return LexemeCMN::LexLogicGE; }
    if word == "="          { return LexemeCMN::LexAssign; }
    if word == "+="         { return LexemeCMN::LexAssignPlus; }
    if word == "-="         { return LexemeCMN::LexAssignMinus; }
    if word == "*="         { return LexemeCMN::LexAssignMulti; }
    if word == "/="         { return LexemeCMN::LexAssignDiv; }
    if word == "%="         { return LexemeCMN::LexAssignRem; }
    if word == "<<="        { return LexemeCMN::LexAssignBitLeftRot; }
    if word == ">>="        { return LexemeCMN::LexAssignBitRightRot; }
    if word == "|="         { return LexemeCMN::LexAssignBitOr; }
    if word == "&="         { return LexemeCMN::LexAssignBitAnd; }
    if word == "++"         { return LexemeCMN::LexInc; }
    if word == "--"         { return LexemeCMN::LexDec; }
    if word == "&"          { return LexemeCMN::LexAmpersand; }
    if word == "|"          { return LexemeCMN::LexVertBar; }
    if word == "("          { return LexemeCMN::LexLParen; }
    if word == ")"          { return LexemeCMN::LexRParen; }
    if word == "{"          { return LexemeCMN::LexLCParen; }
    if word == "}"          { return LexemeCMN::LexRCParen; }
    if word == "["          { return LexemeCMN::LexLSParen; }
    if word == "]"          { return LexemeCMN::LexRSParen; }
    if word == ":"          { return LexemeCMN::LexTwoDots; }
    if word == "::"         { return LexemeCMN::LexDblTwoDots; }
    if word == "\""         { return LexemeCMN::LexDblQuote; }
    if word == "'"          { return LexemeCMN::LexSingleQuote; }
    if word == "."          { return LexemeCMN::LexDot; }
    if word == ","          { return LexemeCMN::LexComma; }
    if word == ";"          { return LexemeCMN::LexSemic; }
    if word == "->"         { return LexemeCMN::LexArrow; }
    if word == "fun"        { return LexemeCMN::LexFunction; }
    if word == "struct"     { return LexemeCMN::LexStruct; }
    if word == "var"        { return LexemeCMN::LexVar; }
    if word == "arr"        { return LexemeCMN::LexArr; }
    if word == "decl"       { return LexemeCMN::LexDecl; }
    if word == "impl"       { return LexemeCMN::LexImpl; }
    if word == "return"     { return LexemeCMN::LexReturn; }
    if word == "call"       { return LexemeCMN::LexCall; }
    if word == "while"      { return LexemeCMN::LexWhile; }
    if word == "if"         { return LexemeCMN::LexIf; }
    if word == "elif"       { return LexemeCMN::LexElIf; }
    if word == "else"       { return LexemeCMN::LexElse; }
    if word == "include"    { return LexemeCMN::LexInclude; }
    if word == "const"      { return LexemeCMN::LexConst; }
    if word == "true"       { return LexemeCMN::LexTrue; }
    if word == "false"      { return LexemeCMN::LexFalse; }
    if word == "nothing"    { return LexemeCMN::LexNothing; }
    if word == "s8"         { return LexemeCMN::LexS8; }
    if word == "s16"        { return LexemeCMN::LexS16; }
    if word == "s32"        { return LexemeCMN::LexS32; }
    if word == "s64"        { return LexemeCMN::LexS64; }
    if word == "u8"         { return LexemeCMN::LexU8; }
    if word == "u16"        { return LexemeCMN::LexU16; }
    if word == "u32"        { return LexemeCMN::LexU32; }
    if word == "u64"        { return LexemeCMN::LexU64; }
    if word == "float"      { return LexemeCMN::LexFloat; }
    if word == "bool"       { return LexemeCMN::LexBool; }

    if is_valid_obj_name(word)      { return LexemeCMN::LexObjName; }
    if is_valid_string_value(word)  { return LexemeCMN::LexString; }
    if is_valid_number(word)        { return LexemeCMN::LexNumber; }
 
    return LexemeCMN::LexUndef;
}

pub fn is_spec_symbol(c: &char) -> bool {
    return 
        *c == ' '   ||
        *c == '&'   ||
        *c == '|'   ||
        *c == '('   ||
        *c == ')'   ||
        *c == '['   ||
        *c == ']'   ||
        *c == '{'   ||
        *c == '}'   ||
        *c == ':'   ||
        *c == '"'   ||
        *c == '\''  ||
        *c == '.'   ||
        *c == ','   ||
        *c == ';'   ||
        *c == '\n'
    ;
}

pub fn is_valid_obj_name(s: &String) -> bool {
    if s.is_empty() {
        return false;
    }

    let first_char = s.chars().next().unwrap();
    if first_char.is_digit(10) {
        return false;
    }


    for c in s.chars() {
        if !c.is_alphanumeric() && c != '_' {
            return false;
        }
    }

    return true;
}

pub fn is_valid_string_value(s: &String) -> bool {
    if s.is_empty() {
        return false;
    }

    let first_char = s.chars().next().unwrap();
    let last_char = s.chars().last().unwrap();
    
    if first_char != '"' || last_char != '"' {
        return false;
    }

    return true;
}

pub fn is_valid_number(s: &String) -> bool {
    if s.is_empty() {
        return false;
    }
    
    let first_char = s.chars().next().unwrap();
    let last_char: char = s.chars().last().unwrap();

    if  (last_char != 'u' && last_char != 'U') &&
        (last_char != 'f' && last_char != 'F') &&
        !last_char.is_digit(10) 
    {
        return false;
    }

    if first_char != '-' && !first_char.is_digit(10) {
        return false;
    }

    for i in 1..s.len() - 1 {
        if !s.chars().nth(i).unwrap().is_digit(10) {
            return false;
        }
    }
    
    return true;
}