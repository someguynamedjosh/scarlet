Things that need to be yes() where `w, x, y, z` are variables, `a, b, c, d` are
uniques, `fx, gy` are dexes of `x` and `y` respectively, and `fxy` is a dex of
both `x` and `y`.
```rs
x =<= x
x(a) =<= a
fxy =<= fx({x y})
DECISION{a b c d} =<= DECISION{a b c d}
{a b} =<= {a b}
fx(fx IS gy(x))(x IS a) =<= gy(a)
fx(x IS a)(fx IS gy(x)) =<= gy(a)
```