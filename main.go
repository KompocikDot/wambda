package main

import (
	"context"
	"fmt"
	"log"
	"os"

	"github.com/tetratelabs/wazero"
	"github.com/tetratelabs/wazero/api"
	"github.com/tetratelabs/wazero/imports/wasi_snapshot_preview1"
)

const MaxFileSize = 2 << 20 * 256 // 256Mb

func main() {
	if len(os.Args) < 2 {
		log.Fatalln("Pass WASM filename path, to read file from.")
	}

	wasmCode := readWasmFile(os.Args[1])
	ctx := context.Background()
	runSandboxedCode(ctx, wasmCode)
}

func readWasmFile(filename string) []byte {
	f, err := os.OpenFile(filename, os.O_RDONLY, 0400)
	if err != nil {
		log.Fatalf("Could not open file: %s, err: %v\n", filename, err)
	}

	fstat, err := f.Stat()

	fileSize := fstat.Size()
	if fileSize > MaxFileSize {
		log.Fatalf("File is bigger than maximally allowed(256Mb), file size: %d\n", fileSize)
	}

	defer f.Close()

	buf := make([]byte, fileSize)

	_, err = f.Read(buf)
	if err != nil {
		log.Fatalf("Could not read file: %s, due to an error: %v\n", filename, err)
	}

	return buf
}

func runSandboxedCode(ctx context.Context, wasmCode []byte) {
	runtime := wazero.NewRuntime(ctx)
	defer runtime.Close(ctx)

	wasi_snapshot_preview1.MustInstantiate(ctx, runtime)

	module, err := runtime.InstantiateWithConfig(ctx, wasmCode, wazero.NewModuleConfig().WithStartFunctions("_initialize"))
	if err != nil {
		log.Fatalf("Could not instantiate module due to an error: %v\n", err)
	}

	fn := module.ExportedFunction("add")

	var a int32 = 5
	var b int32 = 6
	res, err := fn.Call(ctx, api.EncodeI32(a), api.EncodeI32(b))
	// if err != nil && err. {
	// 	log.Fatalf("Could not call function due to an error: %v\n", err)
	// }

	fmt.Println(api.DecodeI32(res[0]))
}
