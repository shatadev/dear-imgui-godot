#include <stdarg.h>
#include <stdio.h>

/* Defined in Rust (backend.rs); receives the fully-formatted recovery message. */
extern void dimgui_recover_report(const char *msg);

// Matches Dear ImGui's ImGuiErrorLogCallback. Formatting printf-style varargs in C keeps this
// portable across all targets (including web), avoiding Rust's unstable c_variadic.
// Passed to igErrorCheckEndFrameRecover(); invoked once per structural mistake.
void dimgui_recover_log(void *user_data, const char *fmt, ...) {
    (void)user_data;
    char buf[512];
    va_list args;
    va_start(args, fmt);
    vsnprintf(buf, sizeof(buf), fmt, args);
    va_end(args);
    dimgui_recover_report(buf);
}
