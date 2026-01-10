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

func main() {
	if len(os.Args) < 2 {
		log.Fatalln("Pass WASM filename path, to read file from.")
	}
	ctx := context.Background()

	// 64Mb buffer
	buf := make([]byte, 2<<20*64)

	// Read only file for owner
	// f, err := os.OpenFile(os.Args[1], os.O_RDONLY, 0400)
	// if err != nil {
	// 	log.Fatalln(err)
	// }
	//
	// readSize, err := f.Read(buf)
	// if err != nil {
	// 	log.Panicln(err)
	// }

	// log.Printf("Read: %d bytes\n", readSize)

	buf, err := os.ReadFile(os.Args[1])
	if err != nil {
		log.Fatalf("Could not read file: %s\n", os.Args[1])
	}

	runtime := wazero.NewRuntime(ctx)
	defer runtime.Close(ctx)

	wasi_snapshot_preview1.MustInstantiate(ctx, runtime)

	module, err := runtime.InstantiateWithConfig(ctx, buf, wazero.NewModuleConfig().WithStartFunctions("_initialize"))
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
