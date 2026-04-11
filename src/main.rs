// Lolcode Markdown Language Translation by Albert Cantoria
// Assistance provided by GitHub Copilot.

// Character-by-character lexical analyzer
// Partitions lexems into tokens using LexicalAnalyzer trait.
pub struct CharLexicalAnalyzer {
    characters: Vec<char>,
    position: usize,
    current_token: String,
    pub tokens: Vec<String>,
}

impl LexicalAnalyzer for CharLexicalAnalyzer { // Implementations of the LexicalAnalyzer trait for CharLexicalAnalyzer struct
    fn get_char(&mut self) -> char {
        // Implementation to return the next character from the input
        // If input is exhausted, should terminate the program.
        if self.position >= self.characters.len() {
            panic!("Error: Unexpected end of input.");
        }
        let c = self.characters[self.position];
        self.position += 1;
        return c
    }

    fn add_char(&mut self, c: char) {
        // Implementation to add a character to the current potential token
        self.current_token.push(c);
    }

    // So, in trying to stay as close as possible to the lolcode compiler we worked on in class,
    // I tried to utilize the same logic that was used to match lexemes to tokens (articles, verbs, nouns, etc.).
    // But the issue was that in the lolcode sentence compiler, it analyzed lexemes on a word-by-word basis,
    // seperated by spaces. In this lolcode markdown compiler, we have to analyze lexemes *char-by-char*,
    // which requires different logic. Namely, in lookup(), I've decided to use the matches! macro
    // to check if the current lexeme matches any of the terminal annotative tokens defined by the grammar document.
    fn lookup(&self, s: &str) -> bool {
        // Implementation of lookup function to first see if the lexeme matches any of the
        // valid terminal tokens to check for invalid chars or invalid usage of '#'.
        // If true, categorize the lexeme into the appropriate token type.
        // If false, print and error and exit the program.

        matches!(s.to_uppercase().as_str(), 
        "#HAI" | "#KBYE" | "#OBTW" | "#TLDR" | "#MAEK" | "#OIC" | "#GIMMEH" | "#MKAY" | 
        "HEAD" | "TITLE" | "PARAGRAF" | "BOLD" | "ITALICS" | "LIST" | "ITEM" | "NEWLINE" | 
        "LINX" | "#IHAZ" | "#ITIZ" | "#LEMMEESEE"
        )
        
    }
}

impl CharLexicalAnalyzer {
    pub fn is_a_hai(&self, token: &str) -> bool {
        matches!(token.to_uppercase().as_str(), "#HAI")
    }

    pub fn is_a_kbye(&self, token: &str) -> bool {
        matches!(token.to_uppercase().as_str(), "#KBYE")
    }

    pub fn is_a_obtw(&self, token: &str) -> bool {
        matches!(token.to_uppercase().as_str(), "#OBTW")
    }

    pub fn is_a_tldr(&self, token: &str) -> bool {
        matches!(token.to_uppercase().as_str(), "#TLDR")
    }

    pub fn is_a_maek(&self, token: &str) -> bool {
        matches!(token.to_uppercase().as_str(), "#MAEK")
    }

    pub fn is_a_oic(&self, token: &str) -> bool {
        matches!(token.to_uppercase().as_str(), "#OIC")
    }

    pub fn is_a_gimmeh(&self, token: &str) -> bool {
        matches!(token.to_uppercase().as_str(), "#GIMMEH")
    }

    pub fn is_a_mkay(&self, token: &str) -> bool {
        matches!(token.to_uppercase().as_str(), "#MKAY")
    }

    pub fn is_a_head(&self, token: &str) -> bool {
        matches!(token.to_uppercase().as_str(), "HEAD")
    }

    pub fn is_a_title(&self, token: &str) -> bool {
        matches!(token.to_uppercase().as_str(), "TITLE")
    }

    pub fn is_a_paragraf(&self, token: &str) -> bool {
        matches!(token.to_uppercase().as_str(), "PARAGRAF")
    }

    pub fn is_a_bold(&self, token: &str) -> bool {
        matches!(token.to_uppercase().as_str(), "BOLD")
    }

    pub fn is_a_italics(&self, token: &str) -> bool {
        matches!(token.to_uppercase().as_str(), "ITALICS")
    }

    pub fn is_a_list(&self, token: &str) -> bool {
        matches!(token.to_uppercase().as_str(), "LIST")
    }

    pub fn is_a_item(&self, token: &str) -> bool {
        matches!(token.to_uppercase().as_str(), "ITEM")
    }

    pub fn is_a_newline(&self, token: &str) -> bool {
        matches!(token.to_uppercase().as_str(), "NEWLINE")
    }

    pub fn is_a_linx(&self, token: &str) -> bool {
        matches!(token.to_uppercase().as_str(), "LINX")
    }

    pub fn is_a_ihaz(&self, token: &str) -> bool {
        matches!(token.to_uppercase().as_str(), "#IHAZ")
    }

    pub fn is_a_itiz(&self, token: &str) -> bool {
        matches!(token.to_uppercase().as_str(), "#ITIZ")
    }

    pub fn is_a_lemmesee(&self, token: &str) -> bool {
        matches!(token.to_uppercase().as_str(), "#LEMMEESEE")
    }

    // For token types that can take ambiguously long lexemes (VARNAME, VARVALUE, TEXT, and ADDRESS),
    // additional logic will be needed to determine which token the lexeme falls under.

    // VARNAME can only be made up of letters with no spaces.
    pub fn is_a_varname(&self, token: &str) -> bool {
        token.chars().all(|c| c.is_alphabetic())
    }

    // VARVALUE can contain letters and numbers (alphaneumeric) with no spaces.
    pub fn is_a_varvalue(&self, token: &str) -> bool {
        token.chars().all(|c| c.is_alphanumeric())
    }

    // TEXT can contain plain text with spaces, as long as no invalid chars ('#') are used.
    pub fn is_a_text(&self, token: &str) -> bool {
        !token.contains('#') && token.chars().all(|c| c.is_alphanumeric() || c.is_whitespace() || c.is_ascii_punctuation())
    }

    // ADDRESS can contain any valid chars with no spaces.
    // As long as it doesn't have spaces, assume that it's a valid lexeme,
    // regardless of whether or not the lexeme is a valid URL address.
    pub fn is_a_address(&self, token: &str) -> bool {
        !token.contains(' ') && token.chars().all(|c| c.is_alphanumeric() || c.is_ascii_punctuation())
    }





    // Tokenize input characters into tokens of type String, using classify_token to determine token types and store them in the appropriate vectors.
    pub fn tokenize(&mut self) {
        while self.position < self.characters.len() {
            let c = self.get_char();
            
            if c == ' ' || c == '\n' {
                if !self.current_token.is_empty() {
                    self.classify_token();
                    self.current_token.clear();
                }
            } else {
                self.add_char(c);
            }
        }
        // Check in case the last token is not followed by a space or newline
        if !self.current_token.is_empty() {
            self.classify_token();
        }
    }

    // Classify the current token and store it in the tokens vector.
    fn classify_token(&mut self) {
        let token = self.current_token.clone();
        if self.lookup(&token) {
            // Valid keyword token
            self.tokens.push(token);
        } else {
            // Check for invalid '#' usage
            if token.contains('#') {
                eprintln!("Error: Unrecognized token '{}'. The '#' character is only allowed at the start of markdown annotations such as #HAI, #KBYE, etc.", token);
                std::process::exit(1);
            }
            // Check for invalid characters
            if !token.chars().all(|c| c.is_alphanumeric() || c.is_whitespace() || c.is_ascii_punctuation()) {
                eprintln!("Error: Unrecognized token '{}'. Only alphanumeric, whitespace, and ASCII punctuation are allowed.", token);
                std::process::exit(1);
            }
            // Valid non-keyword token
            self.tokens.push(token);
        }
    }

    pub fn next_token(&mut self) -> String {
        if self.tokens.is_empty() {
            String::new()
        } else {
            self.tokens.remove(0)
        }
    }
}    
// ------------------------------------------------------------------------------

// Recursive-descent syntax analyzer using the SyntaxAnalyzer trait.
// Conceptually builds a syntax tree from the tokens produced by the lexical analyzer.

// Declares 'LolcodeCompiler' data type, which takes a String input.
pub struct LolcodeCompiler {
    pub current_token: String,
}

impl LolcodeCompiler {
    pub fn new() -> Self {
        Self {
            current_token: String::new(),
        }
    }
}

impl Compiler for LolcodeCompiler {
    fn compile(&mut self, _source: &str) {
        // Entry point for compilation process
    }

    fn next_token(&mut self) -> String {
        self.current_token.clone()
    }

    fn parse(&mut self) {
        // Run syntax analyzer
    }

    fn current_token(&self) -> String {
        self.current_token.clone()
    }

    fn set_current_token(&mut self, tok: String) {
        self.current_token = tok;
    }
}

// The Parser struct holds mutable references to both a CharLexicalAnalyzer and a LolcodeCompiler instance.
pub struct Parser<'a> {
    lexer: &'a mut CharLexicalAnalyzer,
    compiler: &'a mut LolcodeCompiler,
}

impl<'a> Parser<'a> {
    // Constructor for Parser struct, taking mutable references to a CharLexicalAnalyzer and a LolcodeCompiler instance.
    pub fn new(lexer: &'a mut CharLexicalAnalyzer, compiler: &'a mut LolcodeCompiler) -> Self {
        Self { lexer, compiler }
    }

    // next_token() method that retrieves the next token from the lexer and sets it as the current token in the compiler.
    pub fn next_token(&mut self) {
        let tok = self.lexer.next_token();
        self.compiler.set_current_token(tok);
    }
}

impl<'a> SyntaxAnalyzer for Parser<'a> {
    // Here are the implementations of the SyntaxAnalyzer trait for the Parser struct, where the meat of
    // snytax analysis will be done. Each function corresponds to a token type defined in the grammar.

    fn parse_lolcode(&mut self) -> Result<(), String> {
        // Non-terminal for the entire lolcode document.
        // Form: #HAI <comments> <head> <body> #Kbye

        if self.lexer.is_a_hai(&self.compiler.current_token()) {
            // Check for #HAI token
            self.next_token(); // If #HAI is found, get the next token
        } else {
            return Err(format!("Error: Expected '#HAI' at the beginning of the document when {} was found.", self.compiler.current_token()));
        }

        self.parse_comment()?; // Parse comments (optional)
        self.parse_head()?; // Parse head (optional)
        self.parse_body()?; // Parse body (optional)

        if self.lexer.is_a_kbye(&self.compiler.current_token()) {
            // Check for #KBYE token
            self.next_token(); // If #KBYE is found, get the next token
        } else {
            return Err(format!("Error: Expected '#KBYE' at the end of the document when {} was found.", self.compiler.current_token()));
        }

        Ok(())
    }

    fn parse_head(&mut self) -> Result<(), String> {
        // Non-terminal for the optional head section of the document.
        // Form: #MAEK HEAD <title> #MKAY

        // Because head is optional, and it starts with the #MAEK token,
        // check if the current token is #MAEK. If true, then parse the entire head section.
        // If false, then return OK(()) and exit parse_head.
        if !self.lexer.is_a_maek(&self.compiler.current_token()) {
            return Ok(());
        } else {
            self.next_token(); // If #MAEK is found, get the next token
        }

        if self.lexer.is_a_head(&self.compiler.current_token()) {
            // Check for HEAD token
            self.next_token(); // If HEAD is found, get the next token
        } else {
            return Err(format!("Error: Expected 'HEAD' in the head section when {} was found.", self.compiler.current_token()));
        }

        self.parse_title()?; // Parse title

        if self.lexer.is_a_mkay(&self.compiler.current_token()) {
            // Check for #MKAY token
            self.next_token(); // If #MKAY is found, get the next token
        } else {
            return Err(format!("Error: Expected '#MKAY' at the end of the head section when {} was found.", self.compiler.current_token()));
        }
        Ok(())
    }

    fn parse_title(&mut self) -> Result<(), String> {
        // Non-terminal for the title of the document, which is a required part of the head section.
        // Form: #GIMMEH TITLE <text> #OIC

        if self.lexer.is_a_gimmeh(&self.compiler.current_token()) {
            // Check for #GIMMEH token
            self.next_token(); // If #GIMMEH is found, get the next token
        } else {
            return Err(format!("Error: Expected '#GIMMEH' at the beginning of the title section when {} was found.", self.compiler.current_token()));
        }

        if self.lexer.is_a_title(&self.compiler.current_token()) {
            // Check for TITLE token
            self.next_token(); // If TITLE is found, get the next token
        } else {
            return Err(format!("Error: Expected 'TITLE' in the title section when {} was found.", self.compiler.current_token()));
        }

        self.parse_text()?; // Text is a terminal token, so we can just parse it here.

        if self.lexer.is_a_oic(&self.compiler.current_token()) {
            // Check for #OIC token
            self.next_token(); // If #OIC is found, get the next token
        } else {
            return Err(format!("Error: Expected '#OIC' at the end of the title section when {} was found.", self.compiler.current_token()));
        }   

        Ok(())
    }

    fn parse_comment(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn parse_body(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn parse_paragraph(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn parse_inner_paragraph(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn parse_inner_text(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn parse_variable_define(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn parse_variable_use(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn parse_bold(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn parse_italics(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn parse_list(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn parse_list_items(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn parse_inner_list(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn parse_link(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn parse_newline(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn parse_text(&mut self) -> Result<(), String> {
        // Text is a terminal token, so we can just check if the current token is valid text and then get the next token.
        if self.lexer.is_a_text(&self.compiler.current_token()) {
            self.next_token(); // If it's valid text, get the next token
        } else {
            return Err(format!("Error: Expected valid text when {} was found.", self.compiler.current_token()));
        }

        Ok(())
    }
}


fn main() {
    println!("Hello, world!");
}




// Below traits are provided by course professor, Dr. Josh Dehlinger.
// ------------------------------------------------------------------------------
/// Trait for a simple lolcompiler front-end. 
/// Errors should cause immediate exit inside the implementation.
pub trait Compiler {
    /// Begin the compilation process (entry point).
    fn compile(&mut self, source: &str);

    /// Get the next token from the lexical analyzer.
    fn next_token(&mut self) -> String;

    /// Run the syntax analyzer starting from <lolcode>.
    fn parse(&mut self);

    /// Get the current token being processed.
    fn current_token(&self) -> String;

    /// Set the current token (typically used internally).
    fn set_current_token(&mut self, tok: String);
}

/// Trait for a simple lexical analyzer.
/// Implements a character-by-character analysis
/// from a state machine design.
pub trait LexicalAnalyzer {
    /// Return the next character from the input.
    /// If input is exhausted, should terminate the program.
    fn get_char(&mut self) -> char;

    /// Add a character to the current potential token.
    fn add_char(&mut self, c: char);

    /// Lookup a potential token to determine if it is valid.
    /// Returns true if a valid token/lexeme, false otherwise.
    fn lookup(&self, s: &str) -> bool;
}

/// Trait for a recursive descent Syntax Analyzer 
/// over Vec<String>. Each function parses a nonterminal in 
/// the grammar. On error: return Err(message), on success: Ok(()).
pub trait SyntaxAnalyzer {
    fn parse_lolcode(&mut self) -> Result<(), String>;        
    fn parse_head(&mut self) -> Result<(), String>;           
    fn parse_title(&mut self) -> Result<(), String>;          
    fn parse_comment(&mut self) -> Result<(), String>;        
    fn parse_body(&mut self) -> Result<(), String>;           
    fn parse_paragraph(&mut self) -> Result<(), String>;      
    fn parse_inner_paragraph(&mut self) -> Result<(), String>;
    fn parse_inner_text(&mut self) -> Result<(), String>;     
    fn parse_variable_define(&mut self) -> Result<(), String>;
    fn parse_variable_use(&mut self) -> Result<(), String>;   
    fn parse_bold(&mut self) -> Result<(), String>;           
    fn parse_italics(&mut self) -> Result<(), String>;        
    fn parse_list(&mut self) -> Result<(), String>;           
    fn parse_list_items(&mut self) -> Result<(), String>;     
    fn parse_inner_list(&mut self) -> Result<(), String>;     
    fn parse_link(&mut self) -> Result<(), String>;          
    fn parse_newline(&mut self) -> Result<(), String>;        
    fn parse_text(&mut self) -> Result<(), String>;           
}
