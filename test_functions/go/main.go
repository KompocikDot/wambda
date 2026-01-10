package main

func main() {}

//go:wasmexport add
func add(a, b int32) int32 {
	return a + b
}
