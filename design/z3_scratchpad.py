import z3

Z = z3.IntSort()
S = z3.StringSort()

Any = z3.Datatype('Any')
Any.declare('Unique', ('id', Z))
Any.declare('Struct', ('label', S), ('value', Any), ('rest', Any))
Any.declare('Invalid')

Any = Any.create()

# https://www.philipzucker.com/programming-and-interactive-proving-with-z3py/


def match(x, **kwargs):
    t = x.sort()
    nc = t.num_constructors()
    acc = kwargs["_"]  # default argument
    for c in range(nc):
        con = t.constructor(c)
        rec = t.recognizer(c)
        nfields = con.arity()
        if nfields == 0:
            res = kwargs[con.name()]
        else:
            res = kwargs[con.name()](*[t.accessor(c, a)(x)
                                       for a in range(nfields)])
        acc = z3.If(rec(x), res, acc)
    return acc


true = Any.Unique(z3.IntVal(0))
false = Any.Unique(z3.IntVal(1))
void = Any.Unique(z3.IntVal(3))


def normalize(x):
    return z3.If(
        Any.is_Struct(x),
        z3.If(
            # True,
            Any.is_Struct(Any.rest(x)),
            x,
            z3.If(
                Any.rest(x) == void,
                x,
                Any.Invalid,
            )
        ),
        x,
    )


def struct_from_fields(*fields):
    base = void
    for (label, value) in fields:
        base = Any.Struct(z3.StringVal(label), value, base)
    return base


val = struct_from_fields(("hello", true), ("world", false))
# val = Any.Struct(z3.StringVal("asdf"), false, false)
val = normalize(val)
print(z3.simplify(val))
