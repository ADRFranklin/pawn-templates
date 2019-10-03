#define RUN_TESTS

#include <a_samp>
#include <YSI\y_utils>
#include <YSI\y_testing>

#include "templates.inc"

main() {
    //
}

Test:Simple() {
    new Template:t = Template_Create("Hello, {{ name }}! Today is {{ date }}");
    new rendered[64];
    new ret = Template_Render(t, rendered, sizeof rendered,
        PAIR_STR("name", "Southclaws"),
        PAIR_STR("date", "Monday")
    );

    printf("ret: %d rendered: '%s'", ret, rendered);
    ASSERT(ret == 0);
    ASSERT(strcmp(rendered, "Hello, Southclaws! Today is Monday") == 0);
}

Test:Types() {
    new Template:t = Template_Create("String: {{ string }} Int: {{ int }} Float: {{ float }}");
    new rendered[64];
    new ret = Template_Render(t, rendered, sizeof rendered,
        PAIR_STR("string", "hello"),
        PAIR_INT("int", 42),
        PAIR_FLOAT("float", 5.5)
    );

    printf("ret: %d rendered: '%s'", ret, rendered);
    ASSERT(ret == 0);
    ASSERT(strcmp(rendered, "String: hello Int: 42 Float: 5.5") == 0);
}

Test:Conditionals() {
    new Template:t = Template_Create("Hello {% if name %}{{ name }}{% else %}Anonymous{% endif %}.");
    new rendered[64];
    new ret = Template_Render(t, rendered, sizeof rendered,
        PAIR_STR("name", "Southclaws")
    );

    printf("ret: %d rendered: '%s'", ret, rendered);
    ASSERT(ret == 0);
    ASSERT(strcmp(rendered, "Hello Southclaws.") == 0);

    // no variables passed here
    ret = Template_Render(t, rendered, sizeof rendered);

    printf("ret: %d rendered: '%s'", ret, rendered);
    ASSERT(ret == 0);
    ASSERT(strcmp(rendered, "Hello Anonymous.") == 0);
}

Test:Filters() {
    new Template:t = Template_Create("{{ name | upcase }}");
    new rendered[64];
    new ret = Template_Render(t, rendered, sizeof rendered,
        PAIR_STR("name", "Southclaws")
    );

    printf("ret: %d rendered: '%s'", ret, rendered);
    ASSERT(ret == 0);
    ASSERT(strcmp(rendered, "SOUTHCLAWS") == 0);
}

Test:Assignment() {
    new Template:t = Template_Create("\
    {% assign fruits = \"apples, oranges, peaches\" %}\
    {% if fruits %}\
    {{ fruits }}\
    {% endif %}");
    new rendered[64];
    new ret = Template_Render(t, rendered, sizeof rendered,
        PAIR_STR("name", "Southclaws")
    );

    printf("ret: %d rendered: '%s'", ret, rendered);
    ASSERT(ret == 0);
    ASSERT(strcmp(rendered, "apples, oranges, peaches") == 0);
}

Test:GlobalVariables() {
    Template_SetGlobalString("player", "name", "Southclaws");
    Template_SetGlobalInt("player", "id", 3720);
    Template_SetGlobalFloat("player", "pos_x", 5.5);
    new Template:t = Template_Create("Name: {{ player.name }}, ID: {{ player.id }}, Pos X: {{ player.pos_x }}");
    new rendered[64];
    new ret = Template_Render(t, rendered, sizeof rendered);

    printf("ret: %d rendered: '%s'", ret, rendered);
    ASSERT(ret == 0);
    ASSERT(strcmp(rendered, "Name: Southclaws, ID: 3720, Pos X: 5.5") == 0);    
}

Test:TemplateVariabes() {
    new Template:t = Template_Create("Location: {{ location }}, Geo Id: {{ geoid }}, Lat: {{ lat }}");
    Template_SetString(t, "location", "England");
    Template_SetInt(t, "geoid", 37);
    Template_SetFloat(t, "lat", 9893.2);

    new rendered[64];
    new ret = Template_Render(t, rendered, sizeof rendered);    

    printf("ret: %d rendered: '%s'", ret, rendered);
    ASSERT(ret == 0);
    ASSERT(strcmp(rendered, "Location: England, Geo Id: 37, Lat: 9893.2001953125") == 0);      
}

Test:LoadFromFile() {
    Template_SetGlobalString("system", "name", "Machine");
    Template_SetGlobalInt("system", "id", 7780);
    Template_SetGlobalFloat("system", "coord_x", 9.5);    
    new Template:t = Template_LoadFromFile("scriptfiles/file.txt");
    new rendered[64];
    new ret = Template_Render(t, rendered, sizeof rendered);    

    printf("ret: %d rendered: '%s'", ret, rendered);
    ASSERT(ret == 0);
    ASSERT(strcmp(rendered, "Name: Machine, ID: 7780, Pos X: 9.5") == 0);    
}