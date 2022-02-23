# Timer

This is a simple timer written in rust

It takes command-line arguments to handle, and was made simply for training-purposes

## Instructions

Download and compile the program, and put it in any folder available in the path. A config-folder will be generated for you
and you must there specify a sound-file to use.

### Flags

The -t flag specifies a timer, and takes arguments in the form of "number" with either s, m or h after, to indicate
seconds, minutes or hours.

the -a flag indicates an alarm, and takes a 24h time.

To these can be added m or s flag, with m indicateing a message to be added to the end of the arguments, or s to show
the time left, until the timer rings.

The -m flag can alternatively be added next to last, to indicate that the next argument is a message to be displayed when the 
timer rings.

### Regarding the s flag

At the moment, it adds about .000052/seconds, which over 24h adds up to around 4 minutes.
Work is being done to lower this, but keep it in mind.
