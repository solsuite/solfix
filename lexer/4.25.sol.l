%{
  #include <iostream>
  using namespace std;
  extern int yylex();
  enum Tokens {
    SEMICOLON = 258,
    PRAGMA = 259,
    SOLIDITY = 260,
    VERSION = 261,
    DECIMAL = 262,
    HEX = 263,
    IDENTIFIER = 264,
    IMPORT = 265,
    CONTRACT = 266,
    LIBRARY = 267,
    INTERFACE = 268,
    IS = 269,
    AS = 270,
    FROM = 271,
    COMMA = 272,
    LEFT_PAREN = 273,
    RIGHT_PAREN = 274,
    LEFT_BRACE = 275,
    RIGHT_BRACE = 276,
    LEFT_BRACKET = 277,
    RIGHT_BRACKET = 278,
    PUBLIC = 279,
    PRIVATE = 280,
    EXTERNAL = 281,
    INTERNAL = 282,
  };
%}
%%
pragma                                       { cout << PRAGMA << endl; }
solidity                                     { cout << SOLIDITY << endl; }
import                                       { cout << IMPORT << endl; } 
contract                                     { cout << CONTRACT << endl; }
library                                      { cout << LIBRARY << endl; }
interface                                    { cout << INTERFACE << endl; }
is                                           { cout << IS << endl; }
public                                       { cout << PUBLIC << endl; }
private                                      { cout << PRIVATE << endl; }
external                                     { cout << EXTERNAL << endl; }
internal                                     { cout << INTERNAL << endl; }
\(                                           { cout << LEFT_PAREN << endl; }
\)                                           { cout << RIGHT_PAREN << endl; }
\{                                           { cout << LEFT_BRACE << endl; }
\}                                           { cout << RIGHT_BRACE << endl; }
\[                                           { cout << LEFT_BRACKET << endl; }
\]                                           { cout << RIGHT_BRACKET << endl; }
\,                                           { cout << COMMA << endl; }
\^[0-9]+\.[0-9]+\.[0-9]+                     { cout << VERSION << ": " << yytext << endl; }
;                                            { cout << SEMICOLON << endl; }
[a-zA-Z_$][a-zA-Z_$0-9]*                     { cout << IDENTIFIER << ": " << yytext << endl; }
[0-9]+(\.[0-9]*)?([eE][0-9]+)?               { cout << DECIMAL << ": " << yytext << endl; }
0x[0-9a-fA-F]+                               { cout << HEX << ": " << yytext << endl; }
[ \t\n]                                      { /* Do nothing */ }
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
