#pragma once

#include <cstdint>
#include <string>

#include "definitions.h"

namespace HASSL
{
	const enum HexadecimalValues
	{
		HEX_0,
		HEX_1,
		HEX_2,
		HEX_3,
		HEX_4,
		HEX_5,
		HEX_6,
		HEX_7,
		HEX_8,
		HEX_9,
		HEX_A,
		HEX_B,
		HEX_C,
		HEX_D,
		HEX_E,
		HEX_F
	};

	class Hex
	{
	public:
		Hex();

		void update(uint8_t initialInt, bool left);
		void increment();

		uint8_t getHexValue() { return hexValue % 16; }

		void operator=(int n);
		void operator=(HASSL::HexadecimalValues hexVal);
		void operator=(HASSL::ExitCodes exitCode);
		bool operator==(int n) { return getHexValue() == n; }
		bool operator==(HASSL::HexadecimalValues hexVal) { return getHexValue() == hexVal; }
		bool operator==(HASSL::ExitCodes exitCode) { return getHexValue() == exitCode; }
		bool operator!=(int n) { return getHexValue() != n; }
		bool operator!=(HASSL::HexadecimalValues hexVal) { return getHexValue() != hexVal; }
		bool operator!=(HASSL::ExitCodes exitCode) { return getHexValue() != exitCode; }
		uint8_t operator+(Hex h);
		void operator++();
	private:
		uint8_t hexValue;
	};
}