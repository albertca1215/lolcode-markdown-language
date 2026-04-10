grammar lolcode;

LETTERS 	: ("A'..'Z' | 'a'..'z' ); 
DIGITS 		: '0'..'9'; 
CHARS 		: ' ' | ',' | '.' | '\"' | '\'' |':' | '?' | '!' | '%' | '/';
BREAK		: '\n';
TAB		: '\t';

NEWLINE 	: '#NEWLINE'; 
OBTW 		: '#OBTW'; 
TLDR 		: '#TLDR'; 
MKAY 		: '#MKAY'; 
OIC 		: '#OIC';

HAI 		: '#HAI';
KBYE 		: '#KBYE'; 
GIMMEH_TITLE 	: '#GIMMEH TITLE'; 
GIMMEH_BOLD 	: '#GIMMEH BOLD'; 
MAEK_HEAD 	: '#MAEK HEAD'; 
GIMMEH_ITALICS 	: '#GIMMEH ITALICS'; 
MAEK_PARAGRAF 	: '#MAEK PARAGRAF';
MAEK_LIST 	: '#MAEK LIST'; 
GIMMEH_ITEM 	: '#GIMMEH ITEM'; 
GIMMEH_LINX 	: '#GIMMEH LINX';
IHAZ		: '#IHAZ';
ITZ		: '#ITZ';
LEMME_SEE	: '#LEMME SEE';

TEXT 		: (LETTERS | DIGITS | CHARS)+;

bold 		: GIMMEH_BOLD (TEXT | BREAK)+ OIC; 
italics 	: GIMMEH_ITALICS (TEXT | BREAK)+ OIC; 
comment 	: OBTW (TEXT | BREAK | TAB)* TLDR;

inline		: bold | italics | TEXT;
inline_seq	: inline+;

title		: GIMMEH_TITLE (BREAK | TAB)*(bold | italics | TEXT)+ (BREAK | TAB)* OIC; 
pre_head	: (comment (BREAK |TAB)*)*; 
header		: MAEK_HEAD (BREAK | TAB | TEXT)* header_body;
header_body	: title (BREAK | TAB | TEXT)* MKAY | MKAY;
paragraph 	: MAEK_PARAGRAF inline_seq MKAY; 
list 		: MAEK_LIST list_items MKAY; 
list_items 	: get_item+; 
get_item	: GIMMEH_ITEM inline_seq OIC; 
link 		: GIMMEH_LINX TEXT OIC; var_name: (LETTERS | DIGITS)+; 
var_declare 	: IHAZ var_name ITZ TEXT MKAY; 
var_getter 	: LEMME_SEE var_name OIC;


block		: paragraph | list | link | var_declare | var_getter;
body_element	: comment | block | NEWLINE | BREAK | TAB;
body		: body_element*;

lolcode		: HAI (BREAK | TAB)* pre_head (BREAK | TAB)* header? 
		(BREAK | TAB)* body (BREAK | TAB)* KBYE;
