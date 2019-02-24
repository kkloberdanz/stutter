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
lines       : sexpr                      { puts("lines"); tree = $1 ; }
            ;

sexpr       : '(' expr ')'               { puts("sexpr"); $$ = $2 ; }
            ;

expr        : '+' expr expr              { puts("expr"); $$ = make_operator_node(ADD, $2, $3) ; }
            | '-' expr expr              { puts("expr"); $$ = make_operator_node(SUB, $2, $3) ; }
            | '*' expr expr              { puts("expr"); $$ = make_operator_node(MUL, $2, $3) ; }
            | '/' expr expr              { puts("expr"); $$ = make_operator_node(DIV, $2, $3) ; }
            | sexpr
            | NUMBER                     { puts("expr"); $$ = make_leaf_node(make_number_obj($1)) ; }
            ;

%%


ASTNode *parse(void) {
    yyparse();
    puts("returning tree");
    return tree;
}


int yylex() {
    int c;
    puts("lexing");
    while ((c = getchar()) == ' ');
    puts("done");
    printf("c = %c\n", c);
    if ((c == '.') || (isdigit(c))) {
        ungetc(c, stdin);
        scanf("%d", &yylval);
        return NUMBER;
    }
    if (c == '\n') {
        return 0;
    }
    return c;
}
