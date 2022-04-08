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

Let's list requirements:
- `thing[g[x]]` needs to know that `thing` depends on `x`.
- `AND` needs to know `f[x] FROM c_bool`
- `AND` needs to know `thing[g[x]] FROM c_bool`

Resolving identifiers is way different because we can do it as soon as we have
structs. Same thing for named members, except not really because we'll need to
know invariants to do named members on variables. We can ignore that for now.
Variable resolution is a noop at the moment.