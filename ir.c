#include <stdlib.h>
#include <stdio.h>


#include "ir.h"


Ir *ir_new(char *ir_str, ir_kind kind) {
    Ir *ir_node = (Ir *)malloc(sizeof(Ir));
    if (ir_node == NULL) {
        fprintf(stderr, "%s\n", "error: out of memory");
    }
    ir_node->kind = kind;
    ir_node->repr = ir_str;
    return ir_node;
}
