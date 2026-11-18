# Performance

A simple benchmark of Reliabilitty against st, xfce4-terminal, Alacritty and
Ghostty. The numbers are indicative, not a full benchmark suite.

## Environment

- Artix Linux, kernel 7.2.15-artix1-1, X11 with i3 and a compositor.
- CPU: Intel Xeon E5-1650 v4 @ 3.60 GHz.
- GPU: AMD Radeon RX 580 2048SP.
- Real desktop session, one terminal running at a time.
- Clean `$XDG_CONFIG_HOME` under `/tmp` for every terminal. Ghostty behaves
  very differently with the real config; see "Ghostty and GTK4" below.
- Same machine for every run.

## Versions

| Terminal | Version | Renderer |
|---|---|---|
| Reliabilitty | 0.1.5 | CPU, swash |
| st | 0.9.3 | CPU, Xft |
| xfce4-terminal | 1.2.0 | CPU, VTE |
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
| 1 | Reliabilitty | 13.7 ms |
| 2 | st | 48.2 ms |
| 3 | xfce4-terminal | 126.2 ms |
| 4 | Alacritty | 139.3 ms |
| 5 | Ghostty | 197.8 ms |

### RSS

| # | Terminal | RSS |
|--:|---|---:|
| 1 | st | 15.3 MiB |
| 2 | Reliabilitty | 28.8 MiB |
| 3 | xfce4-terminal | 78.8 MiB |
| 4 | Alacritty | 95.5 MiB |
| 5 | Ghostty | 151.9 MiB |

### PSS

| # | Terminal | PSS |
|--:|---|---:|
| 1 | st | 4.6 MiB |
| 2 | Reliabilitty | 24.2 MiB |
| 3 | xfce4-terminal | 28.7 MiB |
| 4 | Alacritty | 46.7 MiB |
| 5 | Ghostty | 93.2 MiB |

### 10000 lines

| # | Terminal | Draw time |
|--:|---|---:|
| 1 | Reliabilitty | 34.2 ms |
| 2 | st | 45.7 ms |
| 3 | Alacritty | 129.9 ms |
| 4 | xfce4-terminal | 152.0 ms |
| 5 | Ghostty | 270.0 ms |

### CPU while drawing 10000 lines

| # | Terminal | CPU time | CPU / wall |
|--:|---|---:|---:|
| 1 | Reliabilitty | 8.3 ms | 23.2% |
| 2 | st | 24.0 ms | 53.9% |
| 3 | Alacritty | 71.9 ms | 54.5% |
| 4 | xfce4-terminal | 73.1 ms | 48.8% |
| 5 | Ghostty | 109.9 ms | 40.2% |

Reliabilitty processes the text fastest and uses the least CPU. xfce4-terminal
sits between the lightweight terminals and the GPU terminals on every metric.

## Combined

| Terminal | Launch | RSS | PSS | Draw | CPU |
|---|---:|---:|---:|---:|---:|
| Reliabilitty | 13.7 ms | 28.8 MiB | 24.2 MiB | 34.2 ms | 8.3 ms (23.2%) |
| st | 48.2 ms | 15.3 MiB | 4.6 MiB | 45.7 ms | 24.0 ms (53.9%) |
| xfce4-terminal | 126.2 ms | 78.8 MiB | 28.7 MiB | 152.0 ms | 73.1 ms (48.8%) |
| Alacritty | 139.3 ms | 95.5 MiB | 46.7 MiB | 129.9 ms | 71.9 ms (54.5%) |
| Ghostty | 197.8 ms | 151.9 MiB | 93.2 MiB | 270.0 ms | 109.9 ms (40.2%) |

Reliabilitty is fastest to launch, draws text fastest, and uses the least CPU.
st wins RSS and PSS because it is a smaller C process. xfce4-terminal is close
to Alacritty on draw time and CPU, but uses less memory and starts slightly
faster. Reliabilitty and st are far lighter than the GPU terminals here.

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

| Benchmark | Reliabilitty | st | xfce4-terminal | Alacritty | Ghostty |
|:---:|:---:|:---:|:---:|:---:|
| cursor_motion | 16 ms | 52 ms | 23 ms | 12 ms | 9 ms |
| dense_cells | 39 ms | 940 ms | 578 ms | 33 ms | 33 ms |
| light_cells | 5 ms | 25 ms | 9 ms | 10 ms | 2 ms |
| medium_cells | 10 ms | 34 ms | 21 ms | 12 ms | 10 ms |
| scrolling | 276 ms | 379 ms | 190 ms | 109 ms | 129 ms |
| scrolling_bottom_region | 219 ms | 385 ms | 194 ms | 111 ms | 120 ms |
| scrolling_bottom_small_region | 226 ms | 345 ms | 213 ms | 112 ms | 119 ms |
| scrolling_fullscreen | 13 ms | 27 ms | 19 ms | 12 ms | 28 ms |
| scrolling_top_region | 228 ms | 331 ms | 272 ms | 112 ms | 124 ms |
| scrolling_top_small_region | 376 ms | 341 ms | 219 ms | 112 ms | 120 ms |
| sync_medium_cells | 10 ms | 36 ms | 20 ms | 13 ms | 10 ms |
| unicode | 18 ms | 132 ms | 94 ms | 9 ms | 3 ms |
| **Total** | **1438 ms** | **3027 ms** | **1852 ms** | **657 ms** | **707 ms** |

The GPU terminals lead on pure PTY read throughput. Reliabilitty is about 2x
faster than st overall and well ahead on the cell and unicode workloads, but
VTE is faster on several of the line-scrolling benchmarks.

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
