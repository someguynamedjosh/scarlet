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
    4. If I have dependencies, and the other side has fewer dependencies, refine
       the right.
- For substitutions:
    1. Everything rules
    2. Check if our base is equal to rhs.
        a. If yes, select all substitutions in the result with targets we also
           substitute.
        b. Remove those substitutions from the lhs of the result.
        c. Test the equality of the value from SELF on the left and the value
           from the result on the right.
        d. If it's anything other than Yes, return that.
        e. If it's Yes, insert its substitutions into the result.
        f. Insert any substitutions not already tested.
        g. Return the modified result.
- For uniques:
    1. Everything rules

THE RHS REFINEMENT RULES:
- For variables, substitutions, uniques:
    1. Same procedure as LHS, but reversing any mentions of LHS and RHS, except
       that they always refine on the left.

THE POST-PROCESSING RULES:
1. Any substitution item of the form x(x IS y) should be replaced with y.
2. Any substitution of the form x -> x should be removed, both in items and in
   the result.
3. Any empty substitution item should be converted to its base.
4. Any pair of substitutions on the LHS and RHS with the same target should be
   removed.
    a. Following this, insert any substitutions resulting from computing
       `left_target =<= right_target`.
5. Remove any substitutions for which computing `value =<= target` produces
   `Yes({}, {})`.
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
```
```rs
x(y) =<= y
    x =<= y
    "Yes({x -> y}) by #2"
    y =<= y
    "Yes({}) by #1"
"Yes({}) by #2"
```
```rs
a =<= x(x IS a)
    a =>= x(x IS a)
        a =<= x
            a =>= x
            "Yes({}, {x -> a}) by #2"
        "Yes({}, {x -> a}) by #99"
        a =<= a
        "Yes({}, {}) by #1"
    "Yes({}, {}) by #2"
"Yes({}) by #99"
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
        "Yes({}, {x -> a}) by #2"
    "Yes({}, {x -> a}) by #4"
"Yes({x -> a}, {x -> a}) by #4"
// Post processing
a =<= a
"Yes()"
// After post processing
"Yes({}, {}) by #4"
```
```rs
fx(x IS y) =<= fx(x IS y)
    fx =<= fx(x IS y)
    "Yes({fx -> fx(x IS y)(y IS x), x -> y}) by #3"
    y =<= y
    "Yes() by #1"
"Yes({fx -> fx(x IS y)(y IS x)}) by #2"
```

# Will these be problematic?
```rs
x =<= x(x IS a)
"Yes({x -> x(x IS a)}) by #1"
```

Consider a theorem that proves `x = true` from `x`. You would invoke it as follows:
```rs
invariant_truth_t(y)
```
Which would trigger a search for statements of the form
```rs
x(x IS y)
```
Which is different from these cases.

But if we want to reintroduce ATP, we would start doing things like:
```rs
y =<= x(x IS y)
```
so that we can capture situations where we just need to apply some substitutions
to a theorem to get the result we want. But in that case it might not be
undesirable because we can just do `thing_that_proves_y(y IS x(x IS y))` and
move on with our lives.