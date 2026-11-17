# Performance

A simple benchmark of Reliabilitty against st, Alacritty and Ghostty. The
numbers are indicative, not a full benchmark suite.

## Environment

- Artix Linux, X11 with i3 and a compositor.
- Real desktop session, one terminal running at a time.
- Clean `$XDG_CONFIG_HOME` under `/tmp` for every terminal. Ghostty behaves
  very differently with the real config; see "Ghostty and GTK4" below.
- Same machine for every run.

## Versions

| Terminal | Version | Renderer |
|---|---|---|
| Reliabilitty | 0.1.3 | CPU, swash |
| st | 0.9.3 | CPU, Xft |
| Alacritty | 0.17.0 | GPU, OpenGL |
| Ghostty | 1.4.0-arch1 | GPU, OpenGL |

## Method

- Launch time: process spawn until the active window class matches, the same
  method as the `bench-startup` helper. Median of 3 runs.
- RSS and PSS: sum of the process tree after one second idle. Median of 3
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
| 1 | Reliabilitty | 22.5 ms |
| 2 | st | 44.5 ms |
| 3 | Alacritty | 125.9 ms |
| 4 | Ghostty | 189.7 ms |

### RSS

| # | Terminal | RSS |
|--:|---|---:|
| 1 | st | 15.3 MiB |
| 2 | Reliabilitty | 30.8 MiB |
| 3 | Alacritty | 96.1 MiB |
| 4 | Ghostty | 152.0 MiB |

### PSS

| # | Terminal | PSS |
|--:|---|---:|
| 1 | st | 6.5 MiB |
| 2 | Reliabilitty | 26.3 MiB |
| 3 | Alacritty | 79.5 MiB |
| 4 | Ghostty | 128.0 MiB |

### 10000 lines

| # | Terminal | Draw time |
|--:|---|---:|
| 1 | Reliabilitty | 29.4 ms |
| 2 | st | 36.4 ms |
| 3 | Alacritty | 128.8 ms |
| 4 | Ghostty | 265.4 ms |

### CPU while drawing 10000 lines

| # | Terminal | CPU time | CPU / wall |
|--:|---|---:|---:|
| 1 | Reliabilitty | 7.4 ms | 24.4% |
| 2 | st | 19.6 ms | 53.2% |
| 3 | Alacritty | 70.7 ms | 54.8% |
| 4 | Ghostty | 101.4 ms | 38.0% |

Reliabilitty processes the text fastest and uses the least CPU. The GPU
terminals burn several times more CPU on the same text.

## Combined

| Terminal | Launch | RSS | PSS | Draw | CPU |
|---|---:|---:|---:|---:|---:|
| Reliabilitty | 22.5 ms | 30.8 MiB | 26.3 MiB | 29.4 ms | 7.4 ms (24.4%) |
| st | 44.5 ms | 15.3 MiB | 6.5 MiB | 36.4 ms | 19.6 ms (53.2%) |
| Alacritty | 125.9 ms | 96.1 MiB | 79.5 MiB | 128.8 ms | 70.7 ms (54.8%) |
| Ghostty | 189.7 ms | 152.0 MiB | 128.0 MiB | 265.4 ms | 101.4 ms (38.0%) |

Reliabilitty is fastest to launch, draws text fastest, and uses the least CPU.
st wins RSS and PSS because it is a smaller C process. Both CPU terminals are
far lighter than the GPU terminals in this workload.

Reliabilitty uses about twice the memory it did before switching to mimalloc
as the global allocator (roughly 14 -> 31 MiB RSS). mimalloc reserves arena
memory up front; this is the main cost of the allocator change and is why st
now has a clear edge on RSS and PSS.

The GPU terminals pay for their renderer, toolkit and GPU stack even before
text is drawn. That does not make GPU rendering useless in every case, but
for a normal terminal workload the CPU path is already cheap enough that the
extra machinery buys very little.

## Ghostty and GTK4

The table above uses a clean config so every terminal starts on the same
footing. With the real config, Ghostty alone takes about 1.7 s to launch
(measured repeatedly, matching the `bench-startup` result). The other
terminals are unaffected because none of them use GTK4:

- st uses Xlib directly.
- Alacritty uses winit/glutin.
- Reliabilitty uses x11rb.

The cause is a GTK4 bug, not Ghostty. GTK4's `GApplication::startup` does a
themed window-icon lookup before the window appears, which forces the whole
icon theme to load. The cache parser is quadratic: for each of the theme's
sub-directories it sweeps every entry in the icon cache. The active theme
here is `Papirus-Dark`, with roughly 102 sub-directories and 297,000 cache
entries, about 30 million inspections. GTK4 also loads the theme twice during
startup. The main thread just waits in `futex` for the worker thread doing
that CPU-bound parse, so there is no slow I/O to see in a syscall trace.

With a clean config GTK uses a small default icon theme, so Ghostty lands at
the ~180 ms GTK4 floor. The 1.7 s result comes from loading `Papirus-Dark`,
not from the shell or the number of config files.

See [GTK4-slow-launch-speed-test](https://github.com/Samueru-sama/GTK4-slow-launch-speed-test)
for the full investigation and a minimal GTK3/GTK4 reproduction.
