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
| Reliabilitty | 0.1.11 | CPU, swash |
| Alacritty | 0.17.0 | GPU, OpenGL |
| Ghostty | 1.4.0-arch1 | GPU, OpenGL |
| Ptyxis | 50.1 | GPU, GTK4/VTE |
| st | 0.9.3 | CPU, Xft |
| xfce4-terminal | 1.2.0 | CPU, VTE |

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
| 1 | Reliabilitty | 16.1 ms |
| 2 | st | 44.5 ms |
| 3 | Alacritty | 142.1 ms |
| 4 | xfce4-terminal | 143.3 ms |
| 5 | Ghostty | 198.9 ms |
| 6 | Ptyxis | 302.7 ms |

### RSS

| # | Terminal | RSS |
|--:|---|---:|
| 1 | st | 15.3 MiB |
| 2 | Reliabilitty | 28.9 MiB |
| 3 | xfce4-terminal | 79.4 MiB |
| 4 | Alacritty | 94.0 MiB |
| 5 | Ptyxis | 136.6 MiB |
| 6 | Ghostty | 150.3 MiB |

### PSS

| # | Terminal | PSS |
|--:|---|---:|
| 1 | st | 4.8 MiB |
| 2 | Reliabilitty | 24.3 MiB |
| 3 | xfce4-terminal | 32.1 MiB |
| 4 | Alacritty | 59.5 MiB |
| 5 | Ptyxis | 84.2 MiB |
| 6 | Ghostty | 106.7 MiB |

### 10000 lines

| # | Terminal | Draw time |
|--:|---|---:|
| 1 | Reliabilitty | 36.5 ms |
| 2 | st | 47.6 ms |
| 3 | Alacritty | 135.3 ms |
| 4 | xfce4-terminal | 164.7 ms |
| 5 | Ghostty | 285.2 ms |
| 6 | Ptyxis | 372.1 ms |

### CPU while drawing 10000 lines

| # | Terminal | CPU time | CPU / wall |
|--:|---|---:|---:|
| 1 | Reliabilitty | 7.2 ms | 17.7% |
| 2 | st | 24.8 ms | 54.8% |
| 3 | Alacritty | 76.4 ms | 54.9% |
| 4 | xfce4-terminal | 80.9 ms | 48.9% |
| 5 | Ghostty | 115.5 ms | 40.3% |
| 6 | Ptyxis | 183.7 ms | 49.0% |

Reliabilitty processes the text fastest and uses the least CPU: 7.2 ms versus
23.3 ms for st and 74.5-183.7 ms for the GPU terminals. Ptyxis is the slowest
and heaviest of the group despite its GTK4 stack.

## Combined

| Terminal | Launch | RSS | PSS | Draw | CPU |
|---|---:|---:|---:|---:|---:|
| Reliabilitty | 16.1 ms | 28.9 MiB | 24.3 MiB | 36.5 ms | 7.2 ms (17.7%) |
| Alacritty | 142.1 ms | 94.0 MiB | 59.5 MiB | 135.3 ms | 76.4 ms (54.9%) |
| Ghostty | 198.9 ms | 150.3 MiB | 106.7 MiB | 285.2 ms | 115.5 ms (40.3%) |
| Ptyxis | 302.7 ms | 136.6 MiB | 84.2 MiB | 372.1 ms | 183.7 ms (49.0%) |
| st | 44.5 ms | 15.3 MiB | 4.8 MiB | 47.6 ms | 24.8 ms (54.8%) |
| xfce4-terminal | 143.3 ms | 79.4 MiB | 32.1 MiB | 164.7 ms | 80.9 ms (48.9%) |

Reliabilitty is fastest to launch, draws text fastest, and uses the least CPU:
it launches about 9x faster than Alacritty, draws about 4x faster, and uses
about one tenth of the CPU. st wins RSS and PSS because it is a smaller C
process. Ptyxis is the slowest and heaviest of the group.

Reliabilitty uses about twice the memory it did before switching to mimalloc
as the global allocator (roughly 14 -> 29 MiB RSS). mimalloc reserves arena
memory up front; this is the main cost of the allocator change and is why st
now has a clear edge on RSS and PSS.

The GPU terminals remain slightly ahead only on raw PTY read throughput: their
vtebench totals are 673 ms and 706 ms versus Reliabilitty's 740 ms. On launch,
draw time and CPU use they are several times slower. That does not make GPU
rendering useless in every case, but for a normal terminal workload the CPU
path is already cheap enough that the extra machinery buys very little.

## vtebench

[vtebench](https://github.com/alacritty/vtebench) measures how long a child
blocks writing to the PTY while the terminal reads and processes the output.
It stresses PTY read throughput only, not frame rate or latency, so these
numbers are separate from the draw and CPU results above. Each terminal ran
the full suite with the default settings (1 MiB minimum sample, 10 seconds per
benchmark). Values are mean milliseconds; lower is better.

| Benchmark | Reliabilitty | Alacritty | Ghostty | Ptyxis | st | xfce4-terminal |
|---|---:|---:|---:|---:|---:|---:|
| cursor_motion | 9 ms | 12 ms | 9 ms | 20 ms | 52 ms | 18 ms |
| dense_cells | 41 ms | 32 ms | 33 ms | 85 ms | 1190 ms | 125 ms |
| light_cells | 6 ms | 10 ms | 2 ms | 6 ms | 25 ms | 7 ms |
| medium_cells | 9 ms | 12 ms | 10 ms | 15 ms | 34 ms | 18 ms |
| scrolling | 132 ms | 112 ms | 128 ms | 192 ms | 380 ms | 198 ms |
| scrolling_bottom_region | 130 ms | 114 ms | 120 ms | 175 ms | 384 ms | 187 ms |
| scrolling_bottom_small_region | 127 ms | 115 ms | 122 ms | 183 ms | 345 ms | 197 ms |
| scrolling_fullscreen | 6 ms | 12 ms | 27 ms | 13 ms | 28 ms | 16 ms |
| scrolling_top_region | 127 ms | 115 ms | 122 ms | 212 ms | 330 ms | 231 ms |
| scrolling_top_small_region | 127 ms | 114 ms | 121 ms | 188 ms | 346 ms | 204 ms |
| sync_medium_cells | 9 ms | 16 ms | 10 ms | 17 ms | 36 ms | 17 ms |
| unicode | 16 ms | 9 ms | 3 ms | 36 ms | 129 ms | 30 ms |
| **Total** | **740 ms** | **673 ms** | **706 ms** | **1142 ms** | **3281 ms** | **1247 ms** |

Alacritty and Ghostty lead on pure PTY read throughput, but only by a narrow
margin: 673 ms and 706 ms in total versus Reliabilitty's 740 ms. Reliabilitty
is about 4x faster than st overall, ahead of both VTE terminals, and leads the
GPU terminals decisively on launch time, draw time and CPU use.

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
