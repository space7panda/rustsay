i've been playing around with VIM and CLI. SO one day when i was goofing with command like:

```bash
fortune | cowsay | lolcat
```
Why dont i use it as motivation to learn Vim ad Rust?
So i did

## Installation

### Cargo

```bash
cargo install rustsay

rustsay "Hello World"
```
### Brew
```bash
# Add remote Tap
brew tap space7panda/rustsay https://github.com/space7panda/rustsay
# Installation might take some time if you dont have Rust dependencies in brew
brew install rustsay
```

### From source
```bash
git clone https://github.com/space7panda/rustsay.git

cargo install --path .

rustsay --version
```

## Update

### Cargo
```bash
cargo install rustsay --force

rustsay --version
```

### Brew 
```bash
brew upgrade rustsay

rustsay --version
```

## ToDo list
- make it work lol
- add fortune feature
- add gradient feature
- add another characters?
- add fucntion to add it as terminal boot command? 🤔

## Speedtest cuz why not?

### cowsay
```bash
time cowsay "Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum."
```

```
 _________________________________________
/ Lorem Ipsum is simply dummy text of the \
| printing and typesetting industry.      |
| Lorem Ipsum has been the industry's     |
| standard dummy text ever since the      |
| 1500s, when an unknown printer took a   |
| galley of type and scrambled it to make |
| a type specimen book. It has survived   |
| not only five centuries, but also the   |
| leap into electronic typesetting,       |
| remaining essentially unchanged. It was |
| popularised in the 1960s with the       |
| release of Letraset sheets containing   |
| Lorem Ipsum passages, and more recently |
| with desktop publishing software like   |
| Aldus PageMaker including versions of   |
\ Lorem Ipsum.                            /
 -----------------------------------------
        \   ^__^
         \  (oo)\_______
            (__)\       )\/\
                ||----w |
                ||     ||
cowsay   0.01s user 0.00s system 83% cpu 0.015 total
```
### rustsay
```bash
time rustsay "Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum."
```

```
 _________________________________________
/ Lorem Ipsum is simply dummy text of     \
| the printing and typesetting industry.  |
| Lorem Ipsum has been the industry's     |
| standard dummy text ever since the      |
| 1500s, when an unknown printer took a   |
| galley of type and scrambled it to make |
| a type specimen book. It has survived   |
| not only five centuries, but also       |
| the leap into electronic typesetting,   |
| remaining essentially unchanged. It     |
| was popularised in the 1960s with the   |
| release of Letraset sheets containing   |
| Lorem Ipsum passages, and more recently |
| with desktop publishing software like   |
| Aldus PageMaker including versions of   |
\ Lorem Ipsum.                            /
 -----------------------------------------
     \   ^__^
      \  (oo)\_______
         (__)\       )\/\
             ||----w |
             ||     ||


rustsay 0.00s user 0.00s system 64% cpu 0.003 total
```

cowsay    0.01s user 0.00s system 83% cpu 0.015 total

rustsay   0.00s user 0.00s system 64% cpu 0.003 total
