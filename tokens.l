/*
 * Author: Kyle Kloberdanz
 * Date Created: 27 Nov 2018
 * License: GNU GPLv3 (see LICENSE.txt)
 *     This program is free software: you can redistribute it and/or modify
 *     it under the terms of the GNU General Public License as published by
 *     the Free Software Foundation, either version 3 of the License, or
 *     (at your option) any later version.
 *
 *     This program is distributed in the hope that it will be useful,
 *     but WITHOUT ANY WARRANTY; without even the implied warranty of
 *     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *     GNU General Public License for more details.
 *
 *     You should have received a copy of the GNU General Public License
 *     along with this program.  If not, see <https://www.gnu.org/licenses/>.
 * File: tokens.l
 */


%{
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include "stutter.h"

#include "y.tab.h"

char token_string[MAX_TOKEN_SIZE+1];
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
    strncpy(token_string, yytext, MAX_TOKEN_SIZE);
    return currentToken;
}
