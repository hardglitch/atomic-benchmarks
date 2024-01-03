# Rust Atomics (Intel x64, SecCst) vs Other rust sync primitives:
```
atomic_bench1   ~1200ms (DEV) - ~1130ms (REL)
atomic_bench2   ~7700ms (DEV) - ~5500ms (REL)
channel_bench    ~125ms (DEV) -    ~1ms (REL)
normal_bench     ~600ms (DEV) -  ~0.6ms (REL)
```
