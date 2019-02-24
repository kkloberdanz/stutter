CC=gcc
CFLAGS=-Og -g

parser:
	yacc grammar.y
	$(CC) -o grammar y.tab.c stutter.c -ly $(CFLAGS)

all:
	gcc stutter.c -Wall -Wextra -Wpedantic -ansi $(CFLAGS)
