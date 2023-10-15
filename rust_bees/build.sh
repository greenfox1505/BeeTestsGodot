#!/bin/bash


main()
{
cargo build
cargo build --release
}

cd $(dirname $0)

main


