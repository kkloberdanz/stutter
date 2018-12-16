/*
 * Author: Kyle Kloberdanz
 * Date Created: 27 Nov 2018
 * License: GNU GPLv3 (see LICENSE.txt)
 * File: studder.c
 */


#include <stdio.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>


volatile char author[] = "Author: Kyle Kloberdanz";
volatile char license[] = "License: GNU GPLv3";


//typedef int64_t number;
typedef double real;
typedef int number;


typedef enum StutterType {
    NUMBER,
    REAL,
    BOOL
} StutterType;


typedef struct StutterObject {
    StutterType type;
    union {
        number number_value;
        real real_value;
        bool bool_value;
    } value;
} StutterObject;


StutterObject *make_number_obj(number n) {
    StutterObject *obj;
    if ((obj = (StutterObject*)malloc(sizeof(StutterObject))) == NULL) {
        fprintf(stderr, "failed to allocate memory");
        exit(EXIT_FAILURE);
    }
    obj->type = NUMBER;
    obj->value.number_value = n;
    return obj;
}


void destroy_obj(StutterObject *obj) {
    free(obj);
}


typedef enum {
    CONDITIONAL,
    LEAF
} ASTtype;


typedef struct ASTNode {
    ASTtype type;
    StutterObject *obj;
    struct ASTNode *left;
    struct ASTNode *condition;
    struct ASTNode *right;
} ASTNode;


int main(void) {
    return 0;
}
