#include "VirtualMachine.h"

#include <iostream>
#include <fstream>
#include <algorithm>
#include <ctime>

#include "definitions.h"
#include "Hexadecimal.h"

HASSL::VirtualMachine::VirtualMachine(const char* filepath)
{
	isRunning = true;
	exitCode = EXIT_CODE_NULL;

	//Seed the random number generator
	std::srand(unsigned int(time(0)));

	if (TEST_PRINT_ERROR_CODES)
	{
		for (unsigned int i = 0; i < errorMessages.size(); i++)
			std::cout << errorMessages[i] << '\n';
	}

	if (!getFileContents(filepath, srcCode))
		exitCode = EXIT_CODE_ERROR;
	else
	{
		if (!getStateDefinitionPositions())
			exitCode = EXIT_CODE_NO_STATE;
		else
			srcCodeIterator = stateDefinitionPositions[currentState.getHexValue()][0];
	}

	if (exitCode != EXIT_CODE_NULL)
		isRunning = false;
}

void HASSL::VirtualMachine::run()
{
	if (FRAME_BY_FRAME)
	{
		std::string unuStr;
		std::cout << "Enter a newline to pass the time: ";
		std::getline(std::cin, unuStr);
	}

	if (PRINT_ACCESSIBLE_CONTENTS)
		std::cout << "The accessible memory registers currently contain: " << int(accessibleMemory.first.getHexValue()) << " & " << int(accessibleMemory.second.getHexValue()) << '\n';

	if (PRINT_STACK_CONTENTS)
	{
		std::cout << "The stack currently contains " << stack.size() << " values.\n";
		for (unsigned int i = 0; i < stack.size(); i++)
		{
			std::cout << "\tStack position " << i << ", int " << int(stack[i]) << ", char " << char(stack[i]) << ".\n";
		}
	}

	if (PRINT_COMMAND_POINTER_POSITION)
		std::cout << int(srcCodeIterator) << '\n';

	char command = ' ';
	if (getCommand(command))
	{
		runCommand(command);
		if (PRINT_COMMANDS)
		{
			switch (command)
			{
			case ' ':
				std::cout << "Command: ' '\n";
				break;
			case '\t':
				std::cout << "Command: '\\t'\n";
				break;
			case'\n':
				std::cout << "Command: '\\n'\n";
				break;
			default:
				std::cout << "Command: " << command << '\n';
			}
		}
	}
	else
		isRunning = false;

	if (exitCode != EXIT_CODE_NULL)
		isRunning = false;
}

bool HASSL::VirtualMachine::getCommand(char& retChar)
{
	if (srcCodeIterator < srcCode.length())
	{
		retChar = srcCode.at(srcCodeIterator);
		++srcCodeIterator;
		if (srcCodeIterator == srcCode.length())
			srcCodeIterator = 0;
	}
	else
		return false;

	return true;
}

void HASSL::VirtualMachine::runCommand(char command)
{
	Hex tempLeftHex;
	Hex tempRightHex;
	std::string tempLine;

	switch (command)
	{
	case '^':
		stackPop();
		break;
	case 'v':
		stackPush(accessibleMemory.first + accessibleMemory.second);
		break;
	case 's':
		if (stack.size() >= 2)
			std::iter_swap(stack.begin() + stack.size() - 1, stack.begin() + stack.size() - 2);
		else
			exitCode = EXIT_CODE_STACK_INSUFFICIENT_SIZE;
		break;
	case 'd':
		if (stack.size() >= 1 && stack.size() < 256)
			stack.push_back(stack[stack.size() - 1]);
		else if (stack.size() >= 1)
			exitCode = EXIT_CODE_STACK_INSUFFICIENT_SPACE;
		else
			exitCode = EXIT_CODE_STACK_INSUFFICIENT_SIZE;
		break;
	case 'i':
		std::reverse(stack.begin(), stack.end());
		break;
	case 'r':
		if (stack.size() >= 2)
		{
			uint8_t tempInt = stack[stack.size() - 1];
			stack.pop_back();
			stack.insert(stack.begin(), tempInt);
		}
		else
			exitCode = EXIT_CODE_STACK_INSUFFICIENT_SIZE;
		break;
	case '+':
		if (stack.size() >= 2 && stack.size() < 256)
		{
			stack.push_back(stack[stack.size() - 1] + stack[stack.size() - 2]);
		}
		else if (stack.size() >= 2)
			exitCode = EXIT_CODE_STACK_INSUFFICIENT_SPACE;
		else
			exitCode = EXIT_CODE_STACK_INSUFFICIENT_SIZE;
		break;
	case '-':
		if (stack.size() >= 2 && stack.size() < 256)
		{
			stack.push_back(stack[stack.size() - 2] - stack[stack.size() - 1]);
		}
		else if (stack.size() >= 2)
			exitCode = EXIT_CODE_STACK_INSUFFICIENT_SPACE;
		else
			exitCode = EXIT_CODE_STACK_INSUFFICIENT_SIZE;
		break;
	case '(':
		selectedLeft = true;
		break;
	case ')':
		selectedLeft = false;
		break;
	case '*':
		if (selectedLeft)
			++accessibleMemory.first;
		else
			++accessibleMemory.second;
		break;
	case '.':
		if (selectedLeft)
			accessibleMemory.first = 0;
		else
			accessibleMemory.second = 0;
		break;
	case '~':
		if (selectedLeft)
			accessibleMemory.first = std::rand() % 16;
		else
			accessibleMemory.second = std::rand() % 16;
		break;
	case '?':
		if (selectedLeft && accessibleMemory.first == HEX_0)
			goBackToState();
		else if (!selectedLeft && accessibleMemory.second == HEX_0)
			goBackToState();
		break;
	case '!':
		if (selectedLeft && accessibleMemory.first != HEX_0)
			goBackToState();
		else if (!selectedLeft && accessibleMemory.second != HEX_0)
			goBackToState();
		break;
	case 'p':
		std::cout << unsigned char(accessibleMemory.first + accessibleMemory.second);
		accessibleMemory.first = 0;
		accessibleMemory.second = 0;
		break;
	case 'n':
		std::cout << int(accessibleMemory.first + accessibleMemory.second);
		accessibleMemory.first = 0;
		accessibleMemory.second = 0;
		break;
	case 'g':
		std::getline(std::cin, tempLine);
		if (tempLine.length() > 128)
			tempLine.erase(tempLine.begin() + 128, tempLine.end());
		for (unsigned int i = 0; i < tempLine.length(); i++)
			stackPush(uint8_t(unsigned char(tempLine.at(i))));
		break;
	case '#':
		tempLeftHex.update(uint8_t(stack.size()), true);
		tempRightHex.update(uint8_t(stack.size()), false);
		accessibleMemory = { tempLeftHex, tempRightHex };
		break;
	case '&':
		goBackToState();
		break;
	case '@':
		if (selectedLeft)
			exitCode = accessibleMemory.first;
		else
			exitCode = accessibleMemory.second;
		isRunning = false;
		break;
	case '$':
		if (selectedLeft)
			currentState = accessibleMemory.first;
		else
			currentState = accessibleMemory.second;
		break;
	case '<':
		srcCodeIterator = loopThroughReversedPath(srcCodeIterator);
		break;
	case '[':
		srcCodeIterator = loopThroughPath(srcCodeIterator);
		break;
	case '|':
		srcCodeIterator = 0;
		break;
	case '%':
		srcCodeIterator = loopThroughComment(srcCodeIterator);
		break;
	case '0': case '1': case '2': case '3': case '4':
	case '5': case '6': case '7': case '8': case '9':
	case 'A': case 'B': case 'C': case 'D': case 'E':
	case 'F': case ']': case '>': case ':': case ' ':
	case '\t': case '\n':
		break;
	default:
		exitCode = EXIT_CODE_UNEXPECTED_TOKEN;
	}
}

void HASSL::VirtualMachine::stackPush(uint8_t value)
{
	if (stack.size() < 256)
	{
		stack.push_back(value);
		accessibleMemory.first = 0;
		accessibleMemory.second = 0;
	}
	else
		exitCode = EXIT_CODE_STACK_OVERFLOW;
}

void HASSL::VirtualMachine::stackPop()
{
	Hex tempLeftHex;
	Hex tempRightHex;

	if (stack.size() != 0)
	{
		tempLeftHex.update(stack[stack.size() - 1], true);
		tempRightHex.update(stack[stack.size() - 1], false);
		accessibleMemory = { tempLeftHex, tempRightHex };
		stack.pop_back();
	}
	else
		exitCode = EXIT_CODE_STACK_UNDERFLOW;
}

void HASSL::VirtualMachine::goBackToState()
{
	if (stateDefinitionPositions[currentState.getHexValue()].size() > 0)
	{
		for (unsigned int i = 0; i < stateDefinitionPositions[currentState.getHexValue()].size(); i++)
		{
			if (stateDefinitionPositions[currentState.getHexValue()][i] > srcCodeIterator)
			{
				srcCodeIterator = stateDefinitionPositions[currentState.getHexValue()][i];
				break;
			}
			else if (i + 1 == stateDefinitionPositions[currentState.getHexValue()].size())
			{
				srcCodeIterator = stateDefinitionPositions[currentState.getHexValue()][0];
				break;
			}
		}
	}
	else
		exitCode = EXIT_CODE_NO_STATE;
}

unsigned int HASSL::VirtualMachine::loopThroughComment(const unsigned int initialValue, bool autoWrap)
{
	unsigned int retInt = NULL;

	for (int i = initialValue; unsigned int(i) < srcCode.length(); i++)
	{
		if (PRINT_COMMENT_LOOP)
			std::cout << "Hit comment Loop!\n";

		if (srcCode.at(i) == '%')
		{
			if (i + 1 == srcCode.length() && autoWrap)
				retInt = 0;
			else
				retInt = i + 1;
			break;
		}

		if (i + 1 == srcCode.length())
			i = -1;
	}

	return retInt;
}

unsigned int HASSL::VirtualMachine::loopThroughPath(unsigned int initialValue)
{
	unsigned int retInt = NULL;
	unsigned int openingCount = NULL;

	if (initialValue == 0)
		initialValue = srcCode.length();

	for (int i = initialValue - 1; unsigned int(i) < srcCode.length(); i++)
	{
		if (PRINT_PATH_LOOP)
			std::cout << "Hit control path Loop!\n";

		if (srcCode.at(i) == '[')
			openingCount++;

		if (srcCode.at(i) == ']')
			openingCount--;

		if (openingCount == 0)
		{
			if (i + 1 == srcCode.length())
				retInt = 0;
			else
				retInt = i + 1;
			break;
		}

		if (i + 1 == srcCode.length())
			i = -1;
	}

	return retInt;
}

unsigned int HASSL::VirtualMachine::loopThroughReversedPath(const unsigned int initialValue)
{
	unsigned int retInt = NULL;
	unsigned int openingCount = NULL;

	for (int i = initialValue; unsigned int(i) < srcCode.length(); i--)
	{
		if (PRINT_PATH_LOOP)
			std::cout << "Hit control path Loop!\n";

		if (srcCode.at(i) == ']')
			openingCount++;

		if (srcCode.at(i) == '[' && openingCount != 0)
			openingCount--;

		if (srcCode.at(i) == '>' && openingCount == 0)
		{
			if (i + 1 == srcCode.length())
				retInt = 0;
			else
				retInt = i + 1;
			break;
		}

		if (i == 0)
			i = srcCode.length();
	}

	return retInt;
}

bool HASSL::VirtualMachine::getFileContents(const char* filepath, std::string& retStr)
{
	std::ifstream srcFile(filepath);
	if (srcFile.is_open())
	{
		retStr.clear();
		char tempChar;

		while (srcFile.get(tempChar))
		{
			retStr += tempChar;
		}
		srcFile.close();

		return true;
	}
	else
	{
		if (PRINT_CRITICALS)
		{
			std::cerr << "Could not open the file '" << filepath << "'.\n";
		}

		return false;
	}
}

bool HASSL::VirtualMachine::getStateDefinitionPositions()
{
	for (unsigned int i = 0; i < srcCode.length(); i++)
	{
		switch (srcCode.at(i))
		{
		case '%':
			if (i + 1 == srcCode.length())
				i = loopThroughComment(0, false);
			else
				i = loopThroughComment(i + 1);
			break;
		case '0':
			stateDefinitionPositions[0].push_back(i);
			break;
		case '1':
			stateDefinitionPositions[1].push_back(i);
			break;
		case '2':
			stateDefinitionPositions[2].push_back(i);
			break;
		case '3':
			stateDefinitionPositions[3].push_back(i);
			break;
		case '4':
			stateDefinitionPositions[4].push_back(i);
			break;
		case '5':
			stateDefinitionPositions[5].push_back(i);
			break;
		case '6':
			stateDefinitionPositions[6].push_back(i);
			break;
		case '7':
			stateDefinitionPositions[7].push_back(i);
			break;
		case '8':
			stateDefinitionPositions[8].push_back(i);
			break;
		case '9':
			stateDefinitionPositions[9].push_back(i);
			break;
		case 'A':
			stateDefinitionPositions[10].push_back(i);
			break;
		case 'B':
			stateDefinitionPositions[11].push_back(i);
			break;
		case 'C':
			stateDefinitionPositions[12].push_back(i);
			break;
		case 'D':
			stateDefinitionPositions[13].push_back(i);
			break;
		case 'E':
			stateDefinitionPositions[14].push_back(i);
			break;
		case 'F':
			stateDefinitionPositions[15].push_back(i);
		}
	}

	if (stateDefinitionPositions[0].size() == 0)
		return false;
	else
		return true;
}