#ifndef STUTTER_H
#define STUTTER_H


#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>


/* globals */
extern char token_string[101];


/* embedded strings */
static volatile char author[] = "Author: Kyle Kloberdanz";
static volatile char license[] = "License: GNU GPLv3";


/* typdefs */
typedef int64_t number;
typedef double real;


/* enums */
typedef enum {
    VOID_TYPE,  /* voids must be resolved during type deduction */
    NUMBER_TYPE,
    REAL_TYPE,
    BOOL_TYPE,
    STRING_TYPE
} StutterType;


typedef enum {
    CONDITIONAL,
    OPERATOR,
    LEAF
} ASTkind;


typedef enum {
    NOP,
    ADD,
    SUB,
    MUL,
    DIV
} Operator;


/* structs */
typedef struct StutterObject {
    StutterType type;
    union {
        number number_value;
        real real_value;
        bool bool_value;
        const char *string_value;
        const char *symbol;
    } value;
} StutterObject;


typedef struct ASTNode {
    ASTkind kind;
    StutterObject *obj;
    Operator op;
    struct ASTNode *left;
    struct ASTNode *condition;
    struct ASTNode *right;
} ASTNode;


/* constructors */
StutterObject *make_number_obj(const number);
StutterObject *make_string_obj(const char *str);
StutterObject *make_id_obj(const char *str);
char *make_string(const char *str);

ASTNode *make_ast_node(const ASTkind, /* base constructor */
                       StutterObject *,
                       const Operator,
                       ASTNode *,
                       ASTNode *,
                       ASTNode *);

ASTNode *make_leaf_node(StutterObject *); /* just holds stutter object */

ASTNode *make_operator_node(Operator,  /* holds operator and child items */
                            ASTNode *, /* to operate on */
                            ASTNode *);
                                           


/* destructors */
void destroy_obj(StutterObject *);
void destroy_ast_node(ASTNode *);


/* lexer */
int get_token(void);


/* parser */
ASTNode *parse(void);


/* code generation */
char *get_op_str(const Operator op);
char *get_op_val(char *str, const StutterObject *obj);
int emit(FILE *, const ASTNode *);


#endif /* STUTTER_H */
