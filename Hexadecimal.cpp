#include "Hexadecimal.h"

HASSL::Hex::Hex()
{
	hexValue = HEX_0;
}

void HASSL::Hex::update(uint8_t initialInt, bool left)
{
	if (left)
		hexValue = initialInt / 16;
	else
		hexValue = initialInt % 16;
}

void HASSL::Hex::increment()
{
	hexValue++;
	hexValue %= 16;
}

void HASSL::Hex::operator=(int n)
{
	n %= 16;
	hexValue = n;
}

void HASSL::Hex::operator=(HASSL::HexadecimalValues hexVal)
{
	int n = hexVal;
	n %= 16;
	hexValue = n;
}

void HASSL::Hex::operator=(HASSL::ExitCodes exitCode)
{
	int n = exitCode;
	n %= 16;
	hexValue = n;
}

uint8_t HASSL::Hex::operator+(Hex h)
{
	uint8_t retInt = getHexValue() * 16;
	retInt += h.getHexValue();
	return retInt;
}

void HASSL::Hex::operator++()
{
	increment();
}