#include <SFCGAL/capi/sfcgal_c.h>
#include <stdlib.h>
#include <stdio.h>
#include <stdarg.h>
#include "wrapper.h"

char sfcgal_last_warn[ERRLEN];
char sfcgal_last_err[ERRLEN];


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

int sfcgal_init_handlers() {
    sfcgal_set_error_handlers(warning_wrapper, error_wrapper);
    return 0;
}

char *sfcgal_get_last_error(void) {
    return sfcgal_last_err;
}

char *sfcgal_get_last_warning(void) {
    return sfcgal_last_warn;
}
