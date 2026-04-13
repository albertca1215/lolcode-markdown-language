// Lolcode Markdown Language Translation by Albert Cantoria
// Assistance provided by GitHub Copilot.

use std::collections::HashMap; // Imports the HashMap type from the Rust library, which is used as a symbol table to keep track of variable definitions and their scopes during semantic analysis.
use std::process::Command;
use std::path::Path; // Both Command and Path are needed to open an HTML file using the default browser.
use std::fs;
use std::env;

// ============================================================================
// LEXICAL ANALYSIS
// ============================================================================

// Character-by-character lexical analyzer
// Partitions lexems into tokens using LexicalAnalyzer trait.
pub struct CharLexicalAnalyzer {
    characters: Vec<char>,
    position: usize,
    current_token: String,
    pub tokens: Vec<String>,
    current_line: usize, // Track line number for error reporting
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
        // Track line numbers for error reporting
        if c == '\n' {
            self.current_line += 1;
        }
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
        "LINX" | "#IHAZ" | "#ITIZ" | "#LEMMESEE"
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
        matches!(token.to_uppercase().as_str(), "#LEMMESEE")
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
            
            if c == ' ' || c == '\n' || c == '\t' || c == '\r' {
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
        let token = self.current_token.trim().to_string();
        
        if token.is_empty() {
            return; // Skip empty tokens
        }
        
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

// ============================================================================
// SYNTAX ANALYSIS
// ============================================================================

// The Parser struct holds mutable references to both a CharLexicalAnalyzer and a LolcodeCompiler instance,
// plus a symbol table for semantic variable resolution and AST building for HTML translation.
pub struct Parser<'a> {
    lexer: &'a mut CharLexicalAnalyzer,
    compiler: &'a mut LolcodeCompiler,
    symbol_table: VariableSymbolTable,
    current_line: usize,

    // Throughout the parsing process, an abstract snytax tree (AST) is built to represent the hierarchical structure
    // of the LOLCODE document and its elements. Using vectors holding ASTNodes enums, the AST captures and stores the
    // elements in the document and their syntactic placement, which can then be used for translation into HTML later.
    ast_root: Option<ASTNode>,
    body_elements: Vec<ASTNode>,
    current_title: String,
    paragraph_buffer: Vec<ASTNode>,
    list_items_buffer: Vec<ASTNode>,
    in_body_context: bool,  // Track if we're parsing body-level elements
}

impl<'a> Parser<'a> {
    // Constructor for Parser struct, taking mutable references to a CharLexicalAnalyzer and a LolcodeCompiler instance.
    pub fn new(lexer: &'a mut CharLexicalAnalyzer, compiler: &'a mut LolcodeCompiler) -> Self {
        Self {
            lexer,
            compiler,
            symbol_table: VariableSymbolTable::new(),
            current_line: 1,
            ast_root: None,

            // 
            body_elements: Vec::new(),
            current_title: String::new(),
            paragraph_buffer: Vec::new(),
            list_items_buffer: Vec::new(),
            in_body_context: true,
        }
    }

    /// Extract and return the built AST
    pub fn get_ast(self) -> Option<ASTNode> {
        self.ast_root
    }

    // next_token() method that retrieves the next token from the lexer and sets it as the current token in the compiler.
    pub fn next_token(&mut self) {
        let tok = self.lexer.next_token();
        self.compiler.set_current_token(tok);
        // No actual line number update here since tokens don't carry line info;
        // line tracking happens at character level in lexer
    }

    // New method for parsing formatting elements, which is used as the entry point for all formatting types that use the #GIMMEH token.
    fn parse_formatting(&mut self) -> Result<(), String> {
        // Dispatcher for all #GIMMEH-based formatting: BOLD, ITALICS, LINX, NEWLINE
        // Entry point: current token is #GIMMEH
        // Note: TITLE is only used in <head>, not in formatting

        if self.lexer.is_a_gimmeh(&self.compiler.current_token()) {
            self.next_token(); // Consume #GIMMEH
        }

        // Dispatch based on formatting type
        if self.lexer.is_a_bold(&self.compiler.current_token()) {
            self.parse_bold()?;
        } else if self.lexer.is_a_italics(&self.compiler.current_token()) {
            self.parse_italics()?;
        } else if self.lexer.is_a_linx(&self.compiler.current_token()) {
            self.parse_link()?;
        } else if self.lexer.is_a_newline(&self.compiler.current_token()) {
            self.parse_newline()?;
        } else {
            return Err(format!("Error: Expected formatting type (BOLD, ITALICS, LINX, NEWLINE) after '#GIMMEH' when {} was found.", self.compiler.current_token()));
        }

        Ok(())
    }

    fn parse_comments(&mut self) -> Result<(), String> {
        // New function for parsing comments, which can optionally appear after #HAI and before the head section.
        // Form: <comment> <comments> | ε

        // If #OBTW doesn't appear, then the comment section is empty and we can exit with Ok(()).
        if !self.lexer.is_a_obtw(&self.compiler.current_token()) {
            return Ok(());
        }

        self.parse_comment()?; // Parse one comment, as required after #OBTW appears.
        self.parse_comments()?; // Optionally, recursively parse more comments, if #OBTW appears again in the token stream.

        Ok(())
    }

    fn parse_inner_body(&mut self) -> Result<(), String> {
        // Dispatcher for body-level elements.
        // Form: <paragraph> | <comment> | <bold> | <italics> | <list> | <link> | <newline> | <variable-define> | <variable-use> | TEXT

        if self.lexer.is_a_maek(&self.compiler.current_token()) {
            self.next_token(); // Consume #MAEK
            if self.lexer.is_a_paragraf(&self.compiler.current_token()) {
                self.parse_paragraph()?;
            } else if self.lexer.is_a_list(&self.compiler.current_token()) {
                self.parse_list()?;
            } else {
                return Err(format!("Error: Expected 'PARAGRAF' or 'LIST' after '#MAEK' when {} was found.", self.compiler.current_token()));
            }
        } else if self.lexer.is_a_obtw(&self.compiler.current_token()) {
            self.parse_comment()?;
        } else if self.lexer.is_a_gimmeh(&self.compiler.current_token()) {
            self.parse_formatting()?;
        } else if self.lexer.is_a_ihaz(&self.compiler.current_token()) {
            self.parse_variable_define()?;
        } else if self.lexer.is_a_lemmesee(&self.compiler.current_token()) {
            self.parse_variable_use()?;
        } else if self.lexer.is_a_text(&self.compiler.current_token()) {
            self.parse_text()?;
        } else {
            return Err(format!("Error: Unrecognized token '{}' in body section.", self.compiler.current_token()));
        }

        Ok(())
    }

    fn can_be_inner_text(&self) -> bool {
        // New function called by parse_inner_paragraph to check that the current token strictly remains within the types of tokens that can exist in inner paragraph text (plaintext, formatted text, variable usage, and lists).
        self.lexer.is_a_text(&self.compiler.current_token())
            || self.lexer.is_a_gimmeh(&self.compiler.current_token())
            || self.lexer.is_a_maek(&self.compiler.current_token())
            || self.lexer.is_a_lemmesee(&self.compiler.current_token())
    }

    /// Helper: Accumulate consecutive TEXT tokens into a single string
    fn accumulate_text(&mut self) -> String {
        let mut accumulated = String::new();
        
        while self.lexer.is_a_text(&self.compiler.current_token()) {
            if !accumulated.is_empty() {
                accumulated.push(' ');
            }
            accumulated.push_str(&self.compiler.current_token());
            self.next_token();
        }
        
        accumulated
    }

    fn parse_list_item(&mut self) -> Result<(), String> {
        // Non-terminal for a single list item.
        // Form: GIMMEH ITEM <inner_list> OIC

        if self.lexer.is_a_gimmeh(&self.compiler.current_token()) {
            self.next_token(); // Consume #GIMMEH
        } else {
            return Err(format!("Error: Expected '#GIMMEH' at start of list item when {} was found.", self.compiler.current_token()));
        }

        if self.lexer.is_a_item(&self.compiler.current_token()) {
            self.next_token(); // Consume ITEM
        } else {
            return Err(format!("Error: Expected 'ITEM' after '#GIMMEH' when {} was found.", self.compiler.current_token()));
        }

        // Use a temporary buffer for this list item's content
        let temp_buffer = self.list_items_buffer.clone();
        self.list_items_buffer.clear();

        self.parse_inner_list()?; // Parse item content

        if self.lexer.is_a_oic(&self.compiler.current_token()) {
            self.next_token(); // Consume #OIC
        } else {
            return Err(format!("Error: Expected '#OIC' at end of list item when {} was found.", self.compiler.current_token()));
        }

        // Create a ListItem node with the collected content
        let item_content = self.list_items_buffer.drain(..).collect();
        let list_item = ASTNode::ListItem(item_content);

        // Restore previous buffer and add the item to it
        self.list_items_buffer = temp_buffer;
        self.list_items_buffer.push(list_item);

        Ok(())
    }

}

impl<'a> SyntaxAnalyzer for Parser<'a> {
    // Here are the implementations of the SyntaxAnalyzer trait for the Parser struct, where the meat of
    // snytax analysis will be done. Each function corresponds to a token type defined in the grammar.

    fn parse_lolcode(&mut self) -> Result<(), String> {
        // Non-terminal for the entire lolcode document.
        // Form: HAI <comments> <head> <body> KBYE

        if self.lexer.is_a_hai(&self.compiler.current_token()) {
            // Check for #HAI token
            self.next_token(); // If #HAI is found, get the next token
        } else {
            return Err(format!("Error: Expected '#HAI' at the beginning of the document when {} was found.", self.compiler.current_token()));
        }

        self.parse_comments()?; // Parse comments (recursive, zero or more)
        self.parse_head()?; // Parse head (optional)
        self.parse_body()?; // Parse body (recursive)

        if self.lexer.is_a_kbye(&self.compiler.current_token()) {
            // Check for #KBYE token
            self.next_token(); // If #KBYE is found, get the next token
        } else {
            return Err(format!("Error: Expected '#KBYE' at the end of the document when {} was found.", self.compiler.current_token()));
        }

        // Build the complete AST
        let head = ASTNode::Head {
            title: self.current_title.clone(),
        };
        self.ast_root = Some(ASTNode::Document {
            head: Box::new(head),
            body: self.body_elements.clone(),
        });

        Ok(())
    }

    fn parse_head(&mut self) -> Result<(), String> {
        // Non-terminal for the head section of the document.
        // Form: MAEK HEAD <title> MKAY | ε (optional)

        // Because the head section is optional, we can check if the current token is #MAEK. If it is, then we can use it as the entry point for parse_head.
        // If not, we assume that the head section is empty and exit with Ok(()).
        if !self.lexer.is_a_maek(&self.compiler.current_token()) {
            return Ok(());
        }

        self.next_token(); // Consume #MAEK

        if self.lexer.is_a_head(&self.compiler.current_token()) {
            self.next_token(); // Consume HEAD token
        } else {
            return Err(format!("Error: Expected 'HEAD' after '#MAEK' when {} was found.", self.compiler.current_token()));
        }

        self.parse_title()?; // Parse title

        if self.lexer.is_a_mkay(&self.compiler.current_token()) {
            self.next_token(); // Consume #MKAY token
        } else {
            return Err(format!("Error: Expected '#MKAY' at the end of the head section when {} was found.", self.compiler.current_token()));
        }
        Ok(())
    }

    fn parse_title(&mut self) -> Result<(), String> {
        // Non-terminal for the title of the document.
        // Form: GIMMEH TITLE TEXT OIC

        if self.lexer.is_a_gimmeh(&self.compiler.current_token()) {
            self.next_token(); // Consume #GIMMEH
        } else {
            return Err(format!("Error: Expected '#GIMMEH' for title when {} was found.", self.compiler.current_token()));
        }

        if self.lexer.is_a_title(&self.compiler.current_token()) {
            self.next_token(); // Consume TITLE
        } else {
            return Err(format!("Error: Expected 'TITLE' when {} was found.", self.compiler.current_token()));
        }

        // Accumulate consecutive TEXT tokens for the title
        let title_text = self.accumulate_text();
        if title_text.is_empty() {
            return Err(format!("Error: Expected TEXT for title when {} was found.", self.compiler.current_token()));
        }

        if self.lexer.is_a_oic(&self.compiler.current_token()) {
            self.next_token(); // Consume #OIC
        } else {
            return Err(format!("Error: Expected '#OIC' at end of title when {} was found.", self.compiler.current_token()));
        }

        // Store title for AST
        self.current_title = title_text;

        Ok(())
    }

    fn parse_comment(&mut self) -> Result<(), String> {
        // Non-terminal for comments.
        // Form: OBTW TEXT TLDR

        if self.lexer.is_a_obtw(&self.compiler.current_token()) {
            self.next_token(); // Consume #OBTW
        } else {
            return Err(format!("Error: Expected '#OBTW' at start of comment when {} was found.", self.compiler.current_token()));
        }

        // Accumulate consecutive TEXT tokens for the comment content
        let comment_text = self.accumulate_text();
        if comment_text.is_empty() {
            return Err(format!("Error: Expected TEXT in comment when {} was found.", self.compiler.current_token()));
        }

        if self.lexer.is_a_tldr(&self.compiler.current_token()) {
            self.next_token(); // Consume #TLDR
        } else {
            return Err(format!("Error: Expected '#TLDR' at end of comment when {} was found.", self.compiler.current_token()));
        }

        // Add comment to body elements
        self.body_elements.push(ASTNode::Comment(comment_text));

        Ok(())
    }

    fn parse_body(&mut self) -> Result<(), String> {
        // Non-terminal for the body of a document.
        // Form: <inner-body> <body> | ε (recursive)

        // Check first if we've hit the end of the body with #KBYE. If so, exit with Ok(()), but if not, start/continue parsing body elements.
        if self.lexer.is_a_kbye(&self.compiler.current_token()) {
            return Ok(());
        }

        // Otherwise, parse one inner-body element
        self.parse_inner_body()?;
        
        // Recursively parse more body elements
        self.parse_body()?;

        Ok(())
    }

    fn parse_paragraph(&mut self) -> Result<(), String> {
        // Non-terminal for paragraphs, which appear as a body element.
        // Assume: #MAEK already consumed, current token is PARAGRAF
        // Form: PARAGRAF <variable_define> <inner_paragraph> MKAY

        if self.lexer.is_a_paragraf(&self.compiler.current_token()) {
            self.next_token(); // Consume PARAGRAF token
        } else {
            return Err(format!("Error: Expected 'PARAGRAF' when {} was found.", self.compiler.current_token()));
        }

        // Clear paragraph buffer for new paragraph
        self.paragraph_buffer.clear();

        // Enter paragraph scope for variable isolation and set context
        self.symbol_table.enter_scope();
        self.in_body_context = false;

        self.parse_variable_define()?; // Variables are optional in paragraphs
        self.parse_inner_paragraph()?; // Parse inner paragraph content

        if self.lexer.is_a_mkay(&self.compiler.current_token()) {
            self.next_token(); // Consume #MKAY token
        } else {
            // Exit scope and context before returning error
            self.symbol_table.exit_scope();
            self.in_body_context = true;
            return Err(format!("Error: Expected '#MKAY' at end of paragraph when {} was found.", self.compiler.current_token()));
        }

        // Exit paragraph scope and reset context
        self.symbol_table.exit_scope();
        self.in_body_context = true;

        // Add paragraph to body elements
        let paragraph = ASTNode::Paragraph(self.paragraph_buffer.drain(..).collect());
        self.body_elements.push(paragraph);

        Ok(())
    }

    fn parse_inner_paragraph(&mut self) -> Result<(), String> {
        // Non-terminal for inner paragraph content.
        // Form: <inner_text> <inner_paragraph> | ε (recursive)

        // Paragraphs can only have certain types of content (plaintext, formatted text, variable usage, and lists), so we'll first check
        // if the current token falls under one of those categories. If it returns true, we assume we've hit the end of the paragraph's contents and exit with Ok(()).
        // If false, we parse an inner_text element, and then recursively parse to re-check for more inner_text elements.
        if !self.can_be_inner_text() {
            return Ok(());
        }

        self.parse_inner_text()?; // Parse one element
        self.parse_inner_paragraph()?; // Recursively parse more

        Ok(())
    }

    fn parse_inner_text(&mut self) -> Result<(), String> {
        // Non-terminal for inner text content within paragraphs.
        // Form: <variable_use> | <bold> | <italics> | <list> | <link> | <newline> | TEXT | ε

        if self.lexer.is_a_text(&self.compiler.current_token()) {
            self.parse_text()?;
        } else if self.lexer.is_a_gimmeh(&self.compiler.current_token()) {
            self.parse_formatting()?;
        } else if self.lexer.is_a_maek(&self.compiler.current_token()) {
            // Lists can appear in inner text
            self.next_token(); // Consume #MAEK
            if self.lexer.is_a_list(&self.compiler.current_token()) {
                self.parse_list()?;
            } else {
                return Err(format!("Error: Expected 'LIST' after '#MAEK' in inner text when {} was found.", self.compiler.current_token()));
            }
        } else if self.lexer.is_a_lemmesee(&self.compiler.current_token()) {
            self.parse_variable_use()?;
        } else {
            // Not valid inner text - this is ε case
            return Ok(());
        }

        Ok(())
    }

    fn parse_variable_define(&mut self) -> Result<(), String> {
        // Non-terminal for variable declarations.
        // Form: IHAZ VARNAME ITIZ VARVALUE MKAY | ε (optional)
        // Semantic action: Store variable in symbol table

        // Variable declarations are completely option, so we can use the token #IHAZ (exclusively used for var declarations) as the entry point for this function.
        // If the current token is #IHAZ, we assume we're parsing a variable declaration. If not, we exit with Ok(()).
        if !self.lexer.is_a_ihaz(&self.compiler.current_token()) {
            return Ok(());
        }

        let def_line = self.current_line; // Capture line number for semantic tracking
        self.next_token(); // Consume #IHAZ

        // VARNAME must be all letters
        let varname = if self.lexer.is_a_varname(&self.compiler.current_token()) {
            let name = self.compiler.current_token().clone();
            self.next_token(); // Consume VARNAME
            name
        } else {
            return Err(format!("Error: Expected VARNAME after '#IHAZ' when {} was found.", self.compiler.current_token()));
        };

        if self.lexer.is_a_itiz(&self.compiler.current_token()) {
            self.next_token(); // Consume #ITIZ
        } else {
            return Err(format!("Error: Expected '#ITIZ' in variable declaration when {} was found.", self.compiler.current_token()));
        }

        // VARVALUE can be alphanumeric
        let varvalue = if self.lexer.is_a_varvalue(&self.compiler.current_token()) {
            let value = self.compiler.current_token().clone();
            self.next_token(); // Consume VARVALUE
            value
        } else {
            return Err(format!("Error: Expected VARVALUE after '#ITIZ' when {} was found.", self.compiler.current_token()));
        };

        if self.lexer.is_a_mkay(&self.compiler.current_token()) {
            self.next_token(); // Consume #MKAY
        } else {
            return Err(format!("Error: Expected '#MKAY' at end of variable declaration when {} was found.", self.compiler.current_token()));
        }

        // SEMANTIC ACTION: Define variable in symbol table
        self.symbol_table.define(varname, def_line, varvalue)?;

        Ok(())
    }

    fn parse_variable_use(&mut self) -> Result<(), String> {
        // Non-terminal for variable usage, which can optionally appear in the body element, or in the inner_paragraph non-terminal.
        // Form: #LEMMEESEE <varname> #OIC
        // Semantic action: Validate variable exists (use-before-definition check)

        // #LEMMEESEE is exclusively used for variable usage, so in a similar way, we can use it as the entry point for
        // parse_variable_use. So if the current token is #LEMMEESEE, we assume that we're parsing a variable usage. If not, we exit with Ok(()).
        if !self.lexer.is_a_lemmesee(&self.compiler.current_token()) {
            return Ok(());
        } else {
            self.next_token(); // If #LEMMEESEE is found, get the next token
        }

        // VARNAME is a token holding an ambiguously long String containing only letters with no spaces, so we need to check if the current token is a valid VARNAME.
        // If true, get the next token and continue parsing. If false, return an error and exit.
        let varname = if self.lexer.is_a_varname(&self.compiler.current_token()) {
            let name = self.compiler.current_token().clone();
            self.next_token(); // If it's a valid VARNAME, get the next token
            name
        } else {
            return Err(format!("Error: Expected a valid variable name after '#LEMMEESEE' when {} was found.", self.compiler.current_token()));
        };

        // Check for #OIC and exit.
        if self.lexer.is_a_oic(&self.compiler.current_token()) {
            self.next_token(); // If #OIC is found, get the next token
        } else {
            return Err(format!("Error: Expected '#OIC' at the end of the variable usage when {} was found.", self.compiler.current_token()));
        }

        // SEMANTIC ACTION: Validate variable exists in symbol table (use-before-definition check)
        if self.symbol_table.lookup(&varname).is_none() {
            return Err(format!("Error: Variable '{}' used before definition.", varname));
        }

        // Add the variable value as text to appropriate buffer
        if let Some((_, value)) = self.symbol_table.lookup(&varname) {
            if self.in_body_context {
                self.body_elements.push(ASTNode::Paragraph(vec![ASTNode::Text(value)]));
            } else {
                self.paragraph_buffer.push(ASTNode::Text(value));
            }
        }

        Ok(())
    }

    fn parse_bold(&mut self) -> Result<(), String> {
        // Non-terminal for formatting bold text.
        // Assume: #GIMMEH already consumed, current token is BOLD
        // Form: BOLD TEXT OIC

        if self.lexer.is_a_bold(&self.compiler.current_token()) {
            self.next_token(); // Consume BOLD token
        } else {
            return Err(format!("Error: Expected 'BOLD' when {} was found.", self.compiler.current_token()));
        }

        // Accumulate consecutive TEXT tokens for the bold content
        let bold_text = self.accumulate_text();
        if bold_text.is_empty() {
            return Err(format!("Error: Expected TEXT in bold when {} was found.", self.compiler.current_token()));
        }

        if self.lexer.is_a_oic(&self.compiler.current_token()) {
            self.next_token(); // Consume #OIC token
        } else {
            return Err(format!("Error: Expected '#OIC' at end of bold when {} was found.", self.compiler.current_token()));
        }

        // Add bold node to appropriate buffer
        if self.in_body_context {
            self.body_elements.push(ASTNode::Paragraph(vec![ASTNode::Bold(bold_text)]));
        } else {
            self.paragraph_buffer.push(ASTNode::Bold(bold_text));
        }

        Ok(())
    }

    fn parse_italics(&mut self) -> Result<(), String> {
        // Non-terminal for formatting italic text.
        // Assume: #GIMMEH already consumed, current token is ITALICS
        // Form: ITALICS TEXT OIC

        if self.lexer.is_a_italics(&self.compiler.current_token()) {
            self.next_token(); // Consume ITALICS token
        } else {
            return Err(format!("Error: Expected 'ITALICS' when {} was found.", self.compiler.current_token()));
        }

        // Accumulate consecutive TEXT tokens for the italic content
        let italic_text = self.accumulate_text();
        if italic_text.is_empty() {
            return Err(format!("Error: Expected TEXT in italics when {} was found.", self.compiler.current_token()));
        }

        if self.lexer.is_a_oic(&self.compiler.current_token()) {
            self.next_token(); // Consume #OIC token
        } else {
            return Err(format!("Error: Expected '#OIC' at end of italics when {} was found.", self.compiler.current_token()));
        }

        // Add italics node to appropriate buffer
        if self.in_body_context {
            self.body_elements.push(ASTNode::Paragraph(vec![ASTNode::Italics(italic_text)]));
        } else {
            self.paragraph_buffer.push(ASTNode::Italics(italic_text));
        }

        Ok(())
    }

    fn parse_list(&mut self) -> Result<(), String> {
        // Non-terminal for formatting list.
        // Assume: #MAEK already consumed, current token is LIST
        // Form: LIST <list_items> MKAY

        if self.lexer.is_a_list(&self.compiler.current_token()) {
            self.next_token(); // Consume LIST token
        } else {
            return Err(format!("Error: Expected 'LIST' when {} was found.", self.compiler.current_token()));
        }

        // Clear list items buffer for new list
        self.list_items_buffer.clear();

        self.parse_list_items()?; // Parse list items

        if self.lexer.is_a_mkay(&self.compiler.current_token()) {
            self.next_token(); // Consume #MKAY token
        } else {
            return Err(format!("Error: Expected '#MKAY' at end of list when {} was found.", self.compiler.current_token()));
        }

        // Build list node and add to appropriate buffer
        let list = ASTNode::List(self.list_items_buffer.drain(..).collect());
        if self.in_body_context {
            self.body_elements.push(ASTNode::Paragraph(vec![list]));
        } else {
            self.paragraph_buffer.push(list);
        }

        Ok(())
    }

    fn parse_list_items(&mut self) -> Result<(), String> {
        // Non-terminal for list items.
        // Form: <list_item> <list_items> | ε (recursive)

        // If we don't see #GIMMEH, list_items is empty (ε case)
        if !self.lexer.is_a_gimmeh(&self.compiler.current_token()) {
            return Ok(());
        }

        self.parse_list_item()?; // Parse one list item
        self.parse_list_items()?; // Recursively parse more items

        Ok(())
    }

    fn parse_inner_list(&mut self) -> Result<(), String> {
        // Non-terminal for list item content.
        // Form: <bold> | <italics> | <link> | TEXT | <variable_use> | ε
        // Collects content into list_items_buffer for the current list item

        if self.lexer.is_a_text(&self.compiler.current_token()) {
            let text = self.compiler.current_token().clone();
            self.next_token();
            self.list_items_buffer.push(ASTNode::Text(text));
        } else if self.lexer.is_a_gimmeh(&self.compiler.current_token()) {
            self.next_token(); // Consume #GIMMEH
            // Dispatch based on formatting type
            if self.lexer.is_a_bold(&self.compiler.current_token()) {
                self.next_token(); // Consume BOLD
                let text = if self.lexer.is_a_text(&self.compiler.current_token()) {
                    let t = self.compiler.current_token().clone();
                    self.next_token();
                    t
                } else {
                    return Err(format!("Error: Expected TEXT in bold when {} was found.", self.compiler.current_token()));
                };
                if self.lexer.is_a_oic(&self.compiler.current_token()) {
                    self.next_token();
                } else {
                    return Err(format!("Error: Expected '#OIC' at end of bold when {} was found.", self.compiler.current_token()));
                }
                self.list_items_buffer.push(ASTNode::Bold(text));
            } else if self.lexer.is_a_italics(&self.compiler.current_token()) {
                self.next_token(); // Consume ITALICS
                let text = if self.lexer.is_a_text(&self.compiler.current_token()) {
                    let t = self.compiler.current_token().clone();
                    self.next_token();
                    t
                } else {
                    return Err(format!("Error: Expected TEXT in italics when {} was found.", self.compiler.current_token()));
                };
                if self.lexer.is_a_oic(&self.compiler.current_token()) {
                    self.next_token();
                } else {
                    return Err(format!("Error: Expected '#OIC' at end of italics when {} was found.", self.compiler.current_token()));
                }
                self.list_items_buffer.push(ASTNode::Italics(text));
            } else if self.lexer.is_a_linx(&self.compiler.current_token()) {
                self.next_token(); // Consume LINX
                let addr = if self.lexer.is_a_address(&self.compiler.current_token()) {
                    let a = self.compiler.current_token().clone();
                    self.next_token();
                    a
                } else {
                    return Err(format!("Error: Expected ADDRESS for link when {} was found.", self.compiler.current_token()));
                };
                if self.lexer.is_a_oic(&self.compiler.current_token()) {
                    self.next_token();
                } else {
                    return Err(format!("Error: Expected '#OIC' at end of link when {} was found.", self.compiler.current_token()));
                }
                self.list_items_buffer.push(ASTNode::Link { address: addr });
            } else if self.lexer.is_a_newline(&self.compiler.current_token()) {
                self.next_token();
                self.list_items_buffer.push(ASTNode::Newline);
            }
        } else if self.lexer.is_a_lemmesee(&self.compiler.current_token()) {
            self.next_token(); // Consume #LEMMEESEE
            let varname = if self.lexer.is_a_varname(&self.compiler.current_token()) {
                let name = self.compiler.current_token().clone();
                self.next_token();
                name
            } else {
                return Err(format!("Error: Expected variable name after '#LEMMEESEE' when {} was found.", self.compiler.current_token()));
            };
            if self.lexer.is_a_oic(&self.compiler.current_token()) {
                self.next_token();
            } else {
                return Err(format!("Error: Expected '#OIC' at end of variable usage when {} was found.", self.compiler.current_token()));
            }
            if self.symbol_table.lookup(&varname).is_none() {
                return Err(format!("Error: Variable '{}' used before definition.", varname));
            }
            // Add variable value as text
            if let Some((_, value)) = self.symbol_table.lookup(&varname) {
                self.list_items_buffer.push(ASTNode::Text(value));
            }
        } else {
            // ε case: no content in this list item
            return Ok(());
        }

        Ok(())
    }

    fn parse_link(&mut self) -> Result<(), String> {
        // Non-terminal for formatting link/hyperlink.
        // Assume: #GIMMEH already consumed, current token is LINX
        // Form: LINX ADDRESS OIC

        if self.lexer.is_a_linx(&self.compiler.current_token()) {
            self.next_token(); // Consume LINX token
        } else {
            return Err(format!("Error: Expected 'LINX' when {} was found.", self.compiler.current_token()));
        }

        // Capture ADDRESS for AST
        let address = if self.lexer.is_a_address(&self.compiler.current_token()) {
            let addr = self.compiler.current_token().clone();
            self.next_token(); // Consume ADDRESS token
            addr
        } else {
            return Err(format!("Error: Expected ADDRESS for link when {} was found.", self.compiler.current_token()));
        };

        if self.lexer.is_a_oic(&self.compiler.current_token()) {
            self.next_token(); // Consume #OIC token
        } else {
            return Err(format!("Error: Expected '#OIC' at end of link when {} was found.", self.compiler.current_token()));
        }

        // Add link node to appropriate buffer
        if self.in_body_context {
            self.body_elements.push(ASTNode::Paragraph(vec![ASTNode::Link { address }]));
        } else {
            self.paragraph_buffer.push(ASTNode::Link { address });
        }

        Ok(())
    }

    fn parse_newline(&mut self) -> Result<(), String> {
        // Non-terminal for formatting newline.
        // Assume: #GIMMEH already consumed, current token is NEWLINE
        // Form: NEWLINE (no terminator)

        if self.lexer.is_a_newline(&self.compiler.current_token()) {
            self.next_token(); // Consume NEWLINE token
        } else {
            return Err(format!("Error: Expected 'NEWLINE' when {} was found.", self.compiler.current_token()));
        }

        // Add newline node to appropriate buffer
        if self.in_body_context {
            self.body_elements.push(ASTNode::Paragraph(vec![ASTNode::Newline]));
        } else {
            self.paragraph_buffer.push(ASTNode::Newline);
        }

        Ok(())
    }

    fn parse_text(&mut self) -> Result<(), String> {
        // Accumulate consecutive TEXT tokens into a single text node
        let text = self.accumulate_text();
        if text.is_empty() {
            return Err(format!("Error: Expected valid text when {} was found.", self.compiler.current_token()));
        }

        // If at body level, wrap in paragraph; otherwise add to paragraph buffer
        if self.in_body_context {
            self.body_elements.push(ASTNode::Paragraph(vec![ASTNode::Text(text)]));
        } else {
            self.paragraph_buffer.push(ASTNode::Text(text));
        }

        Ok(())
    }
}


// ============================================================================
// LOL to HTML File Translation
// ============================================================================
fn open_in_browser(file: &str) {
    let path = Path::new(file).canonicalize().expect("File not found");

    #[cfg(target_os = "macos")]
    Command::new("open")
        .arg(path)
        .spawn()
        .expect("Failed to open browser");

    #[cfg(target_os = "linux")]
    Command::new("xdg-open")
        .arg(path)
        .spawn()
        .expect("Failed to open browser");

    #[cfg(target_os = "windows")]
    Command::new("cmd")
        .args(["/C", "start", path.to_str().unwrap()])
        .spawn()
        .expect("Failed to open browser");
}


// ============================================================================
// ABSTRACT SYNTAX TREE (AST) DEFINITION
// ============================================================================

/// Represents the elements that can appear in the AST during HTML translation.
/// The AST datatype is designed to hierarchically capture the structure of the lolcode document and its elements,
/// store them into vectors for assembly into the proper syntactic structure, and provide methods
/// for converting LOLCODE markers into their HTML counterparts.
#[derive(Clone, Debug)]
pub enum ASTNode {
    /// Root document node
    Document {
        head: Box<ASTNode>,
        body: Vec<ASTNode>,
    },
    /// Head section with title
    Head {
        title: String,
    },
    /// Comment (translates to HTML comment)
    Comment(String),
    /// Paragraph containing text and formatting
    Paragraph(Vec<ASTNode>),
    /// Plain text content
    Text(String),
    /// Bold formatted text
    Bold(String),
    /// Italic formatted text
    Italics(String),
    /// Hyperlink
    Link {
        address: String,
    },
    /// Newline break
    Newline,
    /// List of items
    List(Vec<ASTNode>),
    /// List item
    ListItem(Vec<ASTNode>),
}

impl ASTNode {
    /// Convert AST to HTML string
    pub fn to_html(&self) -> String {
        match self {
            ASTNode::Document { head, body } => {
                let head_html = head.to_html();
                let body_html = body
                    .iter()
                    .map(|node| node.to_html())
                    .collect::<Vec<_>>()
                    .join("\n");
                format!("<html>\n{}\n<body>\n{}\n</body>\n</html>", head_html, body_html)
            }
            ASTNode::Head { title } => {
                format!("\t<head>\n\t\t<title>{}</title>\n\t</head>", title)
            }
            ASTNode::Comment(text) => {
                format!("\t<!-- {} -->", text)
            }
            ASTNode::Paragraph(nodes) => {
                let content = nodes
                    .iter()
                    .map(|node| node.to_html())
                    .collect::<Vec<_>>()
                    .join("");
                format!("\t<p>{}\n\t</p>", content)
            }
            ASTNode::Text(text) => {
                format!(" {} ", text)
            }
            ASTNode::Bold(text) => {
                format!("<b> {} </b>", text)
            }
            ASTNode::Italics(text) => {
                format!("<i> {} </i>", text)
            }
            ASTNode::Link { address } => {
                format!("<a href=\"{}\">{}</a>", address, address)
            }
            ASTNode::Newline => {
                "\n\t\t<br>".to_string()
            }
            ASTNode::List(items) => {
                let items_html = items
                    .iter()
                    .map(|item| item.to_html())
                    .collect::<Vec<_>>()
                    .join("\n");
                format!("\t\t<ul>\n{}\n\t\t</ul>", items_html)
            }
            ASTNode::ListItem(nodes) => {
                let content = nodes
                    .iter()
                    .map(|node| node.to_html())
                    .collect::<Vec<_>>()
                    .join("");
                format!("\t\t\t<li>{}</li>", content)
            }
        }
    }
}

// ============================================================================
// SEMANTIC ANALYSIS: Variable Symbol Table for Static Scope Resolution
// ============================================================================

/// Tracks variable definitions and validates variable usage across scopes.
/// Supports paragraph-scoped variables: body-level variables are global,
/// paragraph-level variables are local to their paragraph.
pub struct VariableSymbolTable {
    /// Global scope: variables defined at body level
    /// Declares a HashMap that maps String variable names to a tuple with the definition line (which line the variable was declared on) and its String value.
    /// This acts as the symbol table for variables defined at the body table, accessible globally throughout the entire document.
    global_scope: HashMap<String, (usize, String)>, // (definition_line, value)

    /// Scope stack: variables defined at paragraph level
    /// Declares a vector of HashMaps, where each element represents the scope of a paragraph.
    /// When entering a paragraph, a new HashMap is pushed; when exiting, it's popped.
    /// This way, variables defined in a paragraph are only accessible within that paragraph.
    scope_stack: Vec<HashMap<String, (usize, String)>>,
}

impl VariableSymbolTable {
    /// Create a new empty symbol table with an empty global scope and no local scopes, yet.
    pub fn new() -> Self {
        Self {
            global_scope: HashMap::new(),
            scope_stack: Vec::new(),
        }
    }

    /// Define a variable in the current scope
    /// - If in global scope: adds to global_scope
    /// - If in paragraph scope: adds to top of scope_stack
    /// Returns: Ok(()) on success, Err(msg) if semantic error
    pub fn define(&mut self, varname: String, line: usize, value: String) -> Result<(), String> {
        if let Some(local_scope) = self.scope_stack.last_mut() {
            // Inside a paragraph: define in local scope (allow redefinition)
            local_scope.insert(varname, (line, value));
        } else {
            // At body level: define in global scope (allow redefinition)
            self.global_scope.insert(varname, (line, value));
        }
        Ok(())
    }

    /// Look up a variable in the current scope
    /// Searches: local scope first (if in paragraph), then global scope
    /// Returns: Some((definition_line, value)) if found, None if not found
    pub fn lookup(&self, varname: &str) -> Option<(usize, String)> {
        // Check local scopes from innermost to outermost
        for scope in self.scope_stack.iter().rev() {
            if let Some((line, value)) = scope.get(varname) {
                return Some((*line, value.clone()));
            }
        }
        // Check global scope
        if let Some((line, value)) = self.global_scope.get(varname) {
            Some((*line, value.clone()))
        } else {
            None
        }
    }

    /// Enter a new scope (called when starting to parse a paragraph)
    pub fn enter_scope(&mut self) {
        self.scope_stack.push(HashMap::new());
    }

    /// Exit the current scope (called when finishing a paragraph)
    pub fn exit_scope(&mut self) {
        self.scope_stack.pop();
    }
}

// ============================================================================
// MAIN ENTRY POINT
// ============================================================================

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <input_file.lol>", args[0]);
        eprintln!("\nExample: {} document.lol", args[0]);
        std::process::exit(1);
    }

    let input_file = &args[1];

    // Check if file exists and ends with .lol
    if !input_file.ends_with(".lol") {
        eprintln!("Error: Input file must have .lol extension");
        std::process::exit(1);
    }

    // Read the input file
    let source_code = match fs::read_to_string(input_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", input_file, e);
            std::process::exit(1);
        }
    };

    // Create lexer and compiler
    let mut lexer = CharLexicalAnalyzer {
        characters: source_code.chars().collect(),
        position: 0,
        current_token: String::new(),
        tokens: Vec::new(),
        current_line: 1,
    };

    // Tokenize
    lexer.tokenize();

    // Create compiler and parser
    let mut compiler = LolcodeCompiler::new();
    let mut parser = Parser::new(&mut lexer, &mut compiler);

    // Get first token
    parser.next_token();

    // Parse the source code
    match parser.parse_lolcode() {
        Ok(()) => {
            // Get the AST
            if let Some(ast) = parser.get_ast() {
                // Generate HTML
                let html = ast.to_html();

                // Create output filename by replacing .lol with .html
                let output_file = input_file.replace(".lol", ".html");

                // Write HTML to output file
                match fs::write(&output_file, html) {
                    Ok(()) => {
                        println!("✓ Compilation successful!");
                        println!("  Input:  {}", input_file);
                        println!("  Output: {}", output_file);
                        println!("\nOpening in browser...");
                        open_in_browser(&output_file);
                    }
                    Err(e) => {
                        eprintln!("Error writing output file '{}': {}", output_file, e);
                        std::process::exit(1);
                    }
                }
            } else {
                eprintln!("Error: Failed to build AST");
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
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
