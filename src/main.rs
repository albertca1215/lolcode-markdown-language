// Lolcode Markdown Language Translation by Albert Cantoria
// Assistance provided by GitHub Copilot.

// Character-by-character lexical analyzer
// Partitions lexems into tokens using LexicalAnalyzer trait.
pub struct CharLexicalAnalyzer {
    characters: Vec<char>,
    position: usize,
    current_token: String,
    tokens: Vec<String>,
    pub hai: Vec<String>,
    pub kbye: Vec<String>,
    pub tldr: Vec<String>,
    pub maek: Vec<String>,
    pub oic: Vec<String>,
    pub gimmeh: Vec<String>,
    pub mkay: Vec<String>,
    pub head: Vec<String>,
    pub title: Vec<String>,
    pub paragraf: Vec<String>,
    pub bold: Vec<String>,
    pub italics: Vec<String>,
    pub list: Vec<String>,
    pub item: Vec<String>,
    pub newline: Vec<String>,
    pub linx: Vec<String>,
    pub ihaz: Vec<String>,
    pub itiz: Vec<String>,
    pub lemmesee: Vec<String>,
    pub varname: Vec<String>,
    pub varvalue: Vec<String>,
    pub text: Vec<String>,
    pub address: Vec<String>,
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

    fn lookup(&self, s: &str) -> bool {
        // Implementation to lookup a potential token to determine if it is valid
        // Returns true if a valid token/lexeme, false otherwise.
        self.hai.iter().any(|token| token == s) ||
        self.kbye.iter().any(|token| token == s) ||
        self.tldr.iter().any(|token| token == s) ||
        self.maek.iter().any(|token| token == s) ||
        self.oic.iter().any(|token| token == s) ||
        self.gimmeh.iter().any(|token| token == s) ||
        self.mkay.iter().any(|token| token == s) ||
        self.head.iter().any(|token| token == s) ||
        self.title.iter().any(|token| token == s) ||
        self.paragraf.iter().any(|token| token == s) ||
        self.bold.iter().any(|token| token == s) ||
        self.italics.iter().any(|token| token == s) ||
        self.list.iter().any(|token| token == s) ||
        self.item.iter().any(|token| token == s) ||
        self.newline.iter().any(|token| token == s) ||
        self.linx.iter().any(|token| token == s) ||
        self.ihaz.iter().any(|token| token == s) ||
        self.itiz.iter().any(|token| token == s) ||
        self.lemmesee.iter().any(|token| token == s) 
        
    }
}

impl CharLexicalAnalyzer {
    pub fn new(characters: Vec<char>) -> Self {
        Self { // Identifies tokens and initializes vectors for each token type
            characters,
            tokens: Vec::new(),
            position: 0,
            current_token: String::new(),
            hai: vec!["#HAI".into()],
            kbye: vec!["#KBYE".into()],
            tldr: vec!["#TLDR".into()],
            maek: vec!["#MAEK".into()],
            oic: vec!["#OIC".into()],
            gimmeh: vec!["#GIMMEH".into()],
            mkay: vec!["#MKAY".into()],
            head: vec!["HEAD".into()],
            title: vec!["TITLE".into()],
            paragraf: vec!["PARAGRAPH".into()],
            bold: vec!["BOLD".into()],
            italics: vec!["ITALICS".into()],
            list: vec!["LIST".into()],
            item: vec!["ITEM".into()],
            newline: vec!["NEWLINE".into()],
            linx: vec!["LINX".into()],
            ihaz: vec!["#IHAZ".into()],
            itiz: vec!["#ITIZ".into()],
            lemmesee: vec!["#LEMMEESEE".into()],
            varname: Vec::new(), // any single word (A-Z, a-z, 0-9, spaces)
            varvalue: Vec::new(), // any allowed text characters
            text: Vec::new(), // plain text (letters, digits, spaces, punctuation)
            address: Vec::new(), // text without spaces (for links)
        }
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

    // Classify the current token and store it in the appropriate vector based on its type.
    fn classify_token(&mut self) {
        let token = self.current_token.clone(); // Declare a variable to hold the current token being classified.
        if self.lookup(&token) {
            match token.as_str() {
                // Match the tokens against terminals defined by the grammar.
                "#HAI" => self.hai.push(token),
                "#KBYE" => self.kbye.push(token),
                "#TLDR" => self.tldr.push(token),
                "#MAEK" => self.maek.push(token),
                "#OIC" => self.oic.push(token),
                "#GIMMEH" => self.gimmeh.push(token),
                "#MKAY" => self.mkay.push(token),
                "HEAD" => self.head.push(token),
                "TITLE" => self.title.push(token),
                "PARAGRAPH" => self.paragraf.push(token),
                "BOLD" => self.bold.push(token),
                "ITALICS" => self.italics.push(token),
                "LIST" => self.list.push(token),
                "ITEM" => self.item.push(token),
                "NEWLINE" => self.newline.push(token),
                "LINX" => self.linx.push(token),
                "#IHAZ" => self.ihaz.push(token),
                "#ITIZ" => self.itiz.push(token),
                "#LEMMEESEE" => self.lemmesee.push(token),
                _ => {} // This case should not occur due to the lookup check, but it's here for completeness.
            }
        } 

        else {
            // Check if '#' appears anywhere in non-annotative tokens (always results in error and exit, since '#' is reserved for markdown annotations).
            if token.contains('#') {
                eprintln!("Error: Unrecognized token '{}'. The '#' character is only allowed at the start of markdown annotations such as #HAI, #KBYE, etc.", token);
                std::process::exit(1);
            }

            // Additional logic is applied to tokens with ambiguous values (varname, varvalue, text, or address)
            // varname takes any single word, considering all characters until a space or newline as a token.
            if token.chars().all(|c| c.is_alphanumeric()) { 
                self.varname.push(token);
            } 
            // varvalue takes any allowed text characters, which can include spaces and punctuation.
            else if token.chars().all(|c| c.is_alphanumeric() || c.is_whitespace() || c.is_ascii_punctuation()) {
                self.varvalue.push(token);
            }
            // address takes text without spaces, which can include alphanumeric characters and punctuation but not whitespace.
            else if token.chars().all(|c| c.is_alphanumeric() || c.is_ascii_punctuation()) {
                self.address.push(token);
            }
            // Lastly, when plaintext is allowed, we can classify an ambiguously long String as TEXT, which can include letters, digits, spaces, and punctuation.
            else if token.chars().all(|c| c.is_alphanumeric() || c.is_whitespace() || c.is_ascii_punctuation()) {
                self.text.push(token);
            }
            
            else {
                // Whenever a lexical error is encountered with invalid characters, the program should print an error message and terminate immediately.
                eprintln!("Error: Unrecognized token '{}'. Only alphanumeric, whitespace, and ASCII punctuation are allowed.", token);
                std::process::exit(1);
            }
        }
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
