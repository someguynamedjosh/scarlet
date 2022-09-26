In the beginning, there was Type:
```rs
Type
```
And on the first day, God said, "Let there be placeholders for types":
```rs
Param1 IS ANY Type
value1 IS ANY Param1
Param2 IS ANY Type
```
And on the second day, God said, "let there be many inhabitants of `Type`":
```rs
True IS NEW_TYPE()
true IS True.new
False IS NEW_TYPE()
false IS False.new
First IS ANY Type
Second IS ANY Type
Pair IS NEW_TYPE(first IS ANY First, second IS ANY Second)
```
And on the third day, God said, "let the types be fruitful and multiply":
```rs
Bool IS True OR False
```
And on the fourth day, God said, "let there be builtin functions:"
```rs
if_then_else(true, a, b) IS a
if_then_else(false, a, b) IS b
a.is_exactly(b) IS is_exactly(a, b)
```
And on the fifth day, God said, "let the types be restricted in their kind:"
```rs
TrueBool IS Bool WHERE IT.is_exactly(true)
EqualPair IS Pair WHERE IT.first.is_exactly(IT.second)
```
And on the sixth day, God said, "may variables be mutable and their mutations
be composable":
```rs
target IS ANY MUTABLE I32
increment IS set(target, target + 1)
double_increment IS {
   increment(target)
   increment(target)
}
```
And on the seventh day, God said, "may syntax sugar bless the world with its 
sweetness":
```rs
ANYTHING IS ANY ANY Type
```
And on the eighth day, God said, "you're probably gonna need theorems bro":
```rs
x IS ANYTHING
prove_refl(x)
// Now x has the type "Anonymous WHERE IT.is_exactly(IT)"
```