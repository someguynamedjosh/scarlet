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