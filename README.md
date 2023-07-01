# pa-status
___
It's a small program I wrote in Rust to display default pulse-audio sink and source status in my slstatus.
___
## Installation
1. Make sure u have pactl installed.
2. Run some commands
```
$ git clone https://github.com/Wasymir/pa-status.git
$ cd pa_status
$ cargo install --path .
```
3. Add `{ run_command, "%s", "pa-status" },` to `args` in your slstatus `config.h`.
___
## Want some features?
I've made this program for my personal use and it's not very advanced. If u want me to implement some additional features, just open an issue and describe them there. I'll implement them as soon as I have some free time.
