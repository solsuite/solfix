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
    PERIOD = 268,
    /* Pragma Directive Lexemes */
    PRAGMA = 269,
    VERSION = 270,    
    /* Import Directive Lexemes */
    IMPORT = 271,
    STRING_LITERAL = 272,
    AS = 273,
    FROM = 274,
    COMMA = 275,
    /* Contract Definition Lexemes */
    CONTRACT = 276,
    LIBRARY = 277,
    INTERFACE = 278,
    IS = 279,
    /* State Variable Declaration Lexemes */
    PUBLIC = 280,
    INTERNAL = 281,
    PRIVATE = 282,
    CONSTANT = 283,
    /* Using For Declaration Lexemes */
    USING = 284,
    FOR = 285,
    /* Struct Definition Lexemes */
    STRUCT = 286,
    /* Modifier Definition Lexemes */ 
    MODIFIER = 287,
    /* Function Definition Lexemes */
    FUNCTION = 288,
    EXTERNAL = 289,
    RETURNS = 290,
    /* Event Definition Lexemes */
    EVENT = 291,
    ANONYMOUS = 292,
    /* Enum Definition Lexemes */
    ENUM = 293,
    /* Event Parameter Lexemes */
    INDEXED = 294,
    /* Mapping Lexemes */
    MAPPING = 295,
    ARROW = 296,
    /* Storage Location Lexemes */
    MEMORY = 297,
    STORAGE = 298,
    /* State Mutability Lexemes */ 
    PURE = 299,
    VIEW = 300,
    PAYABLE = 301,
    /* If Statement Lexemes */
    IF = 302,
    ELSE = 303,
    /* While Statement Lexemes */
    WHILE = 304,
    /* Placeholder Statement Lexemes */
    UNDERSCORE = 305,
    /* Inline Assembly Statement Lexemes */
    ASSEMBLY = 306,
    /* Do While Statement Lexemes */
    DO = 307, 
    /* Continue Lexemes */
    CONTINUE = 308,
    /* Break Lexemes */
    BREAK = 309,
    /* Return Lexemes */
    RETURN = 310,
    /* Throw Lexemes */
    THROW = 311,
    /* Emit Lexemes */
    EMIT = 312,
    /* Variable Declaration Lexemes */
    VAR = 313,
    /* Expression Lexemes */
    INCREMENT = 314,
    DECREMENT = 315,
    NOT = 316,
    COMPLEMENT = 317,
  };
%}
%%
assembly                      { cout << ASSEMBLY << endl; }
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
using                         { cout << USING << endl; }
for                           { cout << FOR << endl; }
function                      { cout << FUNCTION << endl; }
external                      { cout << EXTERNAL << endl; }
returns                       { cout << RETURNS << endl; }
event                         { cout << EVENT << endl; }
anonymous                     { cout << ANONYMOUS << endl; }
enum                          { cout << ENUM << endl; }
indexed                       { cout << INDEXED << endl; }
mapping                       { cout << MAPPING << endl; }
pure                          { cout << PURE << endl; }
view                          { cout << VIEW << endl; }
payable                       { cout << PAYABLE << endl; }
if                            { cout << IF << endl; }
else                          { cout << ELSE << endl; }
while                         { cout << WHILE << endl; }
do                            { cout << DO << endl; }
continue                      { cout << CONTINUE << endl; }
break                         { cout << BREAK << endl; }
return                        { cout << RETURN << endl; }
throw                         { cout << THROW << endl; }
emit                          { cout << EMIT << endl; }
var                           { cout << VAR << endl; }
\+\+                          { cout << INCREMENT << endl; }
--                            { cout << DECREMENT << endl; }
!                             { cout << NOT << endl; }
~                             { cout << COMPLEMENT << endl; }
=>                            { cout << ARROW << endl; }
\.                            { cout << PERIOD << endl; }
,                             { cout << COMMA << endl; }
;                             { cout << SEMICOLON << endl; }
\*                            { cout << ASTERISK << endl; }
_                             { cout << UNDERSCORE << endl; }
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
