#include "lib1.h"

void cool_function(int i, char c, CoolStruct *cs) {
	printf("my int: %d\n", i);
	printf("my char: %c\n", c);
	printf("my struct: {%d, %d}\n", cs->x, cs->y);
}
