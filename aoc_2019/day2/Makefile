CC = g++
OBJS = Utilities.o day2.o
EXECUTABLE = day2
CFLAGS = -Wall -s -O3 -std=c++17
LFLAGS = -s
LIBS=boost_program_options

all:    $(EXECUTABLE)

$(EXECUTABLE):  $(OBJS)
	$(CC) $(LFLAGS) -o $(EXECUTABLE) $(OBJS) -l$(LIBS)

%.o:    %.cpp
	$(CC) $(CFLAGS) -c -o $@ $<

clean:
	-rm -f $(OBJS) $(EXECUTABLE)
