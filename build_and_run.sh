#!/bin/bash
g++ -std=c++20 -o build/$1 src/$1.cpp && ./build/$1
