#include <stdio.h>
#include <stdlib.h>


#include "../linkedlist.h"


static void print_int_list(linkedlist *ll) {
    int value;
    while (ll) {
        if (ll->value != NULL) {
            value = *(int *)(ll->value);
            printf("%d\n", value);
        }
        ll = ll->next;
    }
}


int test_concat() {
    linkedlist *ll1;
    linkedlist *ll2;
    int *i = (int *)malloc(sizeof(int));
    int *j = (int *)malloc(sizeof(int));
    int *k = (int *)malloc(sizeof(int));
    int *l = (int *)malloc(sizeof(int));

    puts("testing concat");

    *i = 1;
    *j = 2;
    *k = 3;
    *l = 100;

    ll1 = ll_new(i);
    ll_append(ll1, j);

    ll2 = ll_new(k);
    ll_append(ll2, l);

    puts("ll1");
    print_int_list(ll1);
    puts("ll2");
    print_int_list(ll2);

    ll_concat(ll1, ll2);

    puts("concat");
    print_int_list(ll1);

    ll_free(ll1);

    return 1;
}


int test_basic() {
    linkedlist *ll;
    int *i = (int *)malloc(sizeof(int));
    int *j = (int *)malloc(sizeof(int));
    int *k = (int *)malloc(sizeof(int));
    int *l = (int *)malloc(sizeof(int));

    puts("testing basic usage");

    *i = 1;
    *j = 2;
    *k = 3;
    *l = 100;

    ll = ll_new(i);

    ll_append(ll, j);
    ll_append(ll, k);
    ll_append(ll, l);
    ll_append(ll, NULL);

    print_int_list(ll);

    ll_free(ll);
    return 1;
}


int main(void) {
    test_basic();
    test_concat();
    puts("done testing linkedlist");
    return 0;
}
