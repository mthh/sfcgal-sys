#include "wrapper.h"

thread_local char sfcgal_last_warn[ERRLEN];
thread_local char sfcgal_last_err[ERRLEN];


int warning_wrapper(const char* format, ...) {
    va_list ap;
    va_start(ap, format);
    vsnprintf(sfcgal_last_warn, (size_t) ERRLEN, format, ap);
    va_end(ap);
    return 0;
}

int error_wrapper(const char* format, ...) {
    va_list ap;
    va_start(ap, format);
    vsnprintf(sfcgal_last_err, (size_t) ERRLEN, format, ap);
    va_end(ap);
    return 0;
}

int w_sfcgal_init_handlers() {
    sfcgal_set_error_handlers(warning_wrapper, error_wrapper);
    return 0;
}

char *w_sfcgal_get_last_error(void) {
    return sfcgal_last_err;
}

char *w_sfcgal_get_last_warning(void) {
    return sfcgal_last_warn;
}
