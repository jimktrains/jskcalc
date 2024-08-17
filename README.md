# jskcalc

A RPN calculator.

Each cell can be a float (f64) or "word" (i64, but treated like bits).

To quit, `q` or ^C.

## Quick Example

```
45 d2r dup sin sq swap cos sq + p
N 1.0
2 sqrt 2 / 45 d2r cos - p
N 0.0
1 2 3 ps + ps - ps 5 p * p ps
Stack is 5 entries deep
 0: Num(3.0)
 1: Num(2.0)
 2: Num(1.0)
 3: Num(0.0)
 4: Num(1.0)

Stack is 4 entries deep
 0: Num(5.0)
 1: Num(1.0)
 2: Num(0.0)
 3: Num(1.0)

Stack is 3 entries deep
 0: Num(-4.0)
 1: Num(0.0)
 2: Num(1.0)

N 5.0
N -20.0
Stack is 3 entries deep
 0: Num(-20.0)
 1: Num(0.0)
 2: Num(1.0)

clr
654321 w p 9876543 w p ps & p pl ps
W b0000 0000  0000 0000  0000 0000  0000 0000  0000 0000  0000 1001  1111 1011  1111 0001
W b0000 0000  0000 0000  0000 0000  0000 0000  0000 0000  1001 0110  1011 0100  0011 1111
Stack is 2 entries deep
 0: Word(9876543)
 1: Word(654321)

W b0000 0000  0000 0000  0000 0000  0000 0000  0000 0000  0000 0000  1011 0000  0011 0001
W b0000 0000  0000 0000  0000 0000  0000 0000  0000 0000  0000 0000  1011 0000  0011 0001
W x00 00 00 00 00 00 B0 31
W 45105i64
W 45105u64
Stack is 1 entries deep
 0: Word(45105)

654321 w dup p 9876543 w dup p rot ps
W b0000 0000  0000 0000  0000 0000  0000 0000  0000 0000  0000 1001  1111 1011  1111 0001  
W b0000 0000  0000 0000  0000 0000  0000 0000  0000 0000  1001 0110  1011 0100  0011 1111  
Stack is 5 entries deep
 0: Word(9876543)
 1: Word(654321)
 2: Word(9876543)
 3: Word(654321)
 4: Word(45105)

& rot swap p drop p drop p     
W b0000 0000  0000 0000  0000 0000  0000 0000  0000 0000  0000 1001  1111 1011  1111 0001  
W b0000 0000  0000 0000  0000 0000  0000 0000  0000 0000  1001 0110  1011 0100  0011 1111  
W b0000 0000  0000 0000  0000 0000  0000 0000  0000 0000  0000 0000  1011 0000  0011 0001

q
```

## Stack
drop
: (1 -- )

swap
: (1 2 -- 2 1)

rot
: (1 2 3 -- 2 3 1)

dup
: duplicates the top of the stack

clr
: clear the stack

## Printing
ps
: prints the stack

p
: prints the top of the stack

pl
: prints a more detailed version of the top of the stack

## floats

To push onto the stack, just enter the number. For details see the
[rust f64 grammar](https://doc.rust-lang.org/std/primitive.f64.html#grammar).

### binary operations
- +
- -
- *
- /
- pow
- atan2
- hypot

### unary operations
- sin
- cos
- tan
- asin
- acos
- atan
- sinh
- cosh
- tanh
- asinh
- acosh
- atanh
- d2r
- r2d
- 1/
- ln
- ln1+
- log10
- log2
- exp
- exp-1
- sqrt
- cbrt
- sq
- cb
- abs
- ceil
- floor

### constants
- pi
- e
- sqrt2
- ln2
- ln10
- tau
- pi/2
- pi/4

### misc
- dms2dd
- dd2dms

## word

To push a word, push an float then issue `w`. The integer portion of the float is considered as the word.

### binary operations
- w+
- w-
- w\*
- w/
- mod
- &
- ^
- |
- sl
- lsr
- asr

### unary operations
- ~

