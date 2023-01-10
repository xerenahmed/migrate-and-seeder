#!/bin/bash
set -e

subcommand=$1

if [ "$subcommand" == "seed" ]; then
	cd seeder
	cargo build --release
	./target/release/seeder
	cd ..

	exit 0
fi

database_uri=(`cat database_uri.txt`)
if [ "$subcommand" == "up" ]; then
	migrate -database "$database_uri" -path migrations up $2
elif [ "$subcommand" == "down" ]; then
	migrate -database "$database_uri" -path migrations down $2
elif [ "$subcommand" == "flush" ]; then
	migrate -database "$database_uri" -path migrations flushr$2
fi
