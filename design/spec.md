# Data

Just like how in set theory, everything is a set, in Scarlet, everything is one
of these two kinds of values:
```py
UNIQUE
PAIR[ left right ]
```
Each different usage of "UNIQUE" is, as the name suggests, UNIQUE. So if I have
in a file "x IS UNIQUE y IS UNIQUE", "x =not y" is a true statement.  There is
also `PANIC`, which represents an unrecoverable error. It is not quite a value
because you can never observe it - any computation involving a PANIC itself
resolves to that same PANIC until the root of the program is reached.
```py
PANIC[ "Something really bad happened" ]
```

# Syntactic Constructs
Some of these make use of the syntax sugars and standard library definitions
listed in the next sections.
```py
UNIQUE

# Provides a single field prepended to
# the fields specified in "rest".
PAIR[ left right ]
pair.LEFT
pair.RIGHT
pair.IS_PAIR

# Declares a new variable. x, y, and z
# are the conditions that must be true
# to be able to assign a value to the 
# variable.
VARIABLE[ x y z ]

# Substitute the variables y and z in 
# the expression x.
x[ y IS 123  z IS 456 ]

# Flat Axiom
# Requires:
# x =NOT y
# DECIDE[ a b x y ] = x
# Produces:
# a = b
t_flat[ a b x y ]

# Or alternatively, all expressions of
# the form:
# DECIDE[ DECIDE[ a b x y ] x u v ]
# Where x =NOT y
# Reduces to
# DECIDE[ a b u v ]
#
# And also:
# DECIDE[ DECIDE[ a b x y ] y u v ]
# Where x =NOT y
# Reduces to
# DECIDE[ a b v u ]

# Invariant Truth Axiom
# Given
# x
# Proves
# x = true
t_invariant_truth[ x ]

# Invariant Truth Inverse Axiom
# Given
# x = true
# Proves
# x
t_invariant_truth_inv[ x ]

# Equality Axiom, requires 
# a = b
# Produces invariant:
# f[ a ] = f[ b ]
t_equal_ext[ a b f ]

# Equality Extension Inverse Axiom
# Given
# f[ a ] =not f[ b ]
# Produces invariant:
# a =not b
t_equal_ext_inv[ a b f ]

# Given
# false
# Proves
# x
t_explode IS
t_invariant_truth_inv[ x ]
USING {
    # Proves false = true
    t_invariant_truth[ false ]
    # Proves x = true
    t_equal_ext[
        false
        true
        DECIDE[ $ true true x ]
    ]
}

# Given
# a = b
# Proves
# DECIDE[ a b x y ] = x
t_decide_equal IS
t_equal_ext[ a  b  DECIDE[ $ b x y ] ]

# Given
# a =NOT b (DECIDE[ a b false true ])
# Proves
# (a = b) = false (DECIDE[ DECIDE[ a b true false ] false true false ])
# Not needed because (a = b) = false reduces to a =NOT b

# Given
# a =NOT b
# Proves
# DECIDE[ a b x y ] = y
#
# DECIDE[ DECIDE[ a b false true ] false x y ]
t_decide_unequal IS
DECIDE[
    a
    b

    t_equal_ext[
        a =NOT b
        true
        DECIDE[ $ false x y ]
    ]
]
USING {
    # Proves (a =NOT b) = true
    t_invariant_truth[ a =NOT b ]
}

# Need something that says
# Given 
# DECIDE[ x y true false ] = false
# Proves
# DECIDE[ DECIDE[ a b x y ] x u v ] 
# = DECIDE[ a b u v ]
t_chain IS DECIDE[
    a b


]

# If a = b, returns x, otherwise 
# returns y.
DECIDE[ a b x y ]
```

```py
# Logicings
a IS VAR[]
b IS VAR[]
c IS VF[ Bool ]
c1 IS VF[ Bool ]
c2 IS VF[ Bool ]
x IS VAR[ ]
fx IS VAR[ SUB x ]
fc IS VAR[ SUB c ]

AXIOM_OF_EQUALITY[ a  b  x = a ]
```

# Syntax Sugars
Some of these use standard library definitions
```py
{ something IS 123 }               
# Sugar for...
POPULATED_STRUCT[ 
    "something" 
    123 
    EMPTY_STRUCT 
]

{ x y z }                          
# Sugar for...
POPULATED_STRUCT[ 
    "" 
    x 

    POPULATED_STRUCT[
        "" 
        y 

        POPULATED_STRUCT[
            "" 
            z 
            EMPTY_STRUCT
        ]

    ]

]

x[ 123 456 ]
# By automatically determining which 
# variables to substitute based on 
# variables contained in x, sugar 
# for...
x[ y IS 123  z IS 456 ]

VF[ x ]
# sugar for...
VARIABLE[ SELF FROM x ]

PICK[
    ON c1 v1
    ON c2 v2
    ELSE v3
]
# sugar for...
IF_THEN_ELSE[
    c1
    v1
    IF_THEN_ELSE[ c2 v2 v3 ]
]

x AND y
# sugar for...
bool.and[ x y ]

x OR y
# sugar for...
bool.or[ x y ]

# Returns 'true' if expr could have been
# constructed using source. Only
# compiles for certain sources probably
# due to Godel's incompleteness theorem
# and definitely because there's no good
# way to handle the case `true FROM
# (hash = expected)`
expr FROM source
# When source is a struct, sugar for...
expr IS_POPULATED_STRUCT
    AND expr.LABEL = source.LABEL
    AND expr.0 FROM source.0
    AND expr.REST FROM source.REST
# When source is a variable, sugar 
# for... (in other words, sugar for if 
# expr matches source's invariants)
all[ 
    (source invariants)[ source IS expr ] 
]
# When source is anything else, sugar 
# for...
expr = source

a + b
USING {
    a IS 123
    b IS 456
}
# Sugar for...
POPULATED_STRUCT[
    ""
    a + b
    {
        a IS 123
        b IS 456
    }
]
.VALUE
```

# Syntactic Constructs again, but with the sugars the average programmer would use.
```py
# Creates a struct
{ f1 IS asdf  f2 IS 123  456 }
# Access a member of a struct by its 
# index
x.1
# Access by its label
x.f2

# Declares a new variable requiring 
# x y and z to be true
VARIABLE[ x y z ]
x.INVARIANTS                       

# Substitute the variables y and z in 
# the expression x.
x[ y IS 123  z IS 456 ]
# Implicit substitution.
x[ 123 456 ]

# Equality
x = y

# Returns 'true' if expr could have been
# constructed using source. 
expr FROM source

# Pythagorean theorem
distance IS
sqrt[ dx * dx + dy * dy ]
USING {
    dx IS x2 - x1
    dy IS y2 - y1
}
```

# The Standard Library (a really big struct)

```py
{
    struct IS 
    {
        # We can't use bool.or or OR
        # because member access has
        # yet to be defined.
        true IS bool.VALUE
        left IS VAR[]
        right IS VAR[]
        or IS PICK[
            ON left true
            ELSE right
        ]

        Struct IS
        VAR[
            or[
                SELF = EMPTY_STRUCT 
                SELF IS_POPULATED_STRUCT
            ]
        ]

        x IS VF[ Struct ]
    }

    bool IS
    {
        true IS UNIQUE
        false IS UNIQUE

        # The compiler can be smart here and
        # realize there is only two things
        # a Bool can be so it can get away
        # with using a single bit to store
        # a Bool.
        Bool IS 
        VAR[ 
            SELF = true OR SELF = false 
        ]

        x IS VF[Bool]
        left IS VF[Bool]
        right IS VF[Bool]

        not IS PICK[ 
            ON x false  
            ELSE true 
        ]

        and IS
        PICK[
            ON left true
            ELSE right
        ]

        or IS
        PICK[
            ON left true
            ELSE right
        ]

        implies IS
        PICK[
            ON left right
            ELSE true
        ]

        equal IS
        PICK[
            ON left right
            ELSE not[ right ]
        ]

        xor IS
        PICK[
            ON left not[ right ]
            ELSE right
        ]
    }

    true IS bool.true
    false IS bool.false
    Bool IS bool.Bool
    # Alternatively:
    # Bool IS bool.REST.REST.VALUE
}
```

# Examples
```

```

# Checks

Given `POPULATED_STRUCT[ label value remainder ]`
- `label` requires `label FROM String`
- `remainder` requires `remainder FROM Struct`

Given `VAR[ i ]`
- `i` requires `i FROM Struct`
- for every field in i:
    - the field must have no label
    - the field requires `field FROM Bool`

Given `x SUB[ s ]`
- `s` requires `s FROM Struct`
- for every field in s:
    - the field must have no label
    - its value must be a struct
        - Satisfying `field FROM { var val }` where `var` and `val` are variables.
        - `field.0` must be a variable
        - all the fields in 

# THE REDUCTION AXIOMS
Forall x, x = x
Given l1 = l2, f1 = f2, r1 = r2, POPULATED_STRUCT[ l1 f1 r1 ] = POPULATED_STRUCT[ l2 f2 r2 ]
Given x = UNIQUE, y = UNIQUE, (x = y) = false
Given x = VAR[ s ], x.INVARIANTS = s
