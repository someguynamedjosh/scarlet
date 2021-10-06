```scarlet

```

```bash
int_pattern is take{value :I32}

# is 124
123 -> int_pattern => int_pattern + 1

123
match{
    on int_pattern 
    is value + 1
}

$x And ($x Is 5)

var
match{
    on 5 is get{var Is 5}
    else error
}

var
match{
    on x + 1 is get{var is x + 1}
    else error
}

Exists $x is 5
```

If I have
```
[1, 2, 3] fmap $x + 1
```
Should it equal
```
[2, 3, 4]
```
or
```
[$x + 1, $x + 1, $x + 1]
```
?

So the thing is, if `fmap` accepts a value of type `U From{any{T}}`, then when
we do `$x + 1`, `$x` is not the same as `any{T}`. So we should do the latter
case. But then how do we make the former (more common) case ergonomic? If we do
100% positional parameters, we will always get the first case. How do we express
the second case? Remember that this is important for pattern matching. 

```
ProofExists[x Is 5] from{}
defining{
    x is any{I32}
}

# vs.

ProofExists[x Is 5] from{x}
defining{
    x is any{I32}
}

# Actually that should be 
x Is 5 from{x}
defining{
    x is any{I32}
}

# Two values
ProofExists[ProofExists[$x Is Pair[$y $z]]] using{$x}

using{$x}
$x Is Pair[$y $z]


($c $d) => 
$c match{
    on Pair[$a $b]
    is $a
}

$x match{
    on $y w+ 1
    is $y
}

defining{
    value a I32
    bound a I32
    other a I32
}
using{value other}
ProofExists::new[
    Data         is I32
    condition    is value Is bound w+ other
    data         is value w- other
    cond_witness is identity_w-w+[value]
]
:ProofExists[value Is bound w+ other]

defining{
    value a I32
    bound a I32
    other a I32
}
using{value other}
something
:Ex value Is bound w+ other

defining{
    value      a Pair[I32]
    left       a I32
    right      a I32
}
using{value}
PairExists is
Ex Ex value Is Pair::new[left right]

defining{
    Data is ProofExists::Data
    value a Data
    bound a Data
}
using{Data value}
trivial_exists is
ProofExists::new[
    condition    is value Is bound
    data         is value
    cond_witness is rfl[value]
]
:Ex value Is bound


function_input a I32
some_function  a I32 From{function_input}
call_with_one is some_function[1]

input     a I32
add_five  is input + 5

normal is
call_with_one[add_five] 
as{I32}
info{}

weird is 
call_with_one[add_five] 
as{I32 Using{input}} 
info{}

normal is
call_with_one
sub{
    some_function is 
    add_five
    sub{
        input is function_input
    }
}

weird is 
call_with_one
sub{
    some_function is add_five
}

normal is
call_with_one
sub{
    some_function is
    after{} add_five 
}

weird is
call_with_one
sub{
    some_function is
    after{input} add_five 
}

weird[5] info{}
```
After should exist everywhere, but usually inferred by the compiler:
```
var1 is any{I32}
var2 is any{I32}

item is (after{var1} var1) + (after{var2} var2)
```
It means "parse the following expression as if the value of $var is already
known, don't allow replacing it from inside the expression."
```
after{var1 var2} 
item is 
(after{} var1) + (after{} var2)
```
```
item is
(var1 :From{var1} I32) + (var2 :From{var2} I32)
:From{var1} From{var2} I32
```
