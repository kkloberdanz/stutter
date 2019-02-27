#include <stdio.h>
#include <stdlib.h>


#include "stutter.h"


int main(int argc, char **argv) {
    char *output_filename = NULL;
    FILE *output;
    int exit_code;
    if (argc != 2) {
        fprintf(stderr, "usage: %s FILENAME\n", argv[0]);
        return 1;
    } else {
        ASTNode *tree = parse();

        if (tree == NULL) {
            fprintf(stderr, "%s\n", "failed to parse input");
            exit(EXIT_FAILURE);
        }

        output_filename = argv[1];
        output = fopen(output_filename, "w");

        if (output == NULL) {
            fprintf(stderr, "%s\n", "failed to open output file");
            exit(EXIT_FAILURE);
        }

        exit_code = emit(output, tree);
        if (fclose(output) != 0) {
            fprintf(stderr, "%s\n", "failed to close output file");
            exit(EXIT_FAILURE);
        }
        destroy_ast_node(tree);
        return exit_code;
    }
}
