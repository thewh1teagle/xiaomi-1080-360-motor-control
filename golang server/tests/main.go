package main

/*
#include stdio.h
#include stdlib.h

void myprint(char* s) {
    printf("This is in C code : %s\n", s);
}
*/
import "C"

import "unsafe"

func Example() {
	cs := C.CString("This is passed from Go Code\n")
	C.myprint(cs)
	C.free(unsafe.Pointer(cs))
}

func main() {
	Example()
}
