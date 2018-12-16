/*
 * Author: Kyle Kloberdanz
 * Date Created: 27 Nov 2018
 * License: GNU GPLv3 (see LICENSE.txt)
 * File: studder.c
 */


#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>


#include "stutter.h"


/* constructors */
StutterObject *make_number_obj(number n) {
    StutterObject *obj;
    if ((obj = (StutterObject *)malloc(sizeof(StutterObject))) == NULL) {
        fprintf(stderr, "failed to allocate memory");
        exit(EXIT_FAILURE);
    }
    obj->type = NUMBER;
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
            node->right = NULL;
            break;

        case CONDITIONAL:
            node->obj = NULL;
            node->op = NOOP;
            node->left = left_node; /* the true path */
            node->condition = condition; /* the expr to evaluate */
            node->right = right_node; /* the false path */
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


int main(void) {
    return 0;
}
