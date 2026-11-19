# Performance

A simple benchmark of Reliabilitty against st, xfce4-terminal, Ptyxis,
Alacritty, Ghostty and foot. The numbers are indicative, not a full benchmark
suite.

## Environment

- Artix Linux, kernel 7.3.4-artix1-2, X11 with i3 and a compositor.
- CPU: Intel Xeon E5-1650 v4 @ 3.60 GHz.
- GPU: AMD Radeon RX 580 2048SP.
- Real desktop session, one terminal running at a time.
- Clean `$XDG_CONFIG_HOME` under `/tmp` for every terminal. xfce4-terminal and
  Ptyxis also get a private D-Bus session so their real settings cannot leak in
  through xfconf or GSettings. Ghostty behaves very differently with the real
  config; see "Ghostty and GTK4" below.
- foot is Wayland only and was run on the compositor at
  [12to11](https://github.com/fish4terrisa-MSDSM/12to11). Cross-checks with
  Alacritty on both 12to11 and plain X11 showed no significant difference in
  any metric, so the Wayland results are comparable to the X11 ones.
- Same machine for every run.

## Versions

| Terminal | Version | Renderer |
|---|---|---|
| Reliabilitty | 0.1.12 | CPU, swash |
| Alacritty | 0.17.0 | GPU, OpenGL |
| foot | 1.28.0 | CPU, fcft |
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
- CPU: user plus system CPU time of the process tree, from just after spawn
  until the draw marker appears, sampled from `/proc/<pid>/stat`. The baseline
  is taken once the child exists, and only that process tree is tracked, so
  this also works for foot whose rendering runs under 12to11. The percentage
  is that CPU time divided by the draw time. Values are quantised to 10 ms by
  the kernel clock tick. Median of 10 runs.

## Results

### Launch time

| # | Terminal | Launch time |
|--:|---|---:|
| 1 | Reliabilitty | 15.5 ms |
| 2 | foot | 25.7 ms |
| 3 | st | 48.4 ms |
| 4 | Alacritty | 132.6 ms |
| 5 | xfce4-terminal | 164.0 ms |
| 6 | Ghostty | 190.2 ms |
| 7 | Ptyxis | 312.0 ms |

### RSS

| # | Terminal | RSS |
|--:|---|---:|
| 1 | st | 15.2 MiB |
| 2 | foot | 23.6 MiB |
| 3 | Reliabilitty | 29.2 MiB |
| 4 | xfce4-terminal | 79.7 MiB |
| 5 | Alacritty | 95.5 MiB |
| 6 | Ptyxis | 138.1 MiB |
| 7 | Ghostty | 151.8 MiB |

### PSS

| # | Terminal | PSS |
|--:|---|---:|
| 1 | st | 5.6 MiB |
| 2 | foot | 11.0 MiB |
| 3 | Reliabilitty | 24.7 MiB |
| 4 | xfce4-terminal | 32.9 MiB |
| 5 | Alacritty | 46.3 MiB |
| 6 | Ptyxis | 70.1 MiB |
| 7 | Ghostty | 92.9 MiB |

### 10000 lines

| # | Terminal | Draw time |
|--:|---|---:|
| 1 | Reliabilitty | 36.5 ms |
| 2 | foot | 41.3 ms |
| 3 | st | 46.6 ms |
| 4 | Alacritty | 141.6 ms |
| 5 | xfce4-terminal | 172.7 ms |
| 6 | Ghostty | 278.9 ms |
| 7 | Ptyxis | 315.9 ms |

### CPU while drawing 10000 lines

| # | Terminal | CPU time | CPU / wall |
|--:|---|---:|---:|
| 1 | Reliabilitty | 20 ms | 51.6% |
| 2 | st | 25 ms | 52.3% |
| 3 | foot | 35 ms | 85.1% |
| 4 | Alacritty | 110 ms | 81.7% |
| 5 | xfce4-terminal | 130 ms | 65.9% |
| 6 | Ptyxis | 280 ms | 69.4% |
| 7 | Ghostty | 290 ms | 100.9% |

Reliabilitty processes the text fastest and uses the least CPU: 20 ms versus
25 ms for st and 110-290 ms for the GPU terminals. foot uses 35 ms, roughly
half of a CPU core more than st. Ptyxis and Ghostty are the heaviest of the
group despite their GTK4 stacks.

## Combined

| Terminal | Launch | RSS | PSS | Draw | CPU |
|---|---:|---:|---:|---:|---:|
| Reliabilitty | 15.5 ms | 29.2 MiB | 24.7 MiB | 36.5 ms | 20 ms (51.6%) |
| Alacritty | 132.6 ms | 95.5 MiB | 46.3 MiB | 141.6 ms | 110 ms (81.7%) |
| foot | 25.7 ms | 23.6 MiB | 11.0 MiB | 41.3 ms | 35 ms (85.1%) |
| Ghostty | 190.2 ms | 151.8 MiB | 92.9 MiB | 278.9 ms | 290 ms (100.9%) |
| Ptyxis | 312.0 ms | 138.1 MiB | 70.1 MiB | 315.9 ms | 280 ms (69.4%) |
| st | 48.4 ms | 15.2 MiB | 5.6 MiB | 46.6 ms | 25 ms (52.3%) |
| xfce4-terminal | 164.0 ms | 79.7 MiB | 32.9 MiB | 172.7 ms | 130 ms (65.9%) |

Reliabilitty is fastest to launch, draws text fastest, and uses the least CPU:
it launches about 9x faster than Alacritty, draws about 4x faster, and uses
about one fifth of the CPU. st wins RSS and PSS because it is a smaller C
process. Ghostty and Ptyxis are the slowest and heaviest of the group.

Reliabilitty uses about twice the memory it did before switching to mimalloc
as the global allocator (roughly 14 -> 29 MiB RSS). mimalloc reserves arena
memory up front; this is the main cost of the allocator change and is why st
now has a clear edge on RSS and PSS.

The GPU terminals remain slightly ahead only on raw PTY read throughput: their
vtebench totals are 659 ms and 717 ms versus Reliabilitty's 720 ms. On launch,
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

| Benchmark | Reliabilitty | Alacritty | foot | Ghostty | Ptyxis | st | xfce4-terminal |
|---|---:|---:|---:|---:|---:|---:|---:|
| cursor_motion | 8 ms | 12 ms | 7 ms | 9 ms | 19 ms | 52 ms | 19 ms |
| dense_cells | 38 ms | 32 ms | 68 ms | 33 ms | 103 ms | 1006 ms | 122 ms |
| light_cells | 6 ms | 10 ms | 8 ms | 2 ms | 6 ms | 25 ms | 7 ms |
| medium_cells | 8 ms | 11 ms | 16 ms | 10 ms | 17 ms | 34 ms | 19 ms |
| scrolling | 126 ms | 111 ms | 132 ms | 130 ms | 206 ms | 385 ms | 214 ms |
| scrolling_bottom_region | 127 ms | 110 ms | 125 ms | 122 ms | 188 ms | 389 ms | 206 ms |
| scrolling_bottom_small_region | 126 ms | 113 ms | 126 ms | 121 ms | 197 ms | 347 ms | 208 ms |
| scrolling_fullscreen | 6 ms | 12 ms | 7 ms | 27 ms | 17 ms | 29 ms | 15 ms |
| scrolling_top_region | 124 ms | 113 ms | 126 ms | 125 ms | 229 ms | 341 ms | 243 ms |
| scrolling_top_small_region | 125 ms | 112 ms | 128 ms | 125 ms | 198 ms | 349 ms | 224 ms |
| sync_medium_cells | 9 ms | 13 ms | 14 ms | 10 ms | 18 ms | 36 ms | 19 ms |
| unicode | 17 ms | 9 ms | 26 ms | 3 ms | 32 ms | 218 ms | 48 ms |
| **Total** | **720 ms** | **659 ms** | **783 ms** | **717 ms** | **1230 ms** | **3212 ms** | **1344 ms** |

Alacritty and Ghostty lead on pure PTY read throughput, but only by a narrow
margin: 659 ms and 717 ms in total versus Reliabilitty's 720 ms. Reliabilitty
is about 4x faster than st overall, ahead of both VTE terminals and foot, and
leads the GPU terminals decisively on launch time, draw time and CPU use.

## Ghostty and GTK4

The tables above use a clean config so every terminal starts on the same
footing. With the real config, Ghostty alone takes about 1.7 s to launch
versus ~180 ms clean; the others do not use GTK4 (st: Xlib, Alacritty:
winit/glutin, Reliabilitty: x11rb, xfce4-terminal: GTK3, foot: fcft/Wayland)
and are unaffected.

The cause is a GTK4 bug, not Ghostty: `GApplication::startup` does a themed
window-icon lookup that loads the whole icon theme, and its cache parser is
quadratic. The active theme, `Papirus-Dark`, has roughly 102 sub-directories
and 297,000 cache entries, so startup does about 30 million inspections and
loads the theme twice. See
[GTK4-slow-launch-speed-test](https://github.com/Samueru-sama/GTK4-slow-launch-speed-test)
for the investigation and a minimal reproduction.
