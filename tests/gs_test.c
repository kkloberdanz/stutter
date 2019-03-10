#include <stdio.h>
#include <stdlib.h>


#include "../growstring.h"


/* run this with valgrind to check for memory errors / corruption / leaks */
int main(void) {
    int i;
    int j;
    growstring *gs1;
    growstring *gs2;

    gs1 = gs_new();
    gs2 = gs_new();
    for (j = 0; j < 100; j++) {
        for (i = 'a'; i <= 'z'; i++) {
            gs1 = gs_append(gs1, i);
            printf("gs = %s\n", gs_get_str(gs1));
        }
    }

    gs_write(gs2, "this is a string");

    printf("gs2 = %s\n", gs_get_str(gs2));

    gs_concat(gs1, gs2);
    
    printf("gs1 is now %s\n", gs_get_str(gs1));

    gs_free(gs1);
    gs_free(gs2);
    return 0;
}
