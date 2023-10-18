# Rust developer tools

This project implements a small suite of developer tools in Rust. The goals here were to develop some tools that I personally needed, and learn Rust in the process :)

### Running

```bash
# build
cargo build --release

# run commands
./target/release/rt command_name --options
```

### Tools

The project currently implements five tools:

1. **Random password generator.** Generates a password of specified length, containing lower- and upper-case letters, numbers, and special symbols, and automatically copies it to your clipboard.
   ```bash
   rt password --len 16
   # o87Hf374N-gm8!Gh
   ```

2. **Disk space checker.** Checks how much free disk space there is and prints the output clearly. Much simpler usage compared to other commands.
   ```bash
   rt disk
   # 89.77 GB free
   ```

4. **Internet speed checker.** Checks download and upload speed by pinging a publicly available internet speed server.
   ```bash
   rt internet
   # download: 184.3 Mb/s
   # upload: 76.1 Mb/s
   ```

6. **Screenshare hiding utility.** Hides all files on desktop while screensharing (and show them again later).
   ```bash
   rt hide
   # hides all desktop files and folders into a temporary folder in documents
   rt show
   # puts all desktop files back and deletes the temporary documents folder
   ```

8. **Dinosaur game.** Helps developers pass the time during work breaks :)
   ```bash
   rt dino --len 16
   # see for yourself :)
   ```
### Reflections on Rust

I enjoyed working with Rust. A lot of small mistakes are caught during compile time. Learned about borrowing, traits, and cloning vs copying.
