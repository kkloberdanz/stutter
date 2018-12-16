CC=gcc

parser:
	yacc grammar.y
	$(CC) -o grammar y.tab.c -ly

all:
	gcc stutter.c -Wall -Wextra -Wpedantic -ansi
