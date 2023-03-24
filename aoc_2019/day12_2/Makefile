CC = g++
OBJS = Utilities.o day12_2.o
EXECUTABLE = day12_2
CFLAGS = -Wall -g -std=c++17
LFLAGS = 
LIBS=boost_program_options

all:    $(EXECUTABLE)

$(EXECUTABLE):  $(OBJS)
	$(CC) $(LFLAGS) -o $(EXECUTABLE) $(OBJS) -l$(LIBS)

%.o:    %.cpp
	$(CC) $(CFLAGS) -c -o $@ $<

clean:
	-rm -f $(OBJS) $(EXECUTABLE)
