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


char token_string[101];


/* constructors */
StutterObject *make_number_obj(const number n) {
    StutterObject *obj;
    if ((obj = (StutterObject *)malloc(sizeof(StutterObject))) == NULL) {
        fprintf(stderr, "failed to allocate memory");
        return NULL;
    }
    obj->type = NUMBER_TYPE;
    obj->value.number_value = n;
    return obj;
}


StutterObject *make_string_obj(const char *str) {
    StutterObject *obj;
    if ((obj = (StutterObject *)malloc(sizeof(StutterObject))) == NULL) {
        fprintf(stderr, "failed to allocate memory");
        return NULL;
    }
    obj->type = STRING_TYPE;
    obj->value.string_value = str;
    return obj;
}


StutterObject *make_id_obj(const char *symb) {
    StutterObject *obj;
    if ((obj = (StutterObject *)malloc(sizeof(StutterObject))) == NULL) {
        fprintf(stderr, "failed to allocate memory");
        return NULL;
    }
    obj->type = VOID_TYPE;
    obj->value.symbol = symb;
    return obj;
}


char *make_string(const char *str) {
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


/* emitter helpers */
char *get_op_str(const Operator op) {
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

        case NOP:
            str = "NOP";
            break;
    }
    return str;
}


char *get_op_val(char *str, const StutterObject *obj) {
    switch (obj->type) {
        case NUMBER_TYPE:
            sprintf(str, "%ld", obj->value.number_value);
            break;

        default:
            fprintf(stderr, "unhandled case: %d\n", obj->type);
            return NULL;
    }
    return str;
}


/* code generation */
int emit(FILE *output, const ASTNode *node) {
    int exit_code = 0;
    switch (node->kind) {
        case CONDITIONAL:
            fprintf(stderr, "CONDITIONAL not implemented");
            exit_code = 1;
            break;

        case OPERATOR:
            exit_code = exit_code || emit(output, node->right);
            exit_code = exit_code || emit(output, node->left);
            fprintf(output, "%s\n", get_op_str(node->op));
            break;

        case LEAF:
        {
            char val[100];
            fprintf(output, "PUSH\n%s\n", get_op_val(val, node->obj));
            break;
        }

        default:
            fprintf(stderr, "unknown ASTNode kind in emit: %d\n", node->kind);
            exit_code = 1;
    }
    return exit_code;
}
