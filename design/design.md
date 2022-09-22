```py
# From y = z
# Proves z = y
eq_symm_t IS 
tail_value({
   y IS VAR()
   z IS VAR()
   y_eq_z_t IS ANY_PROOF(y = z)

   t1 IS eq_ext_rev_t(
      x y z
      PROOF(y = z) IS y_eq_z_t
   )

   just_t(
      z = y   
      PROOF(statement) IS t1
   )
})

# With refinement types:
eq_symm_t IS 
{
    T IS ANY Type
    y IS ANY T
    z IS ANY T THAT is_equal_to(y)

    t1 IS eq_ext_rev_t(x y z)

    conclusion IS y NOTING is_equal_to(z) BECAUSE t1
}.conclusion

# Trying to make dependent types which would invoke Girard's paradox I think?
condition IS ANY Bool
Proof IS IF condition THEN Unit ELSE Never
# To invoke it, we would need to go from Proof to a refinement type thing.
# Basically, something of the type Proof(condition) -> Bool THAT is_equal_to(condition)
# Actually, I'm not sure we need that. Because GP lets us make a term of type
# (T: Type) -> T, which would be bad even if we don't allow you to prove things
# with it.
# But it looks like, if I understand correctly, this is caused by an infinite
# loop, which brings us back to needing to keep track of computational
# requirements of proofs.
# A fixed point combinator looks like this:
x IS ANYTHING
n IS ANYTHING DEPENDING_ON x
s IS ANYTHING DEPENDING_ON x

pre IS n(x IS s(x IS s))
fpc IS pre(s IS n(x IS s(x IS s)))

# With Proof type:
eq_symm IS
{
    T IS ANY Type
    y IS ANY T
    z IS ANY T
    y_eq_z IS ANY Proof(y = z)

    t1 IS eq_ext_rev_t(x y z)

    conclusion IS t1 AS Proof(z = y)
}
.conclusion
```