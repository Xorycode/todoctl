# todoctl
A minimalist todo.txt manager written in Rust.
## Instructions
### Dependencies
Literally just rust.

### Step-by-step
Debian:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
git clone git@github.com:Xorycode/todoctl
cd todoctl
cargo build --release
sudo mv target/release/todoctl /usr/bin/
```
Arch:
```
sudo pacman -S rustup
rustup default stable
git clone git@github.com:Xorycode/todoctl
cd todoctl
cargo build --release
sudo mv target/release/todoctl /usr/bin/
```

# Motivation
Arguably the silliest reason I've written a project for.
A religious fanatic told me that Rust is the language of the anti-christ, despite the fact that the church has released no official interpretation of the book of Apocalypse. As a condemnation of fanaticism and extremism in all religions, I have decided to write this program.
