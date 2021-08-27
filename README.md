# pngshh

This is a utility made in rust that can encode and decode messages from PNG files.

It follows the [PNG Specification, Version 1.2](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html) document.

## Usage

Build with `cargo build --release`

Encode: `./pngshh encode <FILE_PATH> <CHUNK_TYPE> <MESSAGE> [OUTPUT_FILE]`

Decode: `./pngshh decode <FILE_PATH> <CHUNK_TYPE>`

Remove: `./pngshh remove <FILE_PATH> <CHUNK_TYPE>`

Print Chunks: `./pngshh print <FILE_PATH>`

**For example**:

`/.pngshh encode image.png hola "This is my secret message"`

> Successfully encoded.

`./pngshh decode image.png hola`

> Decoded: This is my secret message

`./pngshh remove image.png hola`

> Successfully removed.

`./pngshh print image.png`

> Chunk {
> Length: 13
> Type: IHDR
> Data: 13 bytes
> Crc: 1807389920
> }
>
> Chunk {
> Length: 4
> Type: gAMA
> Data: 4 bytes
> Crc: 201089285
> }
>
> Chunk {
> Length: 46
> Type: tEXt
> Data: 46 bytes
> Crc: 1223306120
> }
>
> Chunk {
> Length: 8192
> Type: IDAT
> Data: 8192 bytes
> Crc: 1808155945
> }
>
> Chunk {
> Length: 8192
> Type: IDAT
> Data: 8192 bytes
> Crc: 977458620
> }
>
> Chunk {
> Length: 4833
> Type: IDAT
> Data: 4833 bytes
> Crc: 4100335391
> }
>
> Chunk {
> Length: 7
> Type: tIME
> Data: 7 bytes
> Crc: 3731693981
> }
>
> Chunk {
> Length: 0
> Type: IEND
> Data: 0 bytes
> Crc: 2923585666
> }
