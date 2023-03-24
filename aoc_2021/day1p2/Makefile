CC = g++
OBJS = Utilities.o day1p2.o
EXECUTABLE = day1p2
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
