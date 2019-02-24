/*
 * Author: Kyle Kloberdanz
 * Date Created: 27 Nov 2018
 * License: GNU GPLv3 (see LICENSE.txt)
 * File: stutter.c
 */


#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>


#include "stutter.h"


/* constructors */
StutterObject *make_number_obj(number n) {
    StutterObject *obj;
    if ((obj = (StutterObject *)malloc(sizeof(StutterObject))) == NULL) {
        fprintf(stderr, "failed to allocate memory");
        exit(EXIT_FAILURE);
    }
    obj->type = NUMBER_TYPE;
    obj->value.number_value = n;
    return obj;
}


ASTNode *make_ast_node(ASTkind kind,
                       StutterObject *obj,
                       Operator op,
                       ASTNode *left_node,
                       ASTNode *condition,
                       ASTNode *right_node) {

    ASTNode *node;
    if ((node = (ASTNode *)malloc(sizeof(ASTNode))) == NULL) {
        fprintf(stderr, "failed to allocate memory");
        exit(EXIT_FAILURE);
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
            node->op = NOOP;
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
            exit(EXIT_FAILURE);
    }

    return node;
}


ASTNode *make_leaf_node(StutterObject *obj) {
    ASTNode *node = make_ast_node(LEAF, obj, NOOP, NULL, NULL, NULL);
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

        if (node->condition) {
            free(node->condition);
            node->condition = NULL;
        }

        /* recursive call */
        if (node->left) {
            destroy_ast_node(node->left);
            free(node->left);
            node->left = NULL;
        }

        /* recursive call */
        if (node->right) {
            destroy_ast_node(node->right);
            free(node->right);
            node->right = NULL;
        }
    }
}


char *get_op_str(Operator op) {
    char *str = NULL;
    switch (op) {
        case ADD:
            str = "ADD";
            break;

        case SUB:
            str = "SUB";
            break;

        case MUL:
            str = "MUL";
            break;

        case DIV:
            str = "DIV";
            break;

        case NOOP:
            str = "NOOP";
            break;
    }
    return str;
}


char *get_op_val(StutterObject *obj) {
    char *str = (char*)malloc(100);
    switch (obj->type) {
        case NUMBER_TYPE:
            sprintf(str, "%ld", obj->value.number_value);
            break;

        default:
            fprintf(stderr, "unhandled case: %d\n", obj->type);
            exit(EXIT_FAILURE);
    }
    return str;
}


/* code generation */
void emit(FILE *output, ASTNode *node) {
    switch (node->kind) {
        case CONDITIONAL:
            fprintf(stderr, "CONDITIONAL not implemented");
            exit(EXIT_FAILURE);
            break;

        case OPERATOR:
            emit(output, node->left);
            emit(output, node->right);
            fprintf(output, "%s\n", get_op_str(node->op));
            break;

        case LEAF:
            fprintf(output, "PUSH\n%s\n", get_op_val(node->obj));
            break;

        default:
            fprintf(stderr, "unknown ASTNode kind in emit: %d\n", node->kind);
            exit(EXIT_FAILURE);
    }
}


int main(int argc, char **argv) {
    if (argc != 2) {
        fprintf(stderr, "usage: %s FILENAME\n", argv[0]);
        return 1;
    } else {
        ASTNode *tree = parse();
        char *output_filename = argv[1];
        FILE *output = fopen(output_filename, "w");
        emit(output, tree);
        return 0;
    }
}
