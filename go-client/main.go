package main

import (
	"fmt"
	"time"
)

/*
#cgo LDFLAGS: -L../target/release -lrust_so_example
#include "../rust_so_example.h"
#include <stdlib.h>
*/
import "C"

func main() {

	msg1 := "42aa"
	msg2 := "23"

	result := C.add(C.CString(msg1), C.CString(msg2))

	fmt.Println(result)

	resultx := C.trigger_once()
	fmt.Println(resultx)

	for {
		if C.loop_once() != 0 {
			break
		}
		time.Sleep(1 * time.Second)
	}

}
