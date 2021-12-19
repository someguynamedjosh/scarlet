if [9] then [9] else [9] -> 9
[5] + [4] -> 5
[4] * [3] -> 4
- [3] -> 3
[2] ? -> 2
[2] . [1] -> 2


# if a + b * c then d else e

Nodes:
Operators:
if

Nodes:
a
Operators:
if

Nodes:
a
Operators:
+
if

Nodes:
a b
Operators:
+
if

Nodes:
a b
Operators:
*
+
if

Nodes:
a b c
Operators:
*
+
if

Nodes:
a b*c
Operators:
+
if

Nodes:
a+b*c
Operators:
if then

Nodes:
a+b*c d
Operators:
if then

Nodes:
a+b*c d
Operators:
if then else

Nodes:
a+b*c d e
Operators:
if then else

Nodes:
ifa+b*cthendelsee
Operators:

# -a * -b

Nodes:
Operators:
-

Nodes:
a
Operators:
-

Nodes:
-a
Operators:
*

Nodes:
-a 
Operators:
-
*

Nodes:
-a b
Operators:
-
*

Nodes:
-a -b
Operators:
*

Nodes:
-a*-b
Operators:
*

# a? * b.c?
Nodes:
a
Operators:
?

Nodes:
a?
Operators:
*

Nodes:
a? b
Operators:
*

Nodes:
a? b
Operators:
.
*

Nodes:
a? b c
Operators:
.
*

Nodes:
a? b.c
Operators:
?
*

Nodes:
a? b.c?
Operators:
*

Nodes:
a?*b.c?
Operators:

# if a + b * c then d else e?

Stack:

Stack:
if ?

Stack:
a
if ?

Stack:
+ a ?
if ?

Stack:
b
+ a ?
if ?

Stack:
* b ?
+ a ?
if ?

Stack:
c
* b ?
+ a ?
if ?

Stack:
* b c
+ a ?
if ?

Stack:
+ a (* b c)
if ?

Stack:
if then (+ a (* b c)) ?

Stack:
d
if then (+ a (* b c)) ?

Stack:
d
if then else (+ a (* b c)) d ?

Stack:
e
if then else (+ a (* b c)) d ?


# a * b + c

Stack:
a

Stack:
* a ?

Stack:
b
* a ?

Stack:
b
* a ?

Stack:
* a b

Stack:
+ (* a b) ?

Stack:
c
+ (* a b) ?
