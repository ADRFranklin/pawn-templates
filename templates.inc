// built-in include guard removal
// just in case the user has a local dependency with the same file name
#if defined _inc_templates
	#undef _inc_templates
#endif
// custom include-guard to ensure we don't duplicate
#if defined _templates_included
	#endinput
#endif
#define _templates_included

enum {
    TYPE_STR = 1,
    TYPE_INT,
    TYPE_FLOAT
}

#define PAIR_STR(%0,%1) TYPE_STR, %0, %1
#define PAIR_INT(%0,%1) TYPE_INT, %0, %1
#define PAIR_FLOAT(%0,%1) TYPE_FLOAT, %0, _:%1

native Template:CreateTemplate(const template[]);
native RenderTemplate(Template:id, dest[], len, ...);
native MakeTemplateVarInt(const namespace[], const key[], value);
native MakeTemplateVarFloat(const namespace[], const key[], Float:value);
native MakeTemplateVarString(const namespace[], const key[], const value[]);
native Template:LoadTemplateFromFile(const path[]);