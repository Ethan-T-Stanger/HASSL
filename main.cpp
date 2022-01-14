#include "HASSL.h"

//Return true if the program was initialized via a file
bool getFileInit(const int cmdargs) { return cmdargs > 1; }

int main(int argc, char** argv)
{
	if (getFileInit(argc))
	{
		HASSL::VirtualMachine vm(argv[1]);

		while (vm.getIsRunning())
			vm.run();

		if (vm.getExitCode() != 0 || DEBUG)
		{
			std::cout << "\nEXIT CODE " << HASSL::errorMessages[vm.getExitCode()];
			std::string unuStr;
			std::cerr << "\nEnter a newline to close the console: ";
			std::getline(std::cin, unuStr);
		}
	}
	else if (DEBUG)
	{
		std::string unuStr;
		std::cerr << "The HASSLVM must be invoked by a file!\nEnter a newline to close the console: ";
		std::getline(std::cin, unuStr);
	}

	return EXIT_SUCCESS;
}