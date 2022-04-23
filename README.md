# termbg-cli

This is a cli version of https://github.com/dalance/termbg

It prints "light" or "dark" depending on your terminal background.

Useful to autoconfigure tools depending on your theme.

### Git Delta:

```
__termbg_preexec_function () {
    export DELTA_FEATURES="$(termbg)-mode"
}

typeset -ag preexec_functions;
preexec_functions=( __termbg_preexec_function ${preexec_functions[@]} )
```

And create two different configurations in your ~/.config/git/config

```
[delta "light-mode"]
    light = true

[delta "dark-mode"]
    dark = true
    file-style = green # or whatever other settings you want; this is just an example
```

