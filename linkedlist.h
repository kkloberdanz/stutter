typedef struct linkedlist {
    void *value;
    struct linkedlist *next;
} linkedlist;


linkedlist *ll_new(void *value);
linkedlist *ll_append(linkedlist *list, void *value);
linkedlist *ll_insert(linkedlist *list, void *value);
linkedlist *ll_delete_next_node(linkedlist *list);
void ll_free(linkedlist *list);
