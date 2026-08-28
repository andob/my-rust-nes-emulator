## My Rust NES emulator

This is a NES (Nintendo Entertainment System) emulator I wrote in Rust. The project is still in alpha phase.

[![ My NES emulator (initial alpha release) ](https://markdown-videos-api.jorgenkh.no/url?url=https%3A%2F%2Fwww.youtube.com%2Fwatch%3Fv%3DWetlhicTpn0)](https://www.youtube.com/watch?v=WetlhicTpn0)

### DONE so far:

✅ Complete 6502 CPU implementation, including unofficial opcodes

✅ The CPU implementation pass both kevtris and blargg NES test ROMs

✅ Mimimally functional PPU (Picture Processing Unit) implementation

✅ Mapper-0 games (Donkey Kong, Pinball, SMB1) playable at 60FPS

✅ USB controller / joystick support

### TO DO:

• complete PPU implementation: hardware accurate scanline-by-scanline rendering, sprite zero hit detection, sprite overflow detection, accurate colors, multiple scrolling mechanisms

• APU (Audio Processing Unit) implementation

• implementation of more ROM mappers

• save state and restore state features

### License

```
Copyright 2023 - present Dobrescu Andrei

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```
