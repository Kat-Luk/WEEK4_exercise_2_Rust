Task 2

Make a program that, based on the given command line parameters, either reads a specific file or writes to a specific file. The parameters should include the name of the file, the operations to be performed (write/read) and possibly another file with the content to be written to the first file. Name the functions write_in_file and read_file. They take 2 and 1 parameters respectively.

Note that the writeln! command creates an extra line in the written file. For example, the write! command allows the user's input to be written to the file as it is.

Note that the program must be run with, for example, the cargo run command instead of the Run button of the rust-analyzer tool, because your program uses command line arguments.


Example run with command line parameters:

cargo run uusi_tiedosto.txt write "testi lisäys"

 


 

Example run read:

cargo run uusi_tiedosto.txt read

The contents of the file:

 

testi lisäys

 

Example run without commands:

cargo run

No arguments were given.

 
