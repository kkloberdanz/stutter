#ifndef IR_H
#define IR_H


#include <stdio.h>
#include "linkedlist.h"


typedef enum ir_kind {
    IR_OP,
    IR_NUMBER
} ir_kind;


typedef enum ir_op {
    IR_NOP,
    IR_HALT,
    IR_ADD,
    IR_SUB,
    IR_MUL,
    IR_DIV,
    IR_PUSH
} ir_op;


typedef struct Ir {
    ir_kind kind;
    char *repr;
    union {
        ir_op op;
        char *number;
    } value;
} Ir;


void ir_print_program(FILE *output, const linkedlist *program);
linkedlist *ir_halt_program(linkedlist* program);


#endif /* IR_H */
