#pragma once

#include <sstream>
#include <cstdint>
#include <vector>
#include <string>

#include "definitions.h"
#include "Hexadecimal.h"

namespace HASSL
{
	class VirtualMachine
	{
	public:
		VirtualMachine(const char* filepath);

		//Runs the next frame of the program
		void run();
		
		bool getIsRunning() { return isRunning; }
		uint8_t getExitCode() { return exitCode.getHexValue(); }
	private:
		//Gets the next command in the source code string
		bool getCommand(char& retChar);
		//Run the given command in the virtual machine
		void runCommand(char command);
		//Pushes the current value to the stack
		void stackPush(uint8_t value);
		//Pops the stack
		void stackPop();
		//Sends the command pointer back to the currently selected state
		void goBackToState();
		//Loop through comment
		unsigned int loopThroughComment(const unsigned int initialValue, bool autoWrap = true);
		//Loop through a control path
		unsigned int loopThroughPath(unsigned int initialValue);
		//Loop through a reversed control path
		unsigned int loopThroughReversedPath(const unsigned int initialValue);

		//Rewrites a given file content string, or returns false on a file access fail
		bool getFileContents(const char* filepath, std::string& retStr);
		//Gets the positions (in characters) of any given state definitions
		bool getStateDefinitionPositions();

		unsigned int srcCodeIterator = NULL;
		std::string srcCode;

		std::vector<unsigned int> stateDefinitionPositions[16];
		Hex currentState;

		std::vector<uint8_t> stack;
		std::pair<Hex, Hex> accessibleMemory = {Hex(), Hex()};
		bool selectedLeft = false;

		Hex exitCode;
		bool isRunning;
	};
}