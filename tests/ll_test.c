#include <stdio.h>
#include <stdlib.h>


#include "../linkedlist.h"


int main(void) {
    int value;
    linkedlist *ll;
    linkedlist *head;
    int *i = (int *)malloc(sizeof(int));
    int *j = (int *)malloc(sizeof(int));
    int *k = (int *)malloc(sizeof(int));
    int *l = (int *)malloc(sizeof(int));

    *i = 1;
    *j = 2;
    *k = 3;
    *l = 100;

    ll = ll_new(i);

    ll_append(ll, j);
    ll_append(ll, k);
    ll_append(ll, l);
    ll_append(ll, NULL);

    head = ll;
    while (ll) {
        if (ll->value != NULL) {
            value = *(int *)(ll->value);
            printf("%d\n", value);
        }
        ll = ll->next;
    }

    ll_free(head);

    return 0;
}
