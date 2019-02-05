#include <SFCGAL/capi/sfcgal_c.h>
#include <stdlib.h>
#include <stdio.h>
#include <stdarg.h>
// char* printf_wrapper(const char* format, ...) {
//     va_list ap;
//     int size = vsnprintf(0, 0, format, ap);
//     char* out = malloc(size + 1);
//     vsprintf(out, format, ap);
//     va_end(ap);
//     return out;
// }
#define ERRLEN 256

int warning_wrapper(const char* format, ...);
int error_wrapper(const char* format, ...);
extern int sfcgal_init_handlers();
char *sfcgal_get_last_error(void);
char *sfcgal_get_last_warning(void);
