# Data

Just like how in set theory, everything is a set, in Scarlet, everything is
one of these three kinds of values:
```py
UNIQUE
EMPTY_STRUCT
POPULATED_STRUCT[ label value rest ]
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
# Consumes Invariants:
# rest FROM Struct
POPULATED_STRUCT[ label value rest ]
struct.LABEL
struct.VALUE
struct.REST
struct IS_POPULATED_STRUCT

# Declares a new variable. x, y, and z
# are the conditions that must be true
# to be able to assign a value to the 
# variable.
VARIABLE[ { x y z } ]

# Substitute the variables y and z in 
# the expression x.
x[ y IS 123  z IS 456 ]

# Returns 'true' if the two expressions
# are definitionally equal
x = y

# Axiom of Equality, requires 
# f FROM Bool
# Produces invariant:
# a = b IMP bool.equal[ f[ a ]  f[ b ] ]
AE[ a b f ]

# Returns 'x' if c is true, otherwise 
# returns y.
IF_THEN_ELSE[ c x y ]
```

```py
# Logicings
a IS VAR[]
b IS VAR[]
c IS VF[ Bool ]
c1 IS VF[ Bool ]
c2 IS VF[ Bool ]
x IS VAR[]
f IS VAR[DEPENDS_ON x]

# Don't know what to call this. Proves
# a = b IMP f[ a ] = f[ b ]
value_ext IS
AE[ a  b  f[ a ] = f[ $ ] ]

# Proves b = a IMP f[ a ] = f[ b ]
rev_value_ext IS
AE[ b  a  f[ $ ] = f[ b ] ]

# Requires x FROM Bool, f[ true ] and 
# f[ false ], proves f[ x ]
prove_bool_by_cases
PICK[
    ON x  AE[ true  x f ] 
    ELSE  AE[ false x f ]
]
USING {
    x IS VF[ Bool ]
    f IS VAR[ 
        SELF FROM Bool  
        SELF[ true ] 
        SELF[ false ] 
        DEPENDS_ON x 
    ]
}



# Proves c = (c = true)
eq_true_is_ident IS
prove_bool_by_cases[ 
    c  
    $ = ($ = true) 
]

# Proves c NOT = (c = false)
eq_false_is_not IS
prove_bool_by_cases[
    c
    $ NOT = ($ = false)
]

# Proves c IMP IF_THEN_ELSE[ c a b ] = a
prove_bool_by_cases[
    c
    $ IMP IF_THEN_ELSE [ $ a b ] = a
]

# Proves IF_THEN_ELSE[ b = a  b  a ] = a
equal_branch_is_identity IS
PICK[
    ON b = a
    AE[ true  b = a  s ]
]
USING {
    s IS IF_THEN_ELSE[ $ b a ] = a
}

x IS VAR[SELF = a OR SELF = b]
swap IS
PICK[
    ON x = a  b
    ELSE      a
]

# Proves (a = b) = (b = a)
eq_symm IS
USING {
    # Proves a = b IMP b = PICK[ ON b = a  b  ELSE  a ]
    value_ext[ a b swap ]
    # Proves a = b IMP b = a
    AE[ swap[ a ]  b  a = b IMP b = $ ]
}

# Need something that from a OR b, a IMP c, b IMP c proves c.
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
