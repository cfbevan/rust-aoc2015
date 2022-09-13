#!/bin/bash

day=${1##+(0)}
project=$(printf "day%02d" $1)
session="$AOC_SESSION"

cargo new ${project}

cd ${project}

# get input
curl -s "https://adventofcode.com/2015/day/${day}/input" --cookie "session=${AOC_SESSION}" -o input.txt

# add readme
touch README.md

echo -n 'fn main() {
    let data = include_str!("../input.txt").trim();
    println!(
        "Part 1: {}",
        ""
    );

    println!(
        "Part 2: {}",
        ""
    );
}' > src/main.rs
