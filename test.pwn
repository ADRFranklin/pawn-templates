#define RUN_TESTS

#include <a_samp>
#include <YSI\y_utils>
#include <YSI\y_testing>

#include "templates.inc"

main() {
    //
}

Test:Simple() {
    new Template:t = CreateTemplate("Hello, {{ name }}! Today is {{ date }}");
    new rendered[64];
    new ret = RenderTemplate(t, rendered, sizeof rendered,
        PAIR_STR("name", "Southclaws"),
        PAIR_STR("date", "Monday")
    );

    printf("ret: %d rendered: '%s'", ret, rendered);
    ASSERT(ret == 0);
    ASSERT(strcmp(rendered, "Hello, Southclaws! Today is Monday") == 0);
}

Test:Types() {
    new Template:t = CreateTemplate("String: {{ string }} Int: {{ int }} Float: {{ float }}");
    new rendered[64];
    new ret = RenderTemplate(t, rendered, sizeof rendered,
        PAIR_STR("string", "hello"),
        PAIR_INT("int", 42),
        PAIR_FLOAT("float", 5.5)
    );

    printf("ret: %d rendered: '%s'", ret, rendered);
    ASSERT(ret == 0);
    ASSERT(strcmp(rendered, "String: hello Int: 42 Float: 5.5") == 0);
}

Test:Conditionals() {
    new Template:t = CreateTemplate("Hello {% if name %}{{ name }}{% else %}Anonymous{% endif %}.");
    new rendered[64];
    new ret = RenderTemplate(t, rendered, sizeof rendered,
        PAIR_STR("name", "Southclaws")
    );

    printf("ret: %d rendered: '%s'", ret, rendered);
    ASSERT(ret == 0);
    ASSERT(strcmp(rendered, "Hello Southclaws.") == 0);

    // no variables passed here
    ret = RenderTemplate(t, rendered, sizeof rendered);

    printf("ret: %d rendered: '%s'", ret, rendered);
    ASSERT(ret == 0);
    ASSERT(strcmp(rendered, "Hello Anonymous.") == 0);
}

Test:Filters() {
    new Template:t = CreateTemplate("{{ name | upcase }}");
    new rendered[64];
    new ret = RenderTemplate(t, rendered, sizeof rendered,
        PAIR_STR("name", "Southclaws")
    );

    printf("ret: %d rendered: '%s'", ret, rendered);
    ASSERT(ret == 0);
    ASSERT(strcmp(rendered, "SOUTHCLAWS") == 0);
}

Test:Assignment() {
    new Template:t = CreateTemplate("\
    {% assign fruits = \"apples, oranges, peaches\" %}\
    {% if fruits %}\
    {{ fruits }}\
    {% endif %}");
    new rendered[64];
    new ret = RenderTemplate(t, rendered, sizeof rendered,
        PAIR_STR("name", "Southclaws")
    );

    printf("ret: %d rendered: '%s'", ret, rendered);
    ASSERT(ret == 0);
    ASSERT(strcmp(rendered, "apples, oranges, peaches") == 0);
}

Test:GlobalVariables() {
    SetTemplateGlobalVarString("player", "name", "Southclaws");
    SetTemplateGlobalVarInt("player", "id", 3720);
    SetTemplateGlobalVarFloat("player", "pos_x", 5.5);
    new Template:t = CreateTemplate("Name: {{ player.name }}, ID: {{ player.id }}, Pos X: {{ player.pos_x }}");
    new rendered[64];
    new ret = RenderTemplate(t, rendered, sizeof rendered);

    printf("ret: %d rendered: '%s'", ret, rendered);
    ASSERT(ret == 0);
    ASSERT(strcmp(rendered, "Name: Southclaws, ID: 3720, Pos X: 5.5") == 0);    
}

Test:TemplateVariabes() {
    new Template:t = CreateTemplate("Location: {{ location }}, Geo Id: {{ geoid }}, Lat: {{ lat }}");
    SetTemplateVarString(t, "location", "England");
    SetTemplateVarInt(t, "geoid", 37);
    SetTemplateVarFloat(t, "lat", 9893.2);

    new rendered[64];
    new ret = RenderTemplate(t, rendered, sizeof rendered);    

    printf("ret: %d rendered: '%s'", ret, rendered);
    ASSERT(ret == 0);
    ASSERT(strcmp(rendered, "Location: England, Geo Id: 37, Lat: 9893.2001953125") == 0);      
}

Test:LoadFromFile() {
    SetTemplateGlobalVarString("system", "name", "Machine");
    SetTemplateGlobalVarInt("system", "id", 7780);
    SetTemplateGlobalVarFloat("system", "coord_x", 9.5);    
    new Template:t = LoadTemplateFromFile("scriptfiles/file.txt");
    new rendered[64];
    new ret = RenderTemplate(t, rendered, sizeof rendered);    

    printf("ret: %d rendered: '%s'", ret, rendered);
    ASSERT(ret == 0);
    ASSERT(strcmp(rendered, "Name: Machine, ID: 7780, Pos X: 9.5") == 0);    
}