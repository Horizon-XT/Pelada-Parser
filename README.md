# Pelada-Parser

this project exist for 2 reasons:
- Learn Rust
- Detach the parsing from the [PeladaAPI](https://github.com/Horizon-XT/PeladApi) project.

##  Install

### With Zsh
1. Clone the repo.
2. Run `install.sh`

### Without Zsh
1. Clone the repo
2. Run `cargo build --release`
3. Add the binary to your path

## Usage

Start as CLI:
```bash
cargo run -- --cli <filepath>
```
### Sample of an expected pattern to parse
```text
⚽PELADA dia 30/10/23 as 19:30hs no LOCAL🥅

Goleiros:
1.
2.
3.

Linha:
1.
2.
3.
4.
5.
6.
7.
8.
9. 
10.
11.
12.
13.
14.
15.

Suplentes/Convidados:
1.
2.
3.

Sub 15:
1.
2.
3.
4.
5.

```

Check `samples` to see more.
