%{
#define YYPARSER


#include <stdio.h>
#include <stdlib.h>
#include <stdarg.h>
#include <ctype.h>
#include <stdbool.h>


#include "stutter.h"


#define YYSTYPE ASTNode *


static int yylex();
void yyerror(const char *s);
static ASTNode *tree = NULL;

%}

%token ENDFILE
%token ERROR
%token IF
%token THEN
%token ELSE
%token PRINT
%token ID
%token NUMBER
%token ASSIGN
%token EQ
%token NE
%token LT
%token GE
%token LE
%token PLUS
%token MINUS
%token TIMES
%token OVER
%token EXPONENT
%token LPAREN
%token RPAREN
%token SEMI


%left MINUS PLUS
%left TIMES OVER
%right EXPONENT        /* exponentiation */

%%
prog        : expr                  { tree = $1 ; }
            ;

expr        : expr PLUS expr        { $$ = make_operator_node(ADD, $1, $3) ; }
            | expr MINUS expr       { $$ = make_operator_node(SUB, $1, $3) ; }
            | expr TIMES expr       { $$ = make_operator_node(MUL, $1, $3) ; }
            | expr OVER expr        { $$ = make_operator_node(DIV, $1, $3) ; }
            | LPAREN expr RPAREN    { $$ = $2 ; }
            | NUMBER                { $$ = make_leaf_node(make_number_obj(atoi(token_string))) ; }
            | ID                    { $$ = make_leaf_node(make_id_obj(make_string(token_string))) ; }
            ;

%%


ASTNode *parse(void) {
    yyparse();
    return tree;
}


static int yylex(void) {
    int token = get_token();
    printf("token = %d\n", token);
    return token;
}

/*
int yylex() {
    int c;
    while ((c = getchar()) == ' ');
    if (isdigit(c)) {
        ungetc(c, stdin);
        if (!scanf("%ld", (number *)&yylval)) {
            fprintf(stderr, "%s\n", "failed to read from stdin");
        }
        return NUMBER;
    } else if (c == '\n') {
        return 0;
    } else if ((c == '+') ||
               (c == '-') ||
               (c == '*') ||
               (c == '/')) {
        return c;
    } else {
        ungetc(c, stdin);
        if (!scanf("%s", (char *)&yylval)) {
            fprintf(stderr, "%s\n", "failed to read from stdin");
        }
        return ID;
    }
}
*/
