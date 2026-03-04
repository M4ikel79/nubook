# 3rd Party Prompts

## Nerd Fonts

Not required but improves prompt with glyphs/icons. See nerdfonts.com.

## Oh My Posh

1. Install oh-my-posh and themes
2. Install nerd font
3. Generate config:
```nu
oh-my-posh init nu --config ~/.poshthemes/M365Princess.omp.json
```
4. Add to config.nu:
```nu
source ~/.oh-my-posh.nu
```

MacOS with brew:
```nu
let posh_dir = (brew --prefix oh-my-posh | str trim)
let posh_theme = $'($posh_dir)/share/oh-my-posh/themes/'
$env.PROMPT_COMMAND = { || oh-my-posh prompt print primary --config $'($posh_theme)/zash.omp.json' }
$env.PROMPT_INDICATOR = $"(ansi y)$> (ansi reset)"
```

## Starship

1. Install Starship
2. Install nerd fonts
3. Add to config:
```nu
$env.STARSHIP_SHELL = "nu"

def create_left_prompt [] {
    starship prompt --cmd-duration $env.CMD_DURATION_MS $'--status=($env.LAST_EXIT_CODE)'
}

$env.PROMPT_COMMAND = { || create_left_prompt }
$env.PROMPT_COMMAND_RIGHT = ""
$env.PROMPT_INDICATOR = ""
$env.PROMPT_INDICATOR_VI_INSERT = ": "
$env.PROMPT_INDICATOR_VI_NORMAL = "〉"
```

Or use official quick install (creates env.nu and config setup).

## Purs

See github.com/xcambar/purs
