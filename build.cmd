g++.exe -Wall -fexceptions -g -c node-run.cpp -o node-run.o
windres.exe -J rc -O coff -i RESOUR~1.RC -o resources.res
g++.exe -o node-run.exe node-run.o resources.res