#include <string>
#include <iostream>
#include <chrono>
#include <ctime>
#include <stdlib.h>

using namespace std;
using std::chrono::duration_cast;
using std::chrono::milliseconds;
using std::chrono::system_clock;

int main(int argc, char *argv[]) {
	string args = "node", title = "title node ";
	if (argc == 1) {
		return 0;
	}

	for (int i = 1; i < argc; i++) {
		args = args + " " + argv[i];
	}

	title += argv[1];
	system(title.c_str());
	system("clear");
	cout << "\x1b[0;34m[Running]" << "\x1b[96m node " << argv[1] << "\x1b[0m" << endl << endl;

	auto start_time = duration_cast<milliseconds>(system_clock::now().time_since_epoch()).count();
    int exit_code = system(args.c_str());
	auto end_time = duration_cast<milliseconds>(system_clock::now().time_since_epoch()).count();

	int total_milliseconds = end_time - start_time;
	int ms = total_milliseconds % 1000;
	int total_seconds = (total_milliseconds - ms) / 1000;
	int ss = total_seconds % 60;
	int total_minutes = (total_seconds - ss) / 60;
	int mm = total_minutes % 60;
	int hh = (total_minutes - mm) / 60;
	
	string execution_time = hh > 0 ? (to_string(hh) + "h:" + to_string(mm) + "m:" + to_string(ss))
		: mm > 0 ? (to_string(mm) + "m:" + to_string(ss))
			: to_string(ss);
	string timelevel = hh > 0 ? "hours" : mm > 0 ? "minutes" : "seconds";

	string exitlevel = exit_code > 0 ? "\x1b[0;31m[Failed]" : "\x1b[0;32m[Done]";
	cout << endl << exitlevel << "\x1b[96m exited with\x1b[95m code=" << exit_code << "\x1b[96m in \x1b[94m" << execution_time << "." << ms << "s \x1b[96m" << timelevel << "\x1b[0m" << endl;
	system("if errorlevel 0 pause");

	return 0;
}
