CFLAGS=-Og -g -ansi
WARN_FLAGS=-Wall -Wextra -Wpedantic -Werror -Wno-unused-function

CC=gcc $(CFLAGS) $(WARN_FLAGS)

all: stutter
	$(CC) -o stutter stutter.o lex.yy.o y.tab.o -lfl -ly

stutter: lexer parser
	$(CC) -c stutter.c

lexer: parser
	lex tokens.l
	$(CC) -c lex.yy.c

parser:
	yacc -y -d grammar.y
	$(CC) -c y.tab.c

clean:
	rm -f *.o
	rm -f stutter
	rm -f lex.yy.c
	rm -f y.tab.c
	rm -f y.tab.h
