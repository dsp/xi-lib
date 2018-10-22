SOURCE = init.c
CRATEROOT = ../

ifeq ($(OS),Windows_NT)
    CC = cl
    STDLIBS = shell32.lib ws2_32.lib userenv.lib advapi32.lib
    CCFLAGS = /I$(CRATEROOT)headers -o -o -o -o
    LDFLAGS_STATIC = $(CRATEROOT)target\debug\xilib.lib
    LDFLAGS_DYNAMIC = $(CRATEROOT)target\debug\xilib.dll.lib
    STATIC = init-static.exe
    DYNAMIC = init-dynamic.exe
else
    CC = cc
    STDLIBS = 
    CCFLAGS = -std=c99 -I$(CRATEROOT)headers -o
    LDFLAGS_STATIC = $(CRATEROOT)target/debug/xilib.a
    LDFLAGS_DYNAMIC = $(CRATEROOT)target/debug/xilib.dynlib
    STATIC = init-static
    DYNAMIC = init-dynamic
endif

$(STATIC) :
	$(CC) $(CCFLAGS) $@ $(SOURCE) $(STDLIBS) $(LDFLAGS)

$(DYNAMIC) :
	$(CC) $(CCFLAGS) $@ $(SOURCE) $(LDFLAGS)
