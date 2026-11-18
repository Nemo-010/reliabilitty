# Performance

A simple benchmark of Reliabilitty against st, xfce4-terminal, Alacritty and
Ghostty. The numbers are indicative, not a full benchmark suite.

## Environment

- Artix Linux, X11 with i3 and a compositor.
- Real desktop session, one terminal running at a time.
- Clean `$XDG_CONFIG_HOME` under `/tmp` for every terminal. Ghostty behaves
  very differently with the real config; see "Ghostty and GTK4" below.
- Same machine for every run.

## Versions

| Terminal | Version | Renderer |
|---|---|---|
| Reliabilitty | 0.1.4 | CPU, swash |
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
| 1 | Reliabilitty | 15.3 ms |
| 2 | st | 45.0 ms |
| 3 | xfce4-terminal | 119.9 ms |
| 4 | Alacritty | 128.3 ms |
| 5 | Ghostty | 195.6 ms |

### RSS

| # | Terminal | RSS |
|--:|---|---:|
| 1 | st | 15.2 MiB |
| 2 | Reliabilitty | 30.8 MiB |
| 3 | xfce4-terminal | 78.8 MiB |
| 4 | Alacritty | 96.0 MiB |
| 5 | Ghostty | 152.0 MiB |

### PSS

| # | Terminal | PSS |
|--:|---|---:|
| 1 | st | 4.5 MiB |
| 2 | Reliabilitty | 26.2 MiB |
| 3 | xfce4-terminal | 28.6 MiB |
| 4 | Alacritty | 47.1 MiB |
| 5 | Ghostty | 93.4 MiB |

### 10000 lines

| # | Terminal | Draw time |
|--:|---|---:|
| 1 | Reliabilitty | 35.7 ms |
| 2 | st | 47.0 ms |
| 3 | Alacritty | 131.8 ms |
| 4 | xfce4-terminal | 145.9 ms |
| 5 | Ghostty | 283.4 ms |

### CPU while drawing 10000 lines

| # | Terminal | CPU time | CPU / wall |
|--:|---|---:|---:|
| 1 | Reliabilitty | 8.1 ms | 24.7% |
| 2 | st | 24.7 ms | 54.7% |
| 3 | xfce4-terminal | 70.2 ms | 48.1% |
| 4 | Alacritty | 74.8 ms | 57.1% |
| 5 | Ghostty | 115.0 ms | 41.1% |

Reliabilitty processes the text fastest and uses the least CPU. xfce4-terminal
sits between the lightweight terminals and the GPU terminals on every metric.

## Combined

| Terminal | Launch | RSS | PSS | Draw | CPU |
|---|---:|---:|---:|---:|---:|
| Reliabilitty | 15.3 ms | 30.8 MiB | 26.2 MiB | 35.7 ms | 8.1 ms (24.7%) |
| st | 45.0 ms | 15.2 MiB | 4.5 MiB | 47.0 ms | 24.7 ms (54.7%) |
| xfce4-terminal | 119.9 ms | 78.8 MiB | 28.6 MiB | 145.9 ms | 70.2 ms (48.1%) |
| Alacritty | 128.3 ms | 96.0 MiB | 47.1 MiB | 131.8 ms | 74.8 ms (57.1%) |
| Ghostty | 195.6 ms | 152.0 MiB | 93.4 MiB | 283.4 ms | 115.0 ms (41.1%) |

Reliabilitty is fastest to launch, draws text fastest, and uses the least CPU.
st wins RSS and PSS because it is a smaller C process. xfce4-terminal is close
to Alacritty on draw time and CPU, but uses less memory and starts slightly
faster. Reliabilitty and st are far lighter than the GPU terminals here.

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
