#!/bin/sh

# This gets the latest Mumble.proto, LICENSE, and AUTHORS from the Mumble
# github repo, then runs protoc (requires protoc-gen-rust plugin) to update
# the mumble module.

cd src/mumble
wget https://raw.githubusercontent.com/mumble-voip/mumble/master/AUTHORS -O AUTHORS
wget https://raw.githubusercontent.com/mumble-voip/mumble/master/LICENSE -O LICENSE
wget https://raw.githubusercontent.com/mumble-voip/mumble/master/src/Mumble.proto -O  Mumble.proto

protoc --rust_out . Mumble.proto
mv Mumble.rs mod.rs

