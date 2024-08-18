# jskcalc

A RPN calculator.

This is just a first pass. It needs to be cleaned up.

Each cell can be a float (f64), "word" (i64, but treated like bits), date, or string.

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
0xfeba74923f p 9876543 w p ps & p pl ps
W b0000 0000  0000 0000  0000 0000  1111 1110  1011 1010  0111 0100  1001 0010  0011 1111
W b0000 0000  0000 0000  0000 0000  0000 0000  0000 0000  1001 0110  1011 0100  0011 1111
Stack is 2 entries deep
 0: Word(9876543)
 1: Word(1094049894975)

W b0000 0000  0000 0000  0000 0000  0000 0000  0000 0000  0001 0100  1001 0000  0011 1111
W b0000 0000  0000 0000  0000 0000  0000 0000  0000 0000  0001 0100  1001 0000  0011 1111
W x00 00 00 00 00 14 90 3F
W 1347647i64
W 1347647u64
Stack is 1 entries deep
 0: Word(1347647)

0xaa dup p 0x55 dup p rot ps
W b0000 0000  0000 0000  0000 0000  0000 0000  0000 0000  0000 0000  0000 0000  1010 1010
W b0000 0000  0000 0000  0000 0000  0000 0000  0000 0000  0000 0000  0000 0000  0101 0101
Stack is 5 entries deep
 0: Word(85)
 1: Word(170)
 2: Word(85)
 3: Word(170)
 4: Word(1347647)

& rot swap p drop p drop p
W b0000 0000  0000 0000  0000 0000  0000 0000  0000 0000  0000 0000  0000 0000  1010 1010
W b0000 0000  0000 0000  0000 0000  0000 0000  0000 0000  0000 0000  0000 0000  0101 0101
W b0000 0000  0000 0000  0000 0000  0000 0000  0000 0000  0000 0000  0000 0000  0000 0000

2024-08-21 p
D Date { year: 2024, month: Aug, day_of_month: 21 }
dup dow p drop
S "Wednesday"
dup doy p drop
N 234.0
dup 78 +days p drop
D Date { year: 2024, month: Nov, day_of_month: 7 }
dup 178 +days p drop
D Date { year: 2025, month: Feb, day_of_month: 15 }

clr
'ustsp 'ustbsp ps conv ps
Stack is 2 entries deep
 0: Str("ustbsp")
 1: Str("ustsp")

Stack is 1 entries deep
 0: Num(0.33333333333333337)

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

pd
: prints and drops the top of the stack

## strings

conv
: Experimental support in parsing the GNU Units file and doing conversions

## floats

To push onto the stack, just enter the number. For details see the
[rust f64 grammar](https://doc.rust-lang.org/std/primitive.f64.html#grammar).

### binary operations
- \+
- \-
- \*
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
- recip
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

To push a word, either push a hex string starting with 0x or push an float then issue `w`. The integer portion of the float is considered as the word.

### binary operations
- +
- -
- \*
- /
- mod
- &
- ^
- |
- sl
- lsr
- asr

### unary operations
- ~

## date

To push a date, enter a string matching the regular expression `\d{4}-\d{2}-\d{2}`

### binary operations

### unary operations

- dow
- doy
- days
