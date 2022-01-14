#pragma once

#include <vector>

#define DEBUG false
#define TEST_PRINT_ERROR_CODES (DEBUG && false)
#define PRINT_COMMAND_POINTER_POSITION (DEBUG && false)
#define PRINT_COMMANDS (DEBUG && false)
#define PRINT_COMMENT_LOOP (DEBUG && false)
#define PRINT_PATH_LOOP (DEBUG && false)
#define PRINT_CRITICALS (DEBUG && false)
#define PRINT_ACCESSIBLE_CONTENTS (DEBUG && false)
#define PRINT_STACK_CONTENTS (DEBUG && false)
#define FRAME_BY_FRAME (DEBUG && false)

namespace HASSL
{
	const enum ExitCodes
	{
		//Successful execution
		EXIT_CODE_NULL,
		//Error in the interpreter
		EXIT_CODE_ERROR,
		//No definition for state 'A' found in the source code
		EXIT_CODE_NO_STATE,
		//Attempted push when the stack limit has been reached
		EXIT_CODE_STACK_OVERFLOW,
		//Attempted pop when the stack has no contents
		EXIT_CODE_STACK_UNDERFLOW,
		//Attempted swap, duplicate, add, or subtract with insufficient stack size
		EXIT_CODE_STACK_INSUFFICIENT_SIZE,
		//Attempted duplicate, add, or subtract with insufficient space in the stack
		EXIT_CODE_STACK_INSUFFICIENT_SPACE,
		//An unexpected token was found in the source code
		EXIT_CODE_UNEXPECTED_TOKEN,

		//Additional codes

		EXIT_CODE_8,
		EXIT_CODE_9,
		EXIT_CODE_A,
		EXIT_CODE_B,
		EXIT_CODE_C,
		EXIT_CODE_D,
		EXIT_CODE_E,
		EXIT_CODE_F
	};

	const std::vector<std::string> errorMessages
	{
		"0",
		"1: the VM encountered a critical error!",
		"2: current state undefined",
		"3: stack overflow",
		"4: stack underflow",
		"5: insufficient stack size to complete operation",
		"6: insufficient stack space to complete operation",
		"7: unexpected token",
		"8",
		"9",
		"A",
		"B",
		"C",
		"D",
		"E",
		"F"
	};
}