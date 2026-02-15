# zellij-nvim-nav-plugin

This Zellij plugin is intended to be used in conjunction with
[zellij-nav.nvim](https://github.com/swaits/zellij-nav.nvim) as an
alternative to [zellij-autolock](https://github.com/fresh2dev/zellij-autolock)
(which served as a valuable reference during the development of this plugin).

The theory of operation is simple: Ctrl-{hjkl} (or whatever keys you want to
use) are bound to the "`MessagePlugin`" action, which send messages which this
plugin listens for. If the currently selected pane is running neovim or vim,
the plugin sends a specified key code to the pane, otherwise it moves focus
in the specified direction.

The plugin supports both single-byte and multi-byte key sequences. Single-byte
sequences are used for control key combinations (e.g., Ctrl+h = 8), while
multi-byte sequences are used for Alt key combinations (e.g., Alt+h = ESC h = 27,104).
The payload field accepts comma-separated byte values. You can refer to [ASCII
table](https://www.asciitable.com/) to find the byte values for the keys you
want to use.

## Configuration Options

The plugin supports the following configuration option:

- `match_commands`: Comma-separated list of commands to match (default: `vim,nvim`)

## Example Configuration

```plain
plugins {
    ...
    nvim-nav location="file:/home/youruser/some_dir/zellij-nvim-nav-plugin.wasm"
}
```

```plain
load_plugins {
    nvim-nav
}
```
 
```plain
    shared_except "scroll" {
        bind "Alt left" { MoveFocusOrTab "left"; }
        bind "Alt down" { MoveFocus "down"; }
        bind "Alt up" { MoveFocus "up"; }
        bind "Alt right" { MoveFocusOrTab "right"; }

        // Single-byte payloads for Ctrl key combinations
        bind "Ctrl h" { MessagePlugin { name "nvim_nav_left"; payload "8"; }; }
        bind "Ctrl j" { MessagePlugin { name "nvim_nav_down"; payload "10"; }; }
        bind "Ctrl k" { MessagePlugin { name "nvim_nav_up"; payload "11"; }; }
        bind "Ctrl l" { MessagePlugin { name "nvim_nav_right"; payload "12"; }; }

        // Use the _tab variants to navigate between panes and switch tabs when at the edge
        bind "Ctrl h" { MessagePlugin { name "nvim_nav_left_tab"; payload "8"; }; }
        bind "Ctrl l" { MessagePlugin { name "nvim_nav_right_tab"; payload "12"; }; }

        // Multi-byte payloads for Alt key combinations (ESC + key)
        bind "Alt h" { MessagePlugin { name "nvim_nav_left"; payload "27,104"; }; }
        bind "Alt j" { MessagePlugin { name "nvim_nav_down"; payload "27,106"; }; }
        bind "Alt k" { MessagePlugin { name "nvim_nav_up"; payload "27,107"; }; }
        bind "Alt l" { MessagePlugin { name "nvim_nav_right"; payload "27,108"; }; }
    }
```

Example with custom commands:

```plain
plugins {
    ...
    nvim-nav location="file:/home/youruser/some_dir/zellij-nvim-nav-plugin.wasm" {
        match_commands "vim,nvim,hx"
    }
}
```
