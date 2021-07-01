#pragma once

struct libant_setting {
	char* name;
	char* value;
};

static const struct libant_setting default_settings[] = {
	{ "linenumbers", "true" },
	{ "relativelinenumbers", "false" },
	{ "wrap", "true" },
	{ "statusbar", "Ant: V0.01" },
};
