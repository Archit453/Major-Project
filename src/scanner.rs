
pub struct Scanner{}

impl Scanner{
    pub fn new(_source: &str) -> Self{
        Self{}
    }
    pub fn scan_tokens(self:&Self) -> Result<Vec<Token>, String>{
        todo!()
    }
}
#[derive(Debug)]
pub struct TokenType {
    // Single-character tokens.
    left_paren;
    right_paren; 
    left_brace; 
    right_brace;
    comma;
    dot; 
    minus; 
    plus; 
    semicolon; 
    slash; 
    star;
    // One or two character tokens.
    bang; 
    bang_equal;
    equal;
    equal_equal;
    greater; 
    greater_equal;
    less; 
    less_equal;
    // Literals.
    identifier; 
    string; 
    number;
    // Keywords.
    and; 
    class; 
    else; 
    false; 
    fun; 
    for; 
    if; 
    nil; 
    or;
    print;
    return; 
    super;
    this; 
    true; 
    var; 
    while;
    eof;
}

#[derive(Debug)]
pub enum LiteralValue{
    IntValue(i64),
    Fvalue(f64),
    StringValue(String),
    Identifiers(String),
}
#[derive(Debug)]
pub struct Token{
    token_type: TokenType,
    lexeme: String,
    literal:LiteralValue,
    line_number: u64,
}

impl Token{
    pub fn new(token_type: TokenType,lexeme: String, literal:LiteralValue, line_number: u64 )-> Self{
        Self{
            token_type,
            lexeme,
            literal,
            line_number,
        }
    }

    pub fn to_string(self: &Self) -> String {
        format!("{} {} {}",self.token_type,self.lexeme,self.literal)
    }
}