CC = cl
SOURCE = init.c
STDLIBS = shell32.lib ws2_32.lib userenv.lib advapi32.lib
CRATEROOT = ../
init-static.exe :
	$(CC) /I $(CRATEROOT)headers $(SOURCE) $(STDLIBS) $(CRATEROOT)target\debug\xilib.lib

init-dll.exe :
	$(CC) /I $(CRATEROOT)headers $(SOURCE) /oinit2.exe $(CRATEROOT)target\debug\xilib.dll.lib
