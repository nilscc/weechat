add_library(weechat_rust STATIC weechat-rust.c)

install(TARGETS weechat_rust DESTINATION .)