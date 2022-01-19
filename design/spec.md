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

# If a = b, returns x, otherwise 
# returns y.
DECISION[ a b x y ]
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
y IS VAR[ ]
u IS VAR[ ]
v IS VAR[ ]

# from
# t_just
# proves
# t_just
t_just IS VAR[ SELF ]

# Axiom, proves
# true
t_trivial

# Axiom, from
# x
# Proves
# x = true (DECISION[ x true true false ])
t_invariant_truth[ x ]

# Axiom, from
# x = true (DECISION[ x true true false ])
# Proves
# x
t_invariant_truth_inv[ x ]

# Axiom, proves
# x = x (DECISION[ x x true false ])
t_eq_refl[ x ]

# Axiom, from
# a = b (DECISION[ a b true false ])
# Proves
# fx[ b ] = fx[ a ] (DECISION[ fx[ b]  fx[ a ]  true  false ])
t_eq_ext_rev[ a b fx ]

# Axiom, From
# a = b (DECISION[ a b true false ])
# a
# Proves
# b
t_inv_eq[ a b ]

# Reduction axiom, from
# a = b
# replaces
# DECISION[ a b x y ] 
# with 
# x

# Reduction axiom, from
# a =NOT b
# replaces
# DECISION[ a b x y ]
# with
# y

# Reduction axiom, replaces
# x[ x IS y ]
# with
# y

# Reduction axiom, replaces
# a[ x IS y ]
# with
# a
# (when no other reduction rule applies)

# From
# a = b
# Proves
# b = a
t_eq_symm IS t_eq_ext_rev[ a b x ]

# From
# a =NOT b
# Proves
# b =NOT a
t_eq_symm IS todo

# From
# a = b
# Proves
# fx[ a ] = fx[ b ]
t_eq_ext IS 
# Proves fx[ a ] = fx[ b ] from b = a
t_eq_ext_rev[ b a fx ]
USING {
    # Proves b = a
    t_eq_symm[ a b ]
}

# From
# false
# proves
# b
t_explode_false IS
t_inv_eq_ext[ false true true_or_b ]
USING {
    true_or_b IS
    DECISION[ $ false true b ]
    # Proves false = true
    t_invariant_truth[ false ]
}

# From
# a
# a = false
# proves
# b
t_explode_contradiction IS
t_explode_false
USING {
    # Proves false
    t_invariant_truth_inv[ false ]
    # Proves false = true
    t_eq_trans[ false a true ]
    # Proves false = a
    t_eq_symm[ a false ]
    # Proves a = true
    t_invariant_truth[ a ]
}

# From
# a = b
# fx[ a ]
# Proves
# fx[ b ]
t_inv_eq_ext IS 
t_inv_eq[ fx[ a ]  fx[ b ] ]
USING {
    # Proves fx[ a ] = fx[ b ]
    t_eq_ext[ a b fx ]
}

# From
# a = b
# b = x
# Proves
# a = x
t_eq_trans IS 
t_inv_eq[ a = b  a = x ]
USING {
    # Proves (a = b) = (a = x)
    t_eq_ext[ b  x  a = $ ]
}

# From
# DECISION[ a  b  x = u  x = v ]
# Proves
# x = DECISION[ a b u v ]
t_extract_eq_from_decision
DECISION[
    a b

    t_just[ x = conclusion_right ]
    USING {
        # Proves x = conclusion_right
        # conclusion_right -> u by a = b
        t_inv_eq_ext[ u  x  $ = conclusion_right ]

        # Proves x = u from (x = u) = (x = u)
        # hypothesis -> (x = u) by a = b
        t_inv_eq[ hypothesis  x = u ]

        t_refl[ x = u ]
    }

    t_just[ x = conclusion_right ]
    USING {
        # Proves x = conclusion_right
        # conclusion_right -> v by a = b
        t_inv_eq_ext[ v  x  $ = conclusion_right ]

        # Proves x = v from (x = v) = (x = v)
        # hypothesis -> (x = v) by a = b
        t_inv_eq[ hypothesis  x = v ]

        t_refl[ x = v ]
    }
]
USING {
    hypothesis IS DECISION[ a  b  x = u  x = v ]
    conclusion IS x = conclusion_right
    conclusion_right IS DECISION[ a b u v ]
}

# From
# (a = b) = false
# proves
# (a =NOT b)
t_equal_then_not_is_eqn IS


# Proves
# DECISION[ a b x x ] = x
t_decision_illusion
DECISION[
    a b
    t_just[ statement ]
    t_just[ statement ]
]
USING {
    statement IS DECISION[ a b x y ] = x
}

# From
# x = y
# Proves
# DECISION[ a b x y ] = x
t_decision_eq_illusion IS
t_inv_eq_ext[ x  y  DECISION[ a b x $ ] = x ]
USING {
    # Proves DECISION[ a b x x ] = x
    t_decision_illusion[ a b x y ]
}
DEPENDING_ON[ a b x y ]

# From
# (x = y) = (u = v)
# Proves
# DECISION[ DECISION[ a b x y ] x u v ] = DECISION[ a b u v ]
DECISION[
    a b

    t_just[ abxy_xuv = u ]
    USING {
        t_just[ abxy = x ]
    }

    DECISION[
        x y

        t_just[ abxy_xuv = abuv ]
        USING {
            # Proves abxy_xuv = v
            t_inv_eq_ext[ u  v  abxy_xuv = $ ]
            # Proves abxy_xuv = u
            t_decision_eq_illusion[ abxy x u v ]
            # Proves u = v
            t_inv_eq[ x = y  u = v ]
        }

        USING {
            # Proves y =NOT x
            t_eqn_symm[ x y ]

            t_just[ x =NOT y ]
        }
    ]
    USING {
        t_just[ abxy = y ]
    }
]
USING {
    abxy IS DECISION[ a b x y ]
    abxy_xuv IS DECISION[ abxy x u v ]
    abuv IS DECISION[ a b u v ]
}

# From
# (x = y) = (u = v)
# Proves
# DECISION[ DECISION[ a b x y ] y u v ] = DECISION[ a b v u ]
```

# Syntax Sugars
Some of these use standard library definitions
```py
{ something IS 123 }               
# Sugar for...
PAIR[ PAIR[ "something" 123 ]  void ]

{ x y z }                          
# Sugar for...
PAIR[ 
    PAIR[ "" x ]
    PAIR[
        PAIR[ "" y ]
        PAIR[ PAIR[ "" z ]  void ]
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
DECISION[
    c1
    true
    v1
    DECISION[ c2 true v2 v3 ]
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
# When source is a pair, sugar for...
expr.IS_PAIR
    AND expr.LEFT FROM source.LEFT
    AND expr.RIGHT FROM source.RIGHT
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
PAIR[
    a + b
    {
        a IS 123
        b IS 456
    }
]
.LEFT
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
