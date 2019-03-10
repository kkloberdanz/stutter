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
            str = "+";
            break;

        case SUB:
            str = "-";
            break;

        case MUL:
            str = "*";
            break;

        case DIV:
            str = "/";
            break;

        case NOP:
            str = ";";
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


static char *next_variable(void) {
    static char var[1] = {'a' - 1};
    var[0]++;
    return var;
}


/* code generation */
static linkedlist *codegen_stack_machine(const ASTNode *node) {
    linkedlist *new_node = NULL;
    switch (node->kind) {
        case CONDITIONAL:
            fprintf(stderr, "CONDITIONAL not implemented");
            exit(EXIT_FAILURE);
            break;

        case OPERATOR:
            ll_append(codegen_stack_machine(node->right));
            ll_append(codegen_stack_machine(node->left));
            ll_append("%s\n", get_op_str(node->op));
            break;

        case LEAF:
        {
            char val[100];
            fprintf(output, "PUSH\n%s\n", get_op_val(val, node->obj));
            break;
        }

        default:
            fprintf(stderr, "unknown ASTNode kind in emit: %d\n", node->kind);
    }
    return new_node;
}


static growstring *emit_helper(const ASTNode *node) {
    growstring *program = gs_new();
    switch (node->kind) {
        case CONDITIONAL:
        {
            fprintf(stderr, "CONDITIONAL not implemented");
            exit(EXIT_FAILURE);
            break;
        }

        case OPERATOR:
        {
            growstring *op_gs = gs_new();
            growstring *left_gs;
            growstring *right_gs;

            /* get left expr */
            left_gs = emit_helper(node->left);

            /* store left expr into program */
            gs_concat(program, left_gs);

            /* get op and store it into program */
            gs_write(op_gs, get_op_str(node->op));
            gs_concat(program, op_gs);

            /* get right expr */
            right_gs = emit_helper(node->right);

            /* store right expr into program */
            gs_concat(program, right_gs);

            gs_free(op_gs);
            gs_free(left_gs);
            gs_free(right_gs);
            break;
        }

        case LEAF:
        {
            char val[100];
            char str[100];
            sprintf(str, " %s ", get_op_val(val, node->obj));
            gs_write(program, str);
            break;
        }

        default:
            fprintf(stderr, "unknown ASTNode kind in emit: %d\n", node->kind);
            exit(EXIT_FAILURE);
    }

    return program;
}


/* code generation */
int emit(FILE *output, const ASTNode *node) {
    char *begin_boilerplate;
    char *end_boilerplate;
    growstring *program = gs_new();
    program = emit_helper(node);
    begin_boilerplate = "#include <stdio.h>\nint main(void) {\n";
    end_boilerplate = "    return 0;\n}\n";
    fprintf(output, "%s    printf(\"ans = %%d\\n\", %s);\n%s", begin_boilerplate,
                                    gs_get_str(program),
                                    end_boilerplate);
    gs_free(program);
    return 0;
}
