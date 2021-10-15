The language looks too much like Lean. We need to try new ideas.

- Use numbers to define inductive types, not the other way around.
- There is no equality type, there is only a proof type and equivalence
  relations.
- Logical connectives are defined as computible operations from which you can
  construct witnesses, not the other way around.

```
Never is builtin{Never}

Witness is 
$condition match{
    on{0} Never
    on{1} Void
}

WitnessEquivalent is Witness[$a == $b]

defining{
    Data any TYPE
    a    any Data
    b    any Data
    f    any After{any{Data}} Data
}
WitnessPreservesEquivalence is
After{any{WitnessEquivalent[a b]}}
WitnessEquivalent[f[a] f[b]]

BigInteger is 
Record{
    length   any 32BitInteger
    negative any Boolean
    data     any Array{repeat{length} 32BitUnsignedInteger}
}

defining{
    and is builtin{bool_and $a $b}
    or  is builtin{bool_or $a $b}
    xor is builtin{bool_xor $a $b}
}
aka{Bool}
Boolean is builtin{Boolean}

defining{
    a is any Bool
    b is any Bool
}
w_xor_symmetry is axiom{}

with{
    Data is 
    Record{
        bit is any Bool
        # $remainder is more significant than $bit.
        remainder is any AbstractInteger
    }
}
AbstractInteger is
Record{
    has_data is any Boolean

    data is 
    any has_data cases{
        on{0} Void
        on{1} Data
    }
}

AbstractInteger is
Optional[Record{
    bit  is any Boolean
    next is any AbstractInteger
}]

OPTIONAL is ENUM{
    some from $DATA 
    none from VOID
}

defining{
    some is 
    OPTIONAL::new[0 $value]

    none is
    OPTIONAL::new[1 void]
}
OPTIONAL is RECORD{
    discriminant is 
}

OPTIONAL::some[123]
# Implicit conversion for whenever we have that.
123 :OPTIONAL[_]
# Looks like we do actually need pattern matching.
$thing match{
    on{some[$x]} $x
    on{none}     0
    else{}       # TODO: Synthesize never.
}
```

What if types are patterns?
```
some_value is record{
    THING is any{TYPE}
    value is any{THING}
}

any{some_value}
```
But then what pattern would an integer be? :(
```
my_template_value is record{
    THING  is any{TYPE}
    length is any{32I}
    data   is any{Repeat{length} THING}
}

my_template_value[32I]
```
Maybe it's a built-in pattern. But a pattern is a description of how to
construct a value, so that won't work. 

Maybe it could work like this?
```
32BitInteger is builtin{32BitInteger}
:after{any{32BitInteger}} 32BitInteger
```
This would not be expressible inside the language I think.

The basic idea is that it needs to satisfy:
- `32BitInteger` is itself a value matching `32BitInteger`.
- It cannot be a unique value in addition to all the other actual 32 bit
  integers like 0, 1, etc.
```
0 match{
    on{32BitInteger[x]} x # is zero
}
```

An alternate construction, saying that `32BitInteger` is not itself a
`32BitInteger`, looks like this:
```
32_BIT_INTEGER is builtin{32_BIT_INTEGER}
:PATTERN

0 match{
    on{32_BIT_INTEGER} 0 # is an instance of 32_BIT_INTEGER
}

0 :0
0 :32_BIT_INTEGER
```
This construction seems more intuitive. We can just say that "primitive patterns
are patterns with the unique property that they do not match themselves. The
opposite category is composite patterns, which includes all patterns defined by
programmers. All composite patterns match themself. `pattern` is a composite
pattern, even though it is built in to the language."
```
ANY is struct{
    DATA  is shared_var{PATTERN}
    value is var{DATA}
}

# Same values of "DATA", potentially different values of "value".
thing1 is var{ANY}
thing2 is var{ANY}
# Different value of "DATA", since we substitute it with another variable.
thing3 is var{ANY[var{PATTERN}]}
```
What are `after` patterns? Is this nomenclature useful or do we need a separate
'from' construct?
```
input is var 32I
FUNCTION is FROM{input} 32I
function is var{FUNCTION}
function[123]
function[456]
```
```
input is var 32I
function is after{input} var{32I}
```
Okay, what if we have `taking` and `lifting`?
```
a is var 32I
b is var 32I

my_struct is struct{
    field1 is a
    field2 is b
}
# same as
my_struct is
taking{a b}
struct{
    field1 is a
    field2 is b
}
```
We could also use this to get rid of `shared_var`:
```
my_struct is struct{
    DATA  is lifted var{PATTERN}
    value is        var{DATA   }
}
```

```
DATA    is lifted var{PATTERN}
present is        var{Boolean}
value   is        var{VALUE}

VALUE is present match{
    on{0} VOID
    on{1} DATA
}

OPTIONAL is record{value}
```

```
THING is 
ENUM{
    a from A 
    b from B
} is 
struct{
    variant is var{32I }
    data    is var{DATA}

    DATA is 
    variant match{
        on{0} A
        on{1} B
        else  NEVER
    }
}

THING::a is 
struct{
    variant is 0
    data    is var{A}
    DATA    is A
}
```
Structs are just maps from keys to values. Maybe modules could just be structs
as well?
```
THING is enum{a is A b is B} is
struct{
    ANY is struct{
        # variant, data, DATA, etc.
    }
    a is value[0]
    b is value[1]
}
```
What fundamental constructs do we have so far?
```
builtin{name args*}
struct{fields*}
x field{ident}
var{pattern}
raise{vars*} x
x match{conditions*}
x replace{replacements*}
identifier{name}
```
A couple of context-dependent ones:
```
target{target} x
on{pattern} x
else{} x
```
We could also make everything resolve to a `builtin` or `fundamental`
construct...
```
fundamental{name args*}
struct{fields*} -> fundamental{struct fields*}
x field{ident} -> fundamental{field x ident}
var{pattern} -> fundamental{var location pattern}
raise{vars*} x -> fundamental{raise x vars*}
x match{conditions*} -> fundamental{match x conditions*}
x replace{replacements*} -> fundamental{replace x replacements*}
identifier{name} -> identifier{name}
target{target} x -> fundamental{target target x}
```

Transformers create or transform expressions. They can take additional
expressions in curly braces{}. There are two special transformers, `identifier`
and `fundamental`. These do not reduce to any other set of expressions.
`identifier` is special in that it quotes what is inside of it, instead of
parsing it as expressions.

```
quote{asdf asdf asdf asdf}
fundamental{prefix fundamental{construct name args} remainder}
```

There are atoms, either of the form `text{some text}` or
`fundamental{text{name} atoms*}`. All other occurences of {} resolve to
`fundamental{text{construct_body} bracket_contents*}`

There is soup. It looks like this:
```
t{two}
t{+}
t{two}
```
```
t{struct}
f{
    t{construct_body}
    t{field1}
    t{is}
    t{0123}
}
```
How the heck are we supposed to know where the operators are? Well, we'd need
some operators to be "built in" just to import other operators, definitely "is":
```
t{struct}
f{
    t{construct_body}
    f{
        t{assign}
        t{field1}
        t{0123}
    }
}
```
But wait no, we want to do it in the reverse order - higher precedence operators
going before lower precedence. Darn.

I guess syntax extensions will need to be specified outside of where they're
used.
```
use_syntax:
  - scarlet.syntax.All
```
We could also have special statements at the top of files:
```
use_syntax scarlet.syntax.All
```
Which would allow for using syntax defined in the same project! Just as long as
the dependencies form a DAG it's fine!

So we need to define:
```
builtin{name args*}
struct{fields*}
x.ident
var{pattern}
raise{vars*} x
x match{conditions*}
x replace{replacements*}
target is x
on{condition} x
else x
```
```
builtin is
root_construct[
    name   is "builtin"
    result is A.f[
        A.label is "struct" 
        A.body is parse.body
    ]
]

. is
infix_construct[
    name   is "."
    prec   is 980
    assoc  is ASSOC.left
    result is A.f[
        A.label is "member" 
        A.body is list{
            parse.left 
            parse.right
        }
    ]
]

square_bracket_substitute is
construct[
    prec   is 980
    assoc  is ASSOC.left
    result is pick{
        on{
            index[parse.file parse.here] 
            == A.t["["]
        }
        struct{
            other_bracket is 
            return 
        }
    }
]
```
okay maybe just have these builtin to start with :P
But the general idea is good, have some way to express arbitrary syntactic
constructs, which can also be used to define prefix, postfix, infix, etc.