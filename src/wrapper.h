#include <SFCGAL/capi/sfcgal_c.h>
#include <stdlib.h>
#include <stdio.h>
#include <stdarg.h>

#define ERRLEN 512

int warning_wrapper(const char* format, ...);
int error_wrapper(const char* format, ...);
extern int w_sfcgal_init_handlers();
char *w_sfcgal_get_last_error(void);
char *w_sfcgal_get_last_warning(void);
