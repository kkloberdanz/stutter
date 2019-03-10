#include <stdio.h>
#include <stdlib.h>


#include "linkedlist.h"


linkedlist *ll_new(void *value) {
    linkedlist *new_list = (linkedlist *)malloc(sizeof(linkedlist));
    if (new_list == NULL) {
        fprintf(stderr, "%s\n", "linkedlist out of memory");
        exit(EXIT_FAILURE);
    }
    new_list->value = value;
    new_list->next = NULL;
    return new_list;
}


linkedlist *ll_append(linkedlist *list, void *value) {
    linkedlist *new_node = (linkedlist *)malloc(sizeof(linkedlist));
    if (new_node == NULL) {
        fprintf(stderr, "%s\n", "linkedlist out of memory");
        exit(EXIT_FAILURE);
    }
    while (list->next) {
        list = list->next;
    }
    new_node->value = value;
    new_node->next = NULL;
    list->next = new_node;
    return new_node;
}


linkedlist *ll_insert(linkedlist *list, void *value) {
    linkedlist *new_node = (linkedlist *)malloc(sizeof(linkedlist));
    if (new_node == NULL) {
        fprintf(stderr, "%s\n", "linkedlist out of memory");
        exit(EXIT_FAILURE);
    }
    new_node->value = value;
    new_node->next = list->next;
    list->next = new_node;
    return new_node;
}


linkedlist *ll_delete_next_node(linkedlist *list) {
    linkedlist *tmp = list->next;
    list->next = list->next->next;

    if (tmp->value) {
        free(tmp->value);
    }
    tmp->value = NULL;
    free(tmp);
    return list;
}


void ll_free(linkedlist *list) {
    while (list) {
        linkedlist *tmp = list;
        list = list->next;
        tmp->next = NULL;

        if (tmp->value) {
            free(tmp->value);
        }
        tmp->value = NULL;
        free(tmp);
    }
}
