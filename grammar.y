%{
#include <stdio.h>
#include <stdlib.h>
#include <stdarg.h>
#include <ctype.h>
#include <stdbool.h>


#include "stutter.h"


#define YYSTYPE ASTNode *


int yylex();
void yyerror(const char *s);
static ASTNode *tree = NULL;

%}

%token NUMBER

%%
lines       : sexpr                      { tree = $1 ; }
            ;

sexpr       : '(' expr ')'               { $$ = $2 ; }
            ;

expr        : '+' expr expr              { $$ = make_operator_node(ADD, $2, $3) ; }
            | '-' expr expr              { $$ = make_operator_node(SUB, $2, $3) ; }
            | '*' expr expr              { $$ = make_operator_node(MUL, $2, $3) ; }
            | '/' expr expr              { $$ = make_operator_node(DIV, $2, $3) ; }
            | sexpr
            | NUMBER                     { $$ = make_leaf_node(make_number_obj((number)$1)) ; }
            ;

%%


ASTNode *parse(void) {
    yyparse();
    return tree;
}


int yylex() {
    int c;
    while ((c = getchar()) == ' ');
    if ((c == '.') || (isdigit(c))) {
        ungetc(c, stdin);
        if (!scanf("%ld", (number *)&yylval)) {
            fprintf(stderr, "%s\n", "failed to read from stdin");
        }
        return NUMBER;
    }
    if (c == '\n') {
        return 0;
    }
    return c;
}
