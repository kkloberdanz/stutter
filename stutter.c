/*
 * Author: Kyle Kloberdanz
 * Date Created: 27 Nov 2018
 * License: GNU GPLv3 (see LICENSE.txt)
 *     This program is free software: you can redistribute it and/or modify
 *     it under the terms of the GNU General Public License as published by
 *     the Free Software Foundation, either version 3 of the License, or
 *     (at your option) any later version.
 *
 *     This program is distributed in the hope that it will be useful,
 *     but WITHOUT ANY WARRANTY; without even the implied warranty of
 *     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *     GNU General Public License for more details.
 *
 *     You should have received a copy of the GNU General Public License
 *     along with this program.  If not, see <https://www.gnu.org/licenses/>.
 * File: stutter.c
 */


#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>


#include "stutter.h"
#include "growstring.h"
#include "linkedlist.h"
#include "ir.h"


char token_string[MAX_TOKEN_SIZE+1];


/* constructors */
StutterObject *make_number_obj(char *n) {
    StutterObject *obj;
    int len_n;
    if ((obj = (StutterObject *)malloc(sizeof(StutterObject))) == NULL) {
        fprintf(stderr, "failed to allocate memory");
        return NULL;
    }
    obj->type = NUMBER_TYPE;

    len_n = strlen(n);
    obj->value.number_value = (char *)calloc(len_n + 1, sizeof(char));
    strcpy(obj->value.number_value, n);
    return obj;
}


StutterObject *make_string_obj(char *str) {
    StutterObject *obj;
    if ((obj = (StutterObject *)malloc(sizeof(StutterObject))) == NULL) {
        fprintf(stderr, "failed to allocate memory");
        return NULL;
    }
    obj->type = STRING_TYPE;
    obj->value.string_value = str;
    return obj;
}


StutterObject *make_id_obj(char *symb) {
    StutterObject *obj;
    if ((obj = (StutterObject *)malloc(sizeof(StutterObject))) == NULL) {
        fprintf(stderr, "failed to allocate memory");
        return NULL;
    }
    obj->type = VOID_TYPE;
    obj->value.symbol = symb;
    return obj;
}


char *make_string(char *str) {
    char *s = (char *)malloc(strlen(str) + 1);
    if (s == NULL) {
        fprintf(stderr, "%s\n", "out of memory");
        return NULL;
    }
    strcpy(s, str);
    return s;
}


ASTNode *make_ast_node(const ASTkind kind,
                       StutterObject *obj,
                       const Operator op,
                       ASTNode *left_node,
                       ASTNode *condition,
                       ASTNode *right_node) {

    ASTNode *node;
    if ((node = (ASTNode *)malloc(sizeof(ASTNode))) == NULL) {
        fprintf(stderr, "failed to allocate memory");
        return NULL;
    }

    node->kind = kind;

    switch (kind) {
        case LEAF:
            node->obj = obj;
            node->op = op;
            node->left = NULL;
            node->condition = NULL;
            node->right = NULL;
            break;

        case CONDITIONAL:
            node->obj = NULL;
            node->op = NOP;
            node->left = left_node; /* the true path */
            node->condition = condition; /* the expr to evaluate */
            node->right = right_node; /* the false path */
            break;

        case OPERATOR:
            node->obj = NULL;
            node->op = op;
            node->left = left_node;
            node->condition = NULL;
            node->right = right_node;
            break;

        default:
            fprintf(stderr, "error: invalid ASTkind %d\n", kind);
            destroy_ast_node(node);
            node = NULL;
    }

    return node;
}


ASTNode *make_leaf_node(StutterObject *obj) {
    ASTNode *node = make_ast_node(LEAF, obj, NOP, NULL, NULL, NULL);
    return node;
}


ASTNode *make_operator_node(Operator op, ASTNode *left, ASTNode *right) {
    ASTNode *node = make_ast_node(OPERATOR, NULL, op, left, NULL, right);
    return node;
}


/* destructors */
void destroy_obj(StutterObject *obj) {
    free(obj);
}


void destroy_ast_node(ASTNode *node) {
    if (node) {
        if (node->obj) {
            destroy_obj(node->obj);
            node->obj = NULL;
        }

        /* recursive call */
        if (node->condition) {
            destroy_ast_node(node->condition);
            node->condition = NULL;
        }

        /* recursive call */
        if (node->left) {
            destroy_ast_node(node->left);
            node->left = NULL;
        }

        /* recursive call */
        if (node->right) {
            destroy_ast_node(node->right);
            node->right = NULL;
        }

        free(node);
        node = NULL;
    }
}


Ir *get_op_ir(const Operator op) {
    Ir *ir = (Ir *)malloc(sizeof(Ir));
    if (ir == NULL) {
        fprintf(stderr, "%s\n", "failed to allocate Ir object");
        exit(EXIT_FAILURE);
    }
    ir->kind = IR_OP;
    switch (op) {
        case ADD:
            ir->repr = "ADD";
            ir->value.op = IR_ADD;
            break;

        case SUB:
            ir->repr = "SUB";
            ir->value.op = IR_SUB;
            break;

        case MUL:
            ir->repr = "MUL";
            ir->value.op = IR_MUL;
            break;

        case DIV:
            ir->repr = "DIV";
            ir->value.op = IR_DIV;
            break;

        case NOP:
            ir->repr = "NOP";
            ir->value.op = IR_NOP;
            break;
    }
    return ir;
}


char *get_op_val(char *str, const StutterObject *obj) {
    switch (obj->type) {
        case NUMBER_TYPE:
            sprintf(str, "%s", obj->value.number_value);
            break;

        default:
            fprintf(stderr, "unhandled case: %d\n", obj->type);
            return NULL;
    }
    return str;
}


static Ir *get_ir_node(const ASTNode *ast) {
    Ir *ir_node = NULL;
    switch (ast->kind) {
        case LEAF:
        {
            char *value = ast->obj->value.number_value;
            ir_node = (Ir *)malloc(sizeof(Ir));
            ir_node->kind = IR_NUMBER;
            ir_node->value.number = value;
            ir_node->repr = value;
            break;
        }

        default:
            fprintf(stderr, "incorrect ast kind: %d\n", ast->kind);
            exit(EXIT_FAILURE);
    }
    if (ir_node == NULL) {
        fprintf(stderr, "%s\n", "failed to initialize ir_node");
        exit(EXIT_FAILURE);
    }
    return ir_node;
}


/* code generation */
static linkedlist *codegen_stack_machine(const ASTNode *ast) {
    linkedlist *program = NULL;
    switch (ast->kind) {
        case CONDITIONAL:
            fprintf(stderr, "CONDITIONAL not implemented");
            exit(EXIT_FAILURE);
            break;

        case OPERATOR:
            program = codegen_stack_machine(ast->right);
            ll_concat(program, codegen_stack_machine(ast->left));
            ll_append(program, get_op_ir(ast->op));
            break;

        case LEAF:
        {
            Ir *ir = (Ir *)malloc(sizeof(Ir));
            ir->repr = "PUSH";
            ir->kind = IR_OP;
            ir->value.op = IR_PUSH;
            program = ll_new(ir);
            ll_append(program, get_ir_node(ast));
            break;
        }

        default:
            fprintf(stderr, "unknown ASTNode kind in emit: %d\n", ast->kind);
            exit(EXIT_FAILURE);
    }
    return program;
}


int emit(FILE *output, const ASTNode *ast) {
    linkedlist *program = codegen_stack_machine(ast);
    program = ir_halt_program(program);
    ir_print_program(output, program);
    ll_free(program);
    return 0;
}
