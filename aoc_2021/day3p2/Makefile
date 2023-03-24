CC = g++
EXECUTABLE = day3p2
OBJS = Utilities.o $(EXECUTABLE).o
CFLAGS = -Wall -O3 -std=c++17
LFLAGS = -s 
LIBS=-lboost_program_options -lfmt

all:    $(EXECUTABLE)

$(EXECUTABLE):  $(OBJS)
	$(CC) $(LFLAGS) -o $(EXECUTABLE) $(OBJS) $(LIBS)

%.o:    %.cpp
	$(CC) $(CFLAGS) -c -o $@ $<

clean:
	-rm -f $(OBJS) $(EXECUTABLE)
