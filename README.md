# pawn-template

[![sampctl](https://shields.southcla.ws/badge/sampctl-pawn--templates-2f2f2f.svg?style=for-the-badge)](https://github.com/ADRFranklin/pawn-templates)

## Description

Creates customizable templates, that can be rendered with custom variables.

## Installation

Simply install to your project:

```bash
sampctl package install adrfranklin/pawn-templates
```

Include in your code and begin using the library:

```pawn
#include <templates>
```

## Usage

```c
static Template:ban_template;

main() {
    SetTemplateGlobalVarString("server", "name", "Example");
}

static RenderBanTemplate(playerid, const reason[])
{
    new name[MAX_PLAYER_NAME + 1];
    GetPlayerName(playerid, name, sizeof name);

    ban_template = CreateTemplate(
        "You have currently been banned from {{ server.name }}. \
        \
        Name: {{ name | capitalize }} \
        Date: {{ date | date: "%Y %h" }} \
        Admin {{ admin_name | capitalize }} \
        Reason: {{ reason }}"
    );

    SetTemplateVarString(ban_template, "name", "Michael");
    SetTemplateVarInt(ban_template, "date", gettime());
    SetTemplateVarString(ban_template, "admin_name", "Southclaws");
    SetTemplateVarString(ban_template, "reason", reason);

    new output[1024];
    RenderTemplate(ban_template, output, sizeof output);
}
```

You can find more about the general syntax here: [link](https://github.com/Shopify/liquid/wiki/Liquid-for-Designers)

## Testing

To test, simply run the package:

```bash
make test-native
```

or

```bash
task test-native
```
