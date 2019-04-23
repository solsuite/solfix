%{
  #include <iostream>
  using namespace std;
  extern int yylex();
  enum Tokens {
    /* Miscallaneous */ 
    SEMICOLON = 258,
    IDENTIFIER = 259,
    RIGHT_BRACE = 260,
    LEFT_BRACE = 261,
    RIGHT_BRACKET = 262,
    LEFT_BRACKET = 263,
    RIGHT_PAREN = 264,
    LEFT_PAREN = 265,
    EQUAL = 266,
    ASTERISK = 267,
    /* Pragma Directive Lexemes */
    PRAGMA = 268,
    VERSION = 269,    
    /* Import Directive Lexemes */
    IMPORT = 270,
    /* TODO: Verify that this works correctly */
    STRING_LITERAL = 271,
    AS = 272,
    FROM = 273,
    COMMA = 274,
    /* Contract Definition Lexemes */
    CONTRACT = 275,
    LIBRARY = 276,
    INTERFACE = 277,
    IS = 278,
    /* State Variable Declaration Lexemes */
    PUBLIC = 279,
    INTERNAL = 280,
    PRIVATE = 281,
    CONSTANT = 282,
  };
%}
%%
pragma                        { cout << PRAGMA << endl; }
import                        { cout << IMPORT << endl; }
as                            { cout << AS << endl; }
from                          { cout << FROM << endl; }
contract                      { cout << CONTRACT << endl; }
library                       { cout << LIBRARY << endl; }
interface                     { cout << INTERFACE << endl; }
is                            { cout << IS << endl; }
public                        { cout << PUBLIC << endl; }
internal                      { cout << INTERNAL << endl; }
private                       { cout << PRIVATE << endl; }
constant                      { cout << CONSTANT << endl; }
,                             { cout << COMMA << endl; }
;                             { cout << SEMICOLON << endl; }
\*                            { cout << ASTERISK << endl; }
\"([^\"\r\n\\]|'\\'.)*\"      { cout << STRING_LITERAL << ": " << yytext << endl; }
[a-zA-Z_$][a-zA-Z_$0-9]*      { cout << IDENTIFIER << ": " << yytext << endl; }
=                             { cout << EQUAL << endl; }
\{                            { cout << RIGHT_BRACE << endl; } 
\}                            { cout << LEFT_BRACE << endl; }
\[                            { cout << RIGHT_BRACKET << endl; }
\]                            { cout << LEFT_BRACKET << endl; }
\(                            { cout << RIGHT_PAREN << endl; }
\)                            { cout << LEFT_PAREN << endl; }
[ \t]                         { /* Do nothing */ }
\n                            { /* Do nothing */ }
[^;a-zA-Z \t\n]+              { cout << VERSION << ": " << yytext << endl; } 
%%
int main(int argc, char** argv) {
  argc--, argv++;
  if (argc != 1) {
    cout << "One argument was expected but " << argc << " were provided" << endl;
    return 1;
  }
  FILE *input = fopen(*argv, "r");
  yyin = input;
  while(yylex());
}
