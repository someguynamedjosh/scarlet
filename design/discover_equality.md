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