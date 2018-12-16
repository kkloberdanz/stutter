%{
#include <stdio.h>
#include <stdlib.h>
#include <stdarg.h>
#include <ctype.h>


#include "stutter.h"

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

expr        : '+' expr expr              { $$ = make_operator_node(ADD, $2, $3) ; }
            | '-' expr expr              { $$ = make_operator_node(SUB, $2, $3) ; }
            | '*' expr expr              { $$ = make_operator_node(MUL, $2, $3) ; }
            | '/' expr expr              { $$ = make_operator_node(DIV, $2, $3) ; }
            | sexpr
            | NUMBER                     { $$ = make_leaf_node(make_number_obj($1)) ; }
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
