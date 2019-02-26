CFLAGS=-Og -g -ansi
WARN_FLAGS=-Wall -Wextra -Wpedantic

CC=gcc $(CFLAGS) $(WARN_FLAGS)


stutter: lexer parser
	$(CC) -o stutter y.tab.c lex.yy.c stutter.c -ly -lfl

lexer:
	lex tokens.l

parser:
	yacc -y -d grammar.y
