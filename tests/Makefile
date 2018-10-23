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
    UNAME_S := $(shell uname -s)
    CC = cc
    STDLIBS =
    ifeq ($(UNAME_S),Darwin)
        STDLIBS = -framework CoreServices
    endif
    CCFLAGS = -std=c99 -I$(CRATEROOT)headers -o
    LDFLAGS = 
    LDFLAGS_STATIC = $(LDFLAGS) $(CRATEROOT)target/debug/libxilib.a
    LDFLAGS_DYNAMIC = $(LDFLAGS) $(CRATEROOT)target/debug/libxilib.dylib
    STATIC = init-static
    DYNAMIC = init-dynamic
endif

$(STATIC) :
	$(CC) $(CCFLAGS) $@ $(SOURCE) $(STDLIBS) $(LDFLAGS_STATIC)

$(DYNAMIC) :
	$(CC) $(CCFLAGS) $@ $(SOURCE) $(LDFLAGS_DYNAMIC)
:
clean:
	@rm $(STATIC) $(DYNAMIC)

.PHONY: clean
