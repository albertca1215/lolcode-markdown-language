GitHub Repo:
https://github.com/albertca1215/lolcode-markdown-language

The lolcompiler.exe included in this project was tested on Windows, but should be compatible for MacOS and Linux (for whoever uses it) as well. 

In the Grammar and Lexical Analysis Design stage (Phase 1), ChatGPT was used to aid in the design of the BNF and ANTLR tree by comparing the BNF I'd worked on against the LOLCODE language requirements stipulated in the Project 1 DOCX file (Project_1\design\Cantoria_ COSC 455Project1_Final.docx). In the Implementation stage, the GitHub Copilot extension for VS Code using Claude Haiku was used to assist in the creation of lexical, syntactic, and semantic analysis methods and the implementation of the given Lexical/Syntax Analysis Rust Traits, as well as the creation of the abstract data tree (AST) struct and LOLCODE-to-HTML translation methods. Further details can be found in design\GPT_Prompts.txt and design\ClaudeHaiku_Prompts.