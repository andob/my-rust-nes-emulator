#!/bin/bash
set -e #fail on first error

cargo run --package nes-emulator --bin nes-emulator test cpu_kevtris_nestest

echo "ALL TESTS PASSED!!!"