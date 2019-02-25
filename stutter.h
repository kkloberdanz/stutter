#ifndef STUTTER_H
#define STUTTER_H


#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>


/* embedded strings */
static volatile char author[] = "Author: Kyle Kloberdanz";
static volatile char license[] = "License: GNU GPLv3";


/* typdefs */
typedef int64_t number;
typedef double real;


/* enums */
typedef enum {
    NUMBER_TYPE,
    REAL_TYPE,
    BOOL_TYPE
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


/* parser */
ASTNode *parse(void);


/* code generation */
char *get_op_str(const Operator op);
char *get_op_val(char *str, const StutterObject *obj);
int emit(FILE *, const ASTNode *);


#endif /* STUTTER_H */
