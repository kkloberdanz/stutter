%{
#include <stdlib.h>
%}

%%

[a-z]       {
                yylval.sIndex = *yytext - 'a';
                return VARIABLE;
            }

0           {
                yylval.iValue = atoi(yytext);
                return INTEGER;
            }

[-()<>=+*/;{}.] {
                    return *yytext;
                }

">="        return GE;
"<="        return LE;
"=="        return EQ;
"!="        return NE;
"while"     return WHILE;
"if"        return IF;
"else"      return ELSE;
"print"     return PRINT;

[ \t\n]+    ; /* ignore whitespace */

.           yyerror("Unknown character");

%%
int yywrap(void) {
    return 1;
}
