CFLAGS=-O3 -g -std=iso9899:1990
WARN_FLAGS=-Wall -Wextra -Wpedantic -Werror -Wno-unused-function

CC=gcc $(CFLAGS) $(WARN_FLAGS)

all: lexer parser stutter main
	$(CC) -o stutter main.o stutter.o growstring.o lex.yy.o y.tab.o -lfl -ly

main:
	$(CC) -c main.c

stutter: growstring
	$(CC) -c stutter.c

growstring:
	$(CC) -c growstring.c

lexer: parser
	lex tokens.l
	$(CC) -c lex.yy.c

parser:
	yacc -y -d grammar.y
	$(CC) -c y.tab.c

lint: clean
	splint *.c

test: ll_test gs_test
	rm -f testreport.log
	echo "Test results" >> testreport.log
	date >> testreport.log

	echo "Testing: gs_test" >> testreport.log && \
		valgrind ./gs_test 2>> testreport.log

	echo "Testing: ll_test" >> testreport.log && \
		valgrind ./ll_test 2>> testreport.log

	less testreport.log

ll_test:
	$(CC) -o ll_test linkedlist.c tests/ll_test.c

gs_test:
	$(CC) -o gs_test growstring.c tests/gs_test.c

clean:
	rm -f *.o
	rm -f stutter
	rm -f lex.yy.c
	rm -f y.tab.c
	rm -f y.tab.h
	rm -f testreport.log
	rm -f *_test
