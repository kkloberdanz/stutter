#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "growstring.h"


growstring *gs_new(void) {
    char *data = NULL;
    int capacity = 10;
    growstring *new_gs = (growstring *)malloc(sizeof(growstring));
    if (new_gs == NULL) {
        fprintf(stderr, "%s\n", "out of memory when allocating string");
        exit(EXIT_FAILURE);
    }

    data = (char *)calloc(capacity + 1, sizeof(char));
    if (data == NULL) {
        fprintf(stderr, "%s\n", "out of memory when allocating string");
        exit(EXIT_FAILURE);
    }
    new_gs->data = data;
    new_gs->capacity = capacity;
    new_gs->size = 0;
    return new_gs;
}


growstring *gs_append(growstring *dest, const char letter) {
    if (dest->size >= dest->capacity) {
        char *new_data;
        size_t new_capacity = 1 + dest->capacity * 2;
        new_data = (char *)realloc(dest->data,
                                   sizeof(char) * new_capacity + 1);
        if (new_data == NULL) {
            fprintf(stderr, "%s\n", "out of memory when allocating string");
            exit(EXIT_FAILURE);
        } else {
            dest->data = new_data;
            dest->capacity = new_capacity;
        }
    }
    dest->data[dest->size] = letter;
    dest->data[dest->size+1] = '\0';
    dest->size++;
    return dest;
}


void gs_free(growstring *gs) {
    free(gs->data);
    gs->data = NULL;
    gs->capacity = 0;
    gs->size = 0;
    free(gs);
}


char *gs_get_str(const growstring *gs) {
    return gs->data;
}


growstring *gs_write(growstring *dest, const char *data) {
    char *new_data = NULL;
    size_t new_data_size = strlen(data) + 1;
    free(dest->data);
    new_data = (char *)calloc(new_data_size, sizeof(char));
    if (new_data == NULL) {
        fprintf(stderr, "%s\n", "out of memory when allocating string");
        exit(EXIT_FAILURE);
    }
    dest->data = new_data;
    dest->size = new_data_size - 1;
    dest->capacity = new_data_size - 1;
    strcpy(dest->data, data);
    return dest;
}


growstring *gs_concat(growstring *dest, const growstring *src) {
    char *new_data = NULL;
    size_t new_capacity;
    size_t new_size;

    new_size = strlen(src->data) + strlen(dest->data);
    new_capacity = sizeof(char) * new_size;
    new_data = (char *)realloc(dest->data,
                               new_capacity + 1);
    if (new_data == NULL) {
        fprintf(stderr, "%s\n", "out of memory when allocating string");
        exit(EXIT_FAILURE);
    }
    strcat(new_data, gs_get_str(src));
    dest->data = new_data;
    dest->capacity = new_capacity;
    dest->size = new_size;
    return dest;
}
