CC = g++
OBJS = Utilities.o day7_2.o
EXECUTABLE = day7_2
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
