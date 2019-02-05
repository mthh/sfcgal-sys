#include <SFCGAL/capi/sfcgal_c.h>
#include <stdlib.h>
#include <stdio.h>
#include <stdarg.h>

#define ERRLEN 256

int warning_wrapper(const char* format, ...);
int error_wrapper(const char* format, ...);
extern int sfcgal_init_handlers();
char *sfcgal_get_last_error(void);
char *sfcgal_get_last_warning(void);
