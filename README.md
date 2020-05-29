# `vapor_archive` - a .ZIP clone written in Rust

## About

`vapor_archive` is, for the most part, a clone of the ZIP file format, with a few differences:
* File headers are stored after the data (making them footers),  
but the file format doesnt care about this
* Uses BLAKE2S instead of CRC32 for the checksums
* Supports ZStandard compression

## Why?

No real reason really. I just needed my own archive format for my game engine that i could design as i please.

## How does it work?

Once i get around to it, you will find the file format structure/specification [here.](SPEC.md)

## How do i use it?

See the integration tests at `tests/` for a pointer. I will update the documentation as soon as im able too.
