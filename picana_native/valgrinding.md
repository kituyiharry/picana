Remember to use tcmalloc when checking for memory leaks with Valgrind!

Like so valgrind --tool=memcheck --soname-synonyms=somalloc=_tcmalloc_

Also a brk segment overflow may be reported in which case -> https://stackoverflow.com/questions/35129135/valgrind-reporting-a-segment-overflow
