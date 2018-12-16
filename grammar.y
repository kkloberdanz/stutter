%{
#include <stdio.h>
#include <stdlib.h>
#include <stdarg.h>
#include <ctype.h>

int yylex();
void yyerror(const char *s);
%}

%token NUMBER

%%
lines       : lines sexpr '\n'           { printf("%d\n", $2) ; }
            | lines '\n'
            | /* empty */
            ;

sexpr       : '(' expr ')'               { $$ = $2 ; }
            ;

expr        : '+' expr expr              { $$ = $2 + $3 ; }
            | '-' expr expr              { $$ = $2 - $3 ; }
            | '*' expr expr              { $$ = $2 * $3 ; }
            | '/' expr expr              { $$ = $2 / $3 ; }
            | sexpr
            | NUMBER
            ;

%%


int yylex() {
    int c;
    while ((c = getchar()) == ' ');
    printf("c = %c\n", c);
    if ((c == '.') || (isdigit(c))) {
        ungetc(c, stdin);
        scanf("%d", &yylval);
        return NUMBER;
    }
    return c;
}
