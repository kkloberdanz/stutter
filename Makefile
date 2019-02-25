CC=gcc
CFLAGS=-Og -g
WARN_FLAGS=-Wall -Wextra -Wpedantic -Werror

parser:
	yacc grammar.y
	$(CC) -o stutter y.tab.c stutter.c -ly $(WARN_FLAGS) $(CFLAGS)
