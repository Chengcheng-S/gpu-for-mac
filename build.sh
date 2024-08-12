#!/bin/bash
#exit on error
set -e

xcrun -sdk macosx metal -c src/shaders.metal -o src/shaders.air
xcrun -sdk macosx metallib src/shaders.air -o src/shaders.metallib