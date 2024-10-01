#!/bin/bash

cargo run --release --bin $1 < ../data/$1.in > ../data/$1.txt
