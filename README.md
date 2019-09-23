# Template Engine for Pawn

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
