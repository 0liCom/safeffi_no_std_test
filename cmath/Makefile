CC = gcc

all: static

static: cmath.o
	ar -rcs libcmath.a cmath.o

cmath.o:
	$(CC) -c cmath.c -o cmath.o -Wall

clean:
	rm -f libcmath.a
	rm -f cmath.o

