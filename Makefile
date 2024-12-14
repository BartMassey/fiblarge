CC = clang
CFLAGS = -O2 -Wall

fiblarge: fiblarge.c
	$(CC) $(CFLAGS) -o fiblarge fiblarge.c -lgmp

clean:
	-rm -rf fiblarge
