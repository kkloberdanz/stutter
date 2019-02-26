%{
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include "stutter.h"

#include "y.tab.h"

char token_string[101];
%}

digit       [0-9]
number      {digit}+
letter      [a-zA-Z]
identifier  {letter}+
newline     \n
whitespace  [ \t]+

%%

":="          { return ASSIGN; }
"="           { return EQ; }
">="          { return GE; }
"<="          { return LE; }
"<>"          { return NE; }
"if"          { return IF; }
"then"        { return THEN; }
"else"        { return ELSE; }
"print"       { return PRINT; }
"("           { return LPAREN; }
")"           { return RPAREN; }
"+"           { return PLUS; }
"-"           { return MINUS; }
"*"           { return TIMES; }
"/"           { return OVER; }
{number}      { return NUMBER; }
{identifier}  { return ID; }
{newline}     { yylineno++; }
{whitespace}  { /* do nothing */; }
.             { return ERROR; }

%%

int get_token(void) {
    static int first_time = true;
    int currentToken;
    if (first_time) {
        first_time = false;
        yylineno++;
        yyin = stdin;
        yyout = stdout;
    }
    currentToken = yylex();
    strncpy(token_string, yytext, 100);
    return currentToken;
}
