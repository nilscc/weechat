#include "../weechat-plugin.h"

WEECHAT_PLUGIN_NAME("weechat-rust");
WEECHAT_PLUGIN_DESCRIPTION(N_("Weechat rust plugin"));
WEECHAT_PLUGIN_AUTHOR("Nils <mail@nils.cc>");
WEECHAT_PLUGIN_VERSION("0.1");
WEECHAT_PLUGIN_LICENSE("MIT");
WEECHAT_PLUGIN_PRIORITY(1000);

extern const char *c_test();

const char *c_test() { return "test"; }