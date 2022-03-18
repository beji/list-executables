# list-executables

Fetches $PATH from the environment, visits each directory (if it exists) and collects all executables.
The list will be sorted with duplicates removed and the result is written to STDOUT.

This is mainly useful to build an application launcher for something like [i3](https://i3wm.org/) by combining this with [fzf](https://github.com/junegunn/fzf) to filter down the list.

Most people will want something like [dmenu](https://tools.suckless.org/dmenu/) or [rofi](https://github.com/davatorium/rofi) as they will
most likely do a much better job than this.

## Example i3 config

Assuming gnome-terminal as the terminal emulator, i3 as the window manager and fzf, a basic config may look like this:

```
bindsym $mod+d exec --no-startup-id "gnome-terminal --title 'launcher' -- bash -c 'list-executables 2>/dev/null | fzf --reverse | xargs -r i3-msg -t command exec'"
for_window [title="^launcher$"] floating enable, border none
```

This will spawn a floating window listing all applications that can be filtered down by typing, the resulting command will then be executed. Note that this will only really do something visible when the target is a graphical application.

## How to build

```sh
make
```

## How to install

```sh
sudo make install
```
