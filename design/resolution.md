- Unresovled structs might trigger equality checking on themselves.

- If you have:
```r
thing IS f[x] AND (#=recursion=# thing)[g[x]]
```
- You need to prove:
```
f[x] FROM c_bool
(#= recursion =# thing)[g[x]] FROM c_bool
```
So we need to have the structure there to be able to do equality traversal on.  

Maybe delay the computation of invariants and such? But invariants produce
dependencies, and dependencies are needed for substitution.