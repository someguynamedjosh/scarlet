Things that need to be yes() where `w, x, y, z` are variables, `a, b, c, d` are
uniques, `fx, gy` are dexes of `x` and `y` respectively, and `fxy` is a dex of
both `x` and `y`.
```rs
x =<= x
x(a) =<= a
fxy(fx({x y})) =<= fx({x y})
DECISION{a b c d} =<= DECISION{a b c d}
{a b} =<= {a b}
fx(fx IS gy(x))(x IS a) =<= gy(a)
fx(x IS a)(fx IS gy(x)) =<= gy(a)
a RECURSION =<= a RECURSION
a =/= a RECURSION
```

THE LHS REFINEMENT RULES:
- For everything:
    1. if lhs.dereference().is_same_instance_as(rhs.dereference()), return yes().
    99. (After everything else) if self is LHS, try refining RHS instead.
- For variables:
    1. Everything rules
    2. If I have no dependencies, return Yes({SELF -> rhs}, {})
    3. If I have dependencies, and the other side has an equal or greater number
       of dependencies, return Yes({SELF -> rhs(rd1 -> sd1, rd2 -> sd2, etc.),
       self_dep_1 -> rhs_dep_1, self_dep_2 -> rhs_dep_2, etc.}, {})
        a. During this process, if I have any dependencies that themselves have
        dependencies, return Unknown.
        b. During this process, do not consider dependencies on the RHS that
        have their own dependencies.
    4. If I am the lhs and I have more dependencies than the other side, refine
    the right.
    5. If I am the rhs and I have more dependencies than the other side, return
    Unknown.
- For substitutions:
    1. Everything rules
    2. Check if our base is equal to rhs.
        a. If yes, select all substitutions in the result with targets we also
           substitute (also select targets that aren't substituted in the result
           and treat them as a substitution like target -> target).
            a. Remove those substitutions from the lhs of the result.
            b1. If on rhs, test equality of the value from the substitution on
            the left and the value from SELF on the right.
            b2. If on lhs, test equality of the value from SELF on the left and
            the value from the substitution on the right.
            c1. If it's anything other than Yes, return Unknown.
            c2. If it's Yes, insert its substitutions into the result as long as
            the appropriate side is actually dependant on them.
        c. Insert any substitutions that the substituted values in the opposite
        side of the result are dependant on into those values.
        d. Return the modified result.
- For uniques:
    1. Everything rules
    2. If rhs is a unique of a different id, return No.

THE RHS REFINEMENT RULES:
- For variables, substitutions, uniques:
    1. Same procedure as LHS, but reversing any mentions of LHS and RHS, except
       that they always refine on the left.

THE POST-PROCESSING RULES:
1. Any substitution item of the form x(x IS y) should be replaced with y.
2. Any substitution of the form x -> x should be removed, both in items and in
   the result.
3. Any empty substitution item should be converted to its base.
4. Any pair of substitutions on the LHS and RHS with the same target and value
should be removed.
5. Remove any substitutions for which computing `value =<= target` produces
   `Yes({}, {})` after applying 1-4.
```rs
x =<= x
"yes() by #1"
```
```rs
x =<= x OTHER
"yes() by #1"
```
```rs
x =<= x(x IS a)
"Yes({x -> x(x IS a)}) by #2"
// After post processing
"Yes({x -> a}) by #2"
```
```rs
x(x IS y) =<= y
    x =<= y
    "Yes({x -> y}) by #2"
    y =<= y
    "Yes() by #1"
"Yes() by #2"
```
```rs
a =<= x(x IS a)
    a =>= x(x IS a)
        a =<= x
            a =>= x
            "Yes({}, {x -> a}) by #2"
        "Yes({}, {x -> a}) by #99"
        a =<= a
        "Yes() by #1"
    "Yes() by #2"
"Yes() by #99"
```
```rs
fx =<= x OTHER
"Yes({fx -> x(x IS x), x -> x}) by #2"
// After post processing
"Yes({fx -> x})"
```
```rs
fx =<= y OTHER
"Yes({fx -> y(y IS x), x -> y}) by #2"
// After post processing
"Yes({fx -> x, x -> y})"
```
```rs
fx =<= {x a} OTHER
"Yes({fx -> {x a}(x IS x), x -> x}) by #2"
// After post processing
"Yes({fx -> {x a}})"
```
```rs
fx =<= {x y} OTHER
"Yes({fx -> {x y}(x IS x), x -> x}) by #2"
// After post processing
"Yes({fx -> {x y}})"
```
```rs
fx(x IS a) =<= fx(x IS a)
    fx =<= fx(x IS a)
        fx =>= fx(x IS a)
            fx =<= fx
            "Yes() by #1"
            x =>= a
            "Yes({x -> a}) by #2"
        "Yes({x -> a}) by #2"
    "Yes({x -> a}) by #4"
    a =>= a
    "Yes() by #2"
"Yes() by #4"
```
```rs
fx(x IS y) =<= fx(x IS y)
    fx =<= fx(x IS y)
    "Yes({fx -> fx(x IS y)(y IS x), x -> y}) by #3"
    y =<= y
    "Yes() by #1"
"Yes({fx -> fx(x IS y)(y IS x)}) by #2"
    // Post processing
    fx(x IS y)(y IS x) =<= fx
        fx(x IS y) =<= fx
            fx =<= fx
            "Yes() by #1"
            x =>= y
            "Yes({x -> y}) by #2"
        "Yes({x -> y}) by #2"
    "Yes({x -> y(y IS x)}) by #2"
    // After post processing
    "Yes()"
// After post processing
"Yes()"
```
```rs
x(x IS y) =<= z
    x =<= z
    "Yes({x IS z}) by #2"
    y =<= z
    "Yes({y IS z}) by #2"
"Yes({y IS z}) by #2"
```
```rs
x(x IS y) =<= z(z IS w)
    x =<= z(z IS w)
    "Yes({x IS z(z IS w)}) by #2"
    y =<= z(z IS w)
    "Yes({y IS z(z IS w)}) by #2"
"Yes({y IS z(z IS w)}) by #2"
// After post-processing.
"Yes({y IS w}) by #2"
```
```rs
fx(x IS y(y IS z)) =<= fx(x IS z)
    fx =<= fx(x IS z)
    "Yes({fx IS fx(x IS z)(z IS x)   x IS z}) by #2"
    y(y IS z) =<= z
        y =<= z
        "Yes({y IS z}) by #2"
        z =<= z
        "Yes() by #1"
    "Yes() by #2"
"Yes({fx IS fx(x IS z)(z IS x)}) by #2"
    // Post processing
    fx(x IS z)(z IS x) =<= fx
        fx(x IS z) =<= fx
            fx =<= fx
            "Yes() by #1"
            z =<= x
            "Yes({z IS x}) by #2"
        "Yes({z IS x}) by #2"
        x =<= x
        "Yes() by #1"
    "Yes() by #2"
// After post-processing.
"Yes() by #2"
```
```rs
fx(x IS y) =<= fx(x IS z)
    fx =<= fx(x IS z)
    "Yes({fx IS fx(x IS z)(z IS x)   x IS z}) by #2"
    y =<= z
    "Yes({y IS z}) by #2"
"Yes({fx IS fx(x IS z)(z IS x)   y IS z}) by #2"
    // Post processing
    fx(x IS z)(z IS x) =<= fx
        fx(x IS z) =<= fx
            fx =<= fx
            "Yes() by #1"
            z =<= x
            "Yes({z IS x}) by #2"
        "Yes({z IS x}) by #2"
        x =<= x
        "Yes() by #1"
    "Yes() by #2"
    z =<= y
    "Yes({z IS y}) by #2"
// After post-processing.
"Yes({y IS z}) by #2"
```
```rs
fx =<= fxy
"Yes({fx IS fxy(x IS x)  x IS x}) by #3"
    // Post processing
    fxy =<= fx
        fxy =>= fx
        "Yes({} {fx IS fxy(x IS x)  x IS x}) by #3"
    "Yes({} {fx IS fxy(x IS x)  x IS x}) by #3"
    // After post-processing.
    "Yes({} {fx IS fxy}) by #3"
// After post-processing.
"Yes({fx IS fxy}) by #2"
```
```rs
fx =<= fxy(y IS a)
"Yes({fx IS fxy(y IS a)(x IS x), x IS x}) by #3"
    // Post processing
    fxy(y IS a) =<= fx
        fxy =<= fx
            fxy =>= fx
            "Yes({} {fx IS fxy(x IS x)  x IS x}) by #3"
        "Yes({} {fx IS fxy(x IS x)  x IS x}) by #3"
        a =<= y
        "Yes({} {y IS a})"
    "Yes({} {fx IS fxy(x IS x)(y IS a)  x IS x}) by #3"
// After post-processing.
"Yes({fx IS fxy(y IS a)}) by #2"
```
```rs
// THIS IS A TRICKY CASE!
// Need to check that:
// fx(x IS y)(fx IS fxy   y IS x)
// Is dependant on both x *and* y, not just x.
// Explanation: the substitution should not replace the y being introduced by
// fxy, only the y in the original expression. However, the first (x IS y)
// should still replace the x in `fxy`.
fx(x IS y) =<= fxy
    fx =<= fxy
    "Yes({fx IS fxy(x IS x), x IS x}) by #3"
    y =<= x
    "Yes({y IS x}) by #2"
"Yes({fx IS fxy(x IS x), y IS x}) by #2"
    // Post processing
    fxy =<= fx
        fxy =>= fx
        "Yes({} {fx IS fxy(x IS x)  x IS x}) by #3"
    "Yes({} {fx IS fxy(x IS x)  x IS x}) by #3"
    y =<= x
    "Yes({y IS x}) by #2"
// After post-processing.
"Yes({fx IS fxy   y IS x}) by #2"
```

```rs
fx(x IS a) =<= fx(x IS a)
    fx =<= fx(x IS a)
        fx =>= fx(x IS a)
            fx =<= fx
            "Yes()"
            x =<= a
            "Yes({x IS a})"
        "Yes({x IS a})"
    "Yes({x IS a})"
    a =>= a
    "Yes({})"
"Yes()"
```

```rs
fx(x IS a) =<= gy(y IS a)
    fx =<= gy(y IS a)
        fx =>= gy(y IS a)
            fx =<= gy
            "Yes({fx IS gy(y IS x)   x IS y})"
        "Yes({fx IS gy(y IS x)   x IS y(y IS a)})"
    "Yes({fx IS gy(y IS x)   x IS y(y IS a)})"
    a =<= y(y IS a)
        a =>= y(y IS a)
            a =<= y
                a =>= y
                "Yes({} {y IS a})"
            "Yes({} {y IS a})"
        a =<= a
            "Yes()"
        "Yes()"
    "Yes()"
"Yes({fx IS gy(y IS x)})"
```

```rs
fx(x IS y) =<= fx(x IS y)
    fx =<= fx(x IS y)
    "Yes({fx IS fx(x IS y)(y IS x)  x IS y})"
    y =<= y
    "Yes()"
"Yes({fx IS fx(x IS a)(a IS x)})"
```


```rs
fx(x IS a)(fx IS gy(y IS x)) =<= gy(y IS a)
    fx(x IS a) =<= gy(y IS a)
        fx =<= gy(y IS a)
            fx =>= gy(y IS a)
                fx =<= gy
                "Yes({fx IS gy(y IS x)   x IS y})"
            "Yes({fx IS gy(y IS x)   x IS y(y IS a)})"
        "Yes({fx IS gy(y IS x)   x IS y(y IS a)})"
        a =<= y(y IS a)
        "Yes({})"
    "Yes({fx IS gy(y IS x)})"
    gy(y IS x) =<= gy(y IS x)
    "Yes({gy IS gy(y IS x)(x IS y)})"
"Yes({})"
```