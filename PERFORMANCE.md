# Performance

A simple benchmark of Reliabilitty against st, xfce4-terminal, Ptyxis,
Alacritty and Ghostty. The numbers are indicative, not a full benchmark suite.

## Environment

- Artix Linux, kernel 7.2.15-artix1-1, X11 with i3 and a compositor.
- CPU: Intel Xeon E5-1650 v4 @ 3.60 GHz.
- GPU: AMD Radeon RX 580 2048SP.
- Real desktop session, one terminal running at a time.
- Clean `$XDG_CONFIG_HOME` under `/tmp` for every terminal. xfce4-terminal and
  Ptyxis also get a private D-Bus session so their real settings cannot leak in
  through xfconf or GSettings. Ghostty behaves very differently with the real
  config; see "Ghostty and GTK4" below.
- Same machine for every run.

## Versions

| Terminal | Version | Renderer |
|---|---|---|
| Reliabilitty | 0.1.8 | CPU, swash |
| st | 0.9.3 | CPU, Xft |
| xfce4-terminal | 1.2.0 | CPU, VTE |
| Ptyxis | 50.1 | GPU, GTK4/VTE |
| Alacritty | 0.17.0 | GPU, OpenGL |
| Ghostty | 1.4.0-arch1 | GPU, OpenGL |

## Method

- Launch time: process spawn until the active window class matches, the same
  method as the `bench-startup` helper. Median of 5 runs.
- RSS and PSS: sum of the process tree after one second idle. Median of 5
  runs.
- Draw time: spawn until a marker written by the child after printing 10000
  lines and hiding the cursor appears. This measures how fast the terminal
  processes the output through the PTY. Median of 10 runs.
- CPU: user plus system CPU time of the whole process tree, from spawn until
  the draw marker appears, measured with `getrusage(RUSAGE_CHILDREN)`. The
  percentage is that CPU time divided by the draw time. Median of 10 runs.

## Results

### Launch time

| # | Terminal | Launch time |
|--:|---|---:|
| 1 | Reliabilitty | 14.5 ms |
| 2 | st | 46.0 ms |
| 3 | Alacritty | 126.8 ms |
| 4 | xfce4-terminal | 159.3 ms |
| 5 | Ghostty | 192.8 ms |
| 6 | Ptyxis | 377.5 ms |

### RSS

| # | Terminal | RSS |
|--:|---|---:|
| 1 | st | 15.3 MiB |
| 2 | Reliabilitty | 28.8 MiB |
| 3 | xfce4-terminal | 79.5 MiB |
| 4 | Alacritty | 95.7 MiB |
| 5 | Ptyxis | 139.2 MiB |
| 6 | Ghostty | 151.6 MiB |

### PSS

| # | Terminal | PSS |
|--:|---|---:|
| 1 | st | 4.9 MiB |
| 2 | Reliabilitty | 24.3 MiB |
| 3 | xfce4-terminal | 32.4 MiB |
| 4 | Alacritty | 48.7 MiB |
| 5 | Ptyxis | 74.0 MiB |
| 6 | Ghostty | 95.4 MiB |

### 10000 lines

| # | Terminal | Draw time |
|--:|---|---:|
| 1 | Reliabilitty | 36.3 ms |
| 2 | st | 49.0 ms |
| 3 | Alacritty | 136.5 ms |
| 4 | xfce4-terminal | 162.4 ms |
| 5 | Ghostty | 278.2 ms |
| 6 | Ptyxis | 360.3 ms |

### CPU while drawing 10000 lines

| # | Terminal | CPU time | CPU / wall |
|--:|---|---:|---:|
| 1 | Reliabilitty | 7.0 ms | 21.8% |
| 2 | st | 25.7 ms | 52.3% |
| 3 | Alacritty | 74.7 ms | 55.4% |
| 4 | xfce4-terminal | 80.9 ms | 49.7% |
| 5 | Ghostty | 111.7 ms | 40.4% |
| 6 | Ptyxis | 178.6 ms | 49.4% |

Reliabilitty processes the text fastest and uses the least CPU. Ptyxis is the
slowest and heaviest of the group despite its GTK4 stack.

## Combined

| Terminal | Launch | RSS | PSS | Draw | CPU |
|---|---:|---:|---:|---:|---:|
| Reliabilitty | 14.5 ms | 28.8 MiB | 24.3 MiB | 36.3 ms | 7.0 ms (21.8%) |
| st | 46.0 ms | 15.3 MiB | 4.9 MiB | 49.0 ms | 25.7 ms (52.3%) |
| xfce4-terminal | 159.3 ms | 79.5 MiB | 32.4 MiB | 162.4 ms | 80.9 ms (49.7%) |
| Ptyxis | 377.5 ms | 139.2 MiB | 74.0 MiB | 360.3 ms | 178.6 ms (49.4%) |
| Alacritty | 126.8 ms | 95.7 MiB | 48.7 MiB | 136.5 ms | 74.7 ms (55.4%) |
| Ghostty | 192.8 ms | 151.6 MiB | 95.4 MiB | 278.2 ms | 111.7 ms (40.4%) |

Reliabilitty is fastest to launch, draws text fastest, and uses the least CPU.
st wins RSS and PSS because it is a smaller C process. Ptyxis is slower and
heavier than the other GTK/GPU terminals here.

Reliabilitty uses about twice the memory it did before switching to mimalloc
as the global allocator (roughly 14 -> 29 MiB RSS). mimalloc reserves arena
memory up front; this is the main cost of the allocator change and is why st
now has a clear edge on RSS and PSS.

The GPU terminals pay for their renderer, toolkit and GPU stack even before
text is drawn. That does not make GPU rendering useless in every case, but
for a normal terminal workload the CPU path is already cheap enough that the
extra machinery buys very little.

## vtebench

[vtebench](https://github.com/alacritty/vtebench) measures how long a child
blocks writing to the PTY while the terminal reads and processes the output.
It stresses PTY read throughput only, not frame rate or latency, so these
numbers are separate from the draw and CPU results above. Each terminal ran
the full suite with the default settings (1 MiB minimum sample, 10 seconds per
benchmark). Values are mean milliseconds; lower is better.

| Benchmark | Reliabilitty | st | xfce4-terminal | Ptyxis | Alacritty | Ghostty |
|---|---:|---:|---:|---:|---:|---:|
| cursor_motion | 15 ms | 51 ms | 19 ms | 22 ms | 12 ms | 9 ms |
| dense_cells | 38 ms | 866 ms | 137 ms | 132 ms | 33 ms | 33 ms |
| light_cells | 5 ms | 25 ms | 10 ms | 7 ms | 11 ms | 2 ms |
| medium_cells | 10 ms | 33 ms | 19 ms | 17 ms | 11 ms | 10 ms |
| scrolling | 132 ms | 383 ms | 207 ms | 211 ms | 112 ms | 127 ms |
| scrolling_bottom_region | 151 ms | 381 ms | 202 ms | 185 ms | 112 ms | 120 ms |
| scrolling_bottom_small_region | 137 ms | 336 ms | 208 ms | 202 ms | 115 ms | 120 ms |
| scrolling_fullscreen | 6 ms | 27 ms | 19 ms | 19 ms | 11 ms | 27 ms |
| scrolling_top_region | 138 ms | 333 ms | 236 ms | 228 ms | 114 ms | 123 ms |
| scrolling_top_small_region | 134 ms | 344 ms | 220 ms | 205 ms | 116 ms | 121 ms |
| sync_medium_cells | 10 ms | 35 ms | 20 ms | 18 ms | 16 ms | 10 ms |
| unicode | 17 ms | 299 ms | 50 ms | 38 ms | 9 ms | 3 ms |
| **Total** | **795 ms** | **3113 ms** | **1347 ms** | **1284 ms** | **672 ms** | **704 ms** |

The GPU terminals lead on pure PTY read throughput. Reliabilitty is about 4x
faster than st overall, ahead of both VTE terminals, and close to the GPU
terminals on most workloads.

## Ghostty and GTK4

The tables above use a clean config so every terminal starts on the same
footing. With the real config, Ghostty alone takes about 1.7 s to launch
versus ~180 ms clean; the others do not use GTK4 (st: Xlib, Alacritty:
winit/glutin, Reliabilitty: x11rb, xfce4-terminal: GTK3) and are unaffected.

The cause is a GTK4 bug, not Ghostty: `GApplication::startup` does a themed
window-icon lookup that loads the whole icon theme, and its cache parser is
quadratic. The active theme, `Papirus-Dark`, has roughly 102 sub-directories
and 297,000 cache entries, so startup does about 30 million inspections and
loads the theme twice. See
[GTK4-slow-launch-speed-test](https://github.com/Samueru-sama/GTK4-slow-launch-speed-test)
for the investigation and a minimal reproduction.
