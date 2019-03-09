#ifndef GROWSTRING_H
#define GROWSTRING_H


#include <stdlib.h>


typedef struct growstring {
    size_t capacity;
    size_t size;
    char *data;
} growstring;


growstring *gs_new(void);
growstring *gs_append(growstring *dest, const char letter);
void gs_free(growstring *gs);
char *gs_get_str(const growstring *gs);
growstring *gs_write(growstring *dest, const char *data);
growstring *gs_concat(growstring *dest, const growstring *src);


#endif /* GROWSTRING_H */
