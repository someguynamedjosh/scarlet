THE STEPS:

1. Check the priority of the left and right constructs.
    Substitution, variable, decision
2. Call discover_equality on the higher priority one (original left breaks ties, every time the right is picked something must be flipped to keep track of this)
<!-- 3. If that turns up nothing, call discover_equality on the other one (this might not be necessary.) -->
4. If result is gained from the right side, flip the substitution sets (if result is Equal.)

discover_equality for variables:
If it is a variable without dependencies, simply return Equal([SELF IS OTHER] [])
Otherwise, if it has dependencies, figure out which of OTHER's dependencies should be assigned one of SELF's dependencies. Then add substitutions for the reverse.
E.G. if OTHER has dependencies x and y and SELF has dependencies a and b, return Equal([SELF IS OTHER[x IS a   y IS b]   a IS x   b IS y] [])

discover_equality for decisions:
if OTHER is a decision:
    EqualityResult::and(vec![
        discover_equality(SELF.left, OTHER.left),
        discover_equality(SELF.right, OTHER.right),
        discover_equality(SELF.equal, OTHER.equal),
        discover_equality(SELF.unequal, OTHER.unequal),
    ])
otherwise return Unknown

discover_equality for substitutions:
1. $result = discover_equality(SELF.base, OTHER)
2. For each substitution in SELF.subs:
    a. If the variable already requires substitution in $result.self:
        i. discover_equality(SELF.subs[var], $result.self[var])
        ii. if Equal, remove $result[var] and add all the new subs to $result
        iii. Otherwise, return the error
    b. Otherwise
        i. if any existing substitution in $result.other depends on the var being substituted, append the substitution to it
        ii. if OTHER is dependent on the variable, add the substitution to the other subs.

```rs
fx[x IS a][fx IS gy] =<= gy[a]
```

```rs
fx[x IS a][fx IS gy] =<= gy[a]
    fx[x IS a] =<= gy[a]
        gy[a] =>= fx
            fx =<= gy
                Yes([fx IS gy[y IS x]   x IS y], [])
            Yes([], [fx IS gy[y IS x]   x IS y])
            Yes([], [fx IS gy[y IS x]   x IS y[y IS a]])
        Yes([fx IS gy[y IS x]   x IS y[y IS a]], [])
        y[y IS a] =>= a
            y =>= a
                Yes([y IS a], [])
            a =>= a
                Yes([], [])
            Yes([], [])
        Yes([fx IS gy[y IS x]], [])
    Yes([fx IS gy[y IS x]], [])
```
```rs
fx[fx IS gy[x]] =<= gy[x]
    gy[y IS x] =>= fx
        fx =<= gy
            Yes([fx IS gy[y IS x]   x IS y], [])
        Yes([], [fx IS gy[y IS x]   x IS y])
        Yes([], [fx IS gy[y IS x]])
    Yes([fx IS gy[y IS x]], [])
    gy[x] =<= gy[x]
        gy[x] =>= gy
            gy =>= gy
                Yes([], [])
            Yes([], [])
            Yes([], [x IS y])
        Yes([x IS y], [])
        y =<= y
            Yes([], [])
        Yes([], [])
    Yes([], [])
```
```rs
abc[x IS y][y IS x] =<= abc
    abc[x IS y] =<= abc
        abc =<= abc
            Yes([], [])
        Yes([], [])
        Yes([], [x IS y])
    Yes([], [x IS y])
    Yes([], [x IS y[y IS x]])
```

Okay, new plan - we do this the old-fashioned way where we keep track of
substitutions as we descend BUT we keep them in an array so that we don't have
to do the whole "apply substitutions to the substitutions to represent nested
substitutions" thing.

```rs
x, y, and z are VAR[]
a, b, and c are UNIQUE

x, = x,
    Yes([])

x, = x, [x IS y]
    x, {} = y, {}
        Yes([x IS y])

x, [x IS y] = x,
    y, = x,
        Yes([y IS x])

a, = x,
    No

x, = a,
    Yes([x IS a])

x, = z[z IS y][y IS x],
    x, = z, [z IS y][y IS x]
        x, = y, [y IS x]
            x, = x,
                Yes([])

{x, a}, = {y, z}[y IS {b c}   z IS a]
    {x, a}, = {y, z}, [y IS {b c}   z IS a]
        x, = y, [y IS {b c}   z IS a]
            x, = {b c},
            Yes([x IS {b c}])
        Yes([x IS {b c}])
        a, = z, [y IS {b c}   z IS a]
            a, = a,
            Yes([])
        Yes([])
    Yes([x IS {b c}])
Yes([x IS {b c}])
```

Okay, we have something tricky.
```
fx = gy[1] should say "yes, fx is gy and x is 1"
vs.
fx = gy[x + 1] should say "yes, fx is gy[x + 1]"
```
```rust
fx, = gy, [y IS 1]
    Yes([fx IS gy[y IS x]   x IS 1])
fx, = gy, [y IS x + 1]
    Yes(fx IS gy[y IS x + 1])
fx, = gy, [y IS z + 1]
    Yes([fx IS gy[y IS z + 1][z IS x]   x IS z])
fx, = gy, [y IS z + x]
    Yes([fx IS gy[y IS z + x]])
fx, = gy, [y IS z + x]
    Yes([fx IS gy[y IS z + x]])
```
