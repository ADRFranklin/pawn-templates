## Template Engine for Pawn

# Basic Use
```c
TemplateBanned = CreateTemplate(
"Your account {{name}} has been banned!\

Reason: {{reason}}\
Duration: {{duration}}\
If you disagree, please file an appeal at: {{forum}}\
");

// ...

new dest[1024];
RenderTemplate(TemplateBanned, dest,
    PAIR_INT("name", playerName),
    PAIR_INT("reason", reason),
    PAIR_STR("forum", "https://forum.website.com")
);
```

# Full Usage
```c
static Template:JoinTemplate;

main() {
    CreateTemplateVar(
        PAIR_STR("primary", "{007BFF}"),
        PAIR_STR("white", "{FFFFFF}"),
        PAIR_STR("server_name", "My Server")
    );

    JoinTemplate = CreateTemplate("\
        Welcome {primary}{user}{white}, {primary}{server_name}{white}"
    );    
}

public OnPlayerConnect(playerid)
{
    new name[MAX_PLAYER_NAME + 1];
    GetPlayerName(playerid, name, sizeof name);

    new dest[1024];
    RenderTemplate(JoinTemplate, dest, sizeof dest, 
        PAIR_STR("user", name)
    );

    SendClientMessage(playerid, 0xFFFFFF, dest);

    return 1;
}
```
