# Constructs
- Of the form: `label{body}`
- Braces `{}` are only used for constructs, so if you see braces, you know a
construct is being used.
- The expected format of the body is different for each construct.
- A root construct is a construct that can be used to begin an expression.  For
example: `Type{}`
- A postfix construct is a construct that can be placed after any root construct
(and after any number of other postfix constructs).  For example: `Type{}
this_is_postfix{}`

## `identifier` Root Construct (M1)
Format: identifier{*text*}

Specifies any identifier. Can be a keyword, can contain special characters.
Whitespace at the start and end is not trimmed. Examples:
```ru
identifier{myvar}
identifier{my_var}
# Different from above.
identifier{ my_var }
identifier{special!@#$%^&*()characters}
```

## `select` Root Construct (M2)
Format: select{ *if statement* *elif statement*\* *else statement* }

Selects a value from the first statement in the list with a true condition.
Examples:
```ru
# Equal to '1'
show select{
    if true, 1
    else     2
}
# Equal to '2'
show select{
    if false, 1
    else      2
}
# Equal to '1'
show select{
    if   true, 1
    elif true, 2
    else       3
}
# Equal to '3'
show select{
    if   false, 1
    elif false, 2
    elif true,  3
    else        4
}
# Equal to '2'
show select{
    if 1 == 2, 1
    else       2
}
```

## `Type` Root Construct (M1)
Format: Type{ *definition or constructor statement*\* }

Defines a new datatype based on zero or more ways it can be constructed.
Examples:
```ru
# This type can never be constructed
Never is Type{}

Void is Type{ void is constructor{Self} }
show Void::void

give Both is 
Type{
    Left is any{TYPE}
    Right is any{TYPE}

    new is constructor{
        Self From{ 
            left is any{Left} 
            right is any{Right} 
        }
    } 
}

# This has type Both(Integer32 Integer32)
show Both::new(123 456)

give Either is
Type{
    Left is any{TYPE}
    Right is any{TYPE}

    left is constructor{Self From{ left is any{Left} }}
    right is constructor{Self From{ right is any{Right} }}
}

show Either::left(123) :Either(Integer32 Never)
show Either::right(456) :Either(Never Integer32)
```

## `field` Postfix Construct (M1)
Format: field{ *identifier construct* }

Gets the value of some variable the preceding expression was constructed with.
Requires a `TruthWitness` indicating that the value was constructed with the
constructor where the variable is defined. Examples:
```ru
# Equal to 123
show Either::left(123) field{left}
# Sugar for previous statement
show Either::left(123).left

take some_value :Either(Integer32 Float32)
take the TruthWitness(some_value matches{left})
# This depends on both the previous variables
show some_value.left
```

## `matched` Postfix Construct (M1)
Format: matched{ *match-case statement*\* }

Allows specifying different expressions to use depending on what constructor the
previous expression was made with. Examples:
```ru
# Equal to 1
show 
Either::left(void) matched{
    on left,  1
    on right, 2
}

# Equal to 123
show 
val matched{
    on left,  456
    on right, val.right
}
where{
    val is Either::right(123)
}

never is any{Never}
show never matched{} :WhateverType
```

## `matches` Postfix Construct (M1)
Format: matches{ *identifier construct* }

Returns `true` if the preceding expression was constructed with a constructor
with the provided name. Examples:
```ru
# Shows true
show Either::left(void) matches{left}

# Shows false
show Either::left(void) matches{right}
```

## `member` Postfix Construct (M1)
Format: member{ *identifier construct* }

Sugar: ::*identifier construct*

Selects a public item defined using a `where` construct. Examples:
```ru
my_module is public void 
where{
    an_int is public any{Integer32}
    double_that is public 2 * an_int
}

# Has type Integer32 From{use my_module member{an_int}}
show my_module member{double_that}

# Same as above, but using sugar.
# Has type Integer32 From{use my_module::an_int}
show my_module::double_that

# Shows 123
show void where{ value is public 123 }::value

# Shows 6
show my_module::double_that
where{
    my_module::an_int is 3
}

# Shows 8
show my_module::double_that(4)
```

## `type_is` Postfix Construct (M1)
Format: type_is{ *standard expression* }

Sugar: :*standard expression*

Causes a compilation error if the type specified in braces is not the same as
the type of the preceding expression. Examples:
```ru
# Works fine
show 123 type_is{Integer32}
# Causes a compilation error
show 123 type_is{Never}
# Syntactic sugar for 123!type_is{Integer32}
show 123 :Integer32
```

## `where` Postfix Construct (M1)
Format: where{ *definition statement*\* }

Defines additional members on the preceding expression. Example:
```ru
# Shows 3
show three where{ three is 3 }
```
Source files can also be understood as bodies of the where construct:
```ru
# In my_file.rer
some_variable is any{Integer32}
some_value is 42 + some_variable
```

# Expressions
- Any root construct followed by zero or more postfix constructs.

## Kinds Of Expressions
In certain situations, only a subset of constructs are allowed. Each set
of restrictions is given a name and described here:

## Standard Expression
An expression where almost everything is allowed. Certain constructs are unique
to certain kinds of expressions, these are not allowed in standard expressions.
The general term "expression" usually refers to standard expressions, it should
be considered the default kind of expression.

## Constructor-Result Expression
An expression only consisting of `identifier{Self}` and `From`.

## Item-Name Expression
An expression only consisting of `identifier`, `type_is`, and `the`.

## Item-Names Expression
An expression only consisting of `identifier`, `also`, `type_is`, and `the`.

## Item-Path Expression
An expression only consisting of `identifier` and `member`.

## Constructor-Item Expression
An expression only consisting of `identifier` and `type_is{*constructor-result expression*}`

# Statements
The body of certain constructs can be described as a list of statements. Each
statement is a series of expressions, sometimes using keywords to denote
different parts of the statement. Examples:
```ru
show select{
    if condition1, value1
    else           value2
}
show mcompose{
    value <- rsomething
    ranother_thing(value)
}
```
The four statements seen here are:
```
(keyword:if) (expression) (keyword:,) (expression)
(keyword:else) (expression)
(name expression) (keyword:<-) (expression)
(expression)
```

## `definition` Statement
Format: (show or is statement)

## `is` Statement
Format: (aka construct?) (item-name expression) is (keyword:public)? (standard expression)

Defines a value as equal to some expression. 

```ru
number is any{Integer32}

# Shows 4
show number where{number is 4}

# Shows 4
show the_value where{
    the_value is x + x 
    where{ x is number }

    number is 2
}

# Has type Integer32 From{use number}
show number
```

## `show` Statement
Format: (keyword:show) (standard expression)

Displays information about the given expression, visible in most IDEs by
hovering over the `show` keyword
