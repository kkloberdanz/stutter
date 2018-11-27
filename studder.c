/*
 * Programmer: Kyle Kloberdanz
 * Date Created: 27 Nov 2018
 * License: GNU GPLv3 (see LICENSE.txt)
 * File: studder.c
 */


#include <stdio.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>


typedef int64_t number;
typedef double real;


typedef enum StudderType {
    NUMBER,
    REAL,
    BOOL
} StudderType;


typedef struct StudderObject {
    StudderType type;
    union {
        number number_value;
        real real_value;
        bool bool_value;
    };
} StudderObject;


StudderObject *make_number_obj(number n) {
    StudderObject *obj = (void*)malloc(sizeof(StudderObject));
    obj->type = NUMBER;
    obj->number_value = n;
    return obj;
}


void destroy_number_obj(StudderObject *obj) {
    free(obj);
}


typedef enum {
    CONDITIONAL,
    LEAF
} ASTtype;


typedef struct ASTNode {
    ASTtype type;
    StudderObject *obj;
    struct ASTNode *left;
    struct ASTNode *condition;
    struct ASTNode *right;
} ASTNode;


int main(void) {
    return 0;
}
