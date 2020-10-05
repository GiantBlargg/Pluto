# Pluto

24-bit big endian stack machine architecture. There are 2 stacks: the value stack and the function stack. When no function is executing a function is popped from the function stack and executed.

Note: Memory is 24-bit aligned ( ie: with 24bit pointers we can address 48 MB instead of 16 MB )

## Header 
A PLT file starts with a header that is 0x40 words long (0x40 words * 3 bytes/word = 0xC0 bytes)

|Address|Name|Meaning|
|:-|:-|:-
0x0 |Magic| always 0x504c54
0x1 |Features| bitflags to enable special features, for now it is always 0.
0x2 |Mapping| specifies the address space mapping
0xf|Reset| pointer to the function to be called at startup.
0x10-0x1f|Title|Title of the game (16 24-bit wide unicode characters)
0x20-0x2f|Developer|Developer of the game (16 24-bit wide unicode characters)
0x30-0x3f|Publisher|Publisher of the game (16 24-bit wide unicode characters)

## Mapping
### Mapping 0:
The ROM will fill as much space as it can

## Functions
Functions begin with 2 12-bit fields, representing the number of params and returns respectively. A function continues until one of the end instructions is reached. Unique about the pluto architecture: arbitrary branching is not allowed, end instructions are instead used to push to the function stack.

## Instructions
For "args" right is the top of the stack
### Stack Manipulation
|asm|opcode|args|Function|
|:-|:-|:-|:-|
`push x`|`0x001000 vvvvvv`||Pushes `v` onto the stack
`drop`|`0x001001`|`x`|Nothing (removes the top element from the stack)
`peek x`|`0x000nnn`||Returns the `n`<sup>th</sup> value of the stack
`load`|`0x001002`|`a`|Returns the value at address `a` in memory
`stor`|`0x001003`|`v a`|Stores `v` at address `a` in memory.

### Math
|asm|opcode|args|Function|
|:-|:-|:-|:-|
`neg`|`0x002000`|`x`|logical negation `!x`
`add`|`0x002001`|`x y`|`x + y`
`sub`|`0x002002`|`x y`|`x - y`
`mul`|`0x002003`|`x y`|`x * y`
`udiv`|`0x002005`|`x y`|unsigned `x / y`
`sdiv`|`0x002006`|`x y`|signed `x / y`
`mod`|`0x002008`|`x y`|unsigned `x % y`
`rem`|`0x002009`|`x y`|signed `x % y`
`not`|`0x00200b`|`x`
`and`|`0x00200c`|`x y`
`or`|`0x00200d`|`x y`
`xor`|`0x00200e`|`x y`

### Comparisons
|asm|opcode|args|Function|
|:-|:-|:-|:-|
`eq`|`0x003000`|`x y`|`x == y`
`ne`|`0x003001`|`x y`|`x != y`
`ult`|`0x003002`|`x y`|unsigned `x < y`
`slt`|`0x003003`|`x y`|signed `x < y`
`ugt`|`0x003004`|`x y`|unsigned `x > y`
`sgt`|`0x003005`|`x y`|signed `x > y`
`ule`|`0x003006`|`x y`|unsigned `x <= y`
`sle`|`0x003007`|`x y`|signed `x <= y`
`uge`|`0x003008`|`x y`|unsigned `x >= y`
`sge`|`0x003009`|`x y`|signed `x >= y`

### End of function
|asm|opcode|args|Function|
|:-|:-|:-|:-|
`ret`|`0x004000`||Ends the function
`jmp`|`0x004001`|`f`| Adds `f` to the function stack
`if`|`0x004002`|`t f2 f1`| If `t == 0`, add `f2` to the function stack, else add `f1` to the function stack
`call`|`0x004003`|`f2 f1`| Add `f2` then `f1` to the function stack. (i.e: `f1` will be executed first)

## System Calls
Function pointers less than `0x40` are reserved for system calls.

There are currently no system calls
