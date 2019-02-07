#include <SFCGAL/capi/sfcgal_c.h>
#include <stdlib.h>
#include <stdio.h>
#include <stdarg.h>

#ifndef thread_local
# if __STDC_VERSION__ >= 201112 && !defined __STDC_NO_THREADS__
#  define thread_local _Thread_local
# elif defined _WIN32 && ( \
       defined _MSC_VER || \
       defined __ICL || \
       defined __DMC__ || \
       defined __BORLANDC__ )
#  define thread_local __declspec(thread)
# elif defined __GNUC__ || \
       defined __SUNPRO_C || \
       defined __xlC__
#  define thread_local __thread
# else
#  error "Cannot define thread_local"
# endif
#endif

#define ERRLEN 512

int warning_wrapper(const char* format, ...);
int error_wrapper(const char* format, ...);
extern int w_sfcgal_init_handlers();
char *w_sfcgal_get_last_error(void);
char *w_sfcgal_get_last_warning(void);
