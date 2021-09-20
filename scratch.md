"templates" should just be functions that produce types or functions that
produce functions.
```
let add := fn(A: Type, B: Type) -> fn(a: A, b: B) -> a + b;
add(Int, Int)(1, 2);
let Array := fn(E: Type, LEN: usize) -> !;
```

Use escape-code symbols to fit in with the cool kids.
```
// Lame, old-fashioned.
let result = 0b11 ^ 0b10;
// Fancy, guaranteed bug-free.
let result = 0b11 \xor 0b10;
let result = 0b11 ⊕ 0b10;
```

Mutable variables?
```
mut var = thing;
var = other_thing;
var ← fancy_thing;
```

Arguments are passed to functions by tuples, possible sugar to not have to do this.
```
let ex := fn(args: (T1, T2)) -> {
    thing1 = args.0;
    thing2 = args.1;
};
let ex := fn((thing1, thing2): (T1, T2)) -> ();
// Sugar could convert this:
let a: Int = 5, b: Int = 6, c: Int = 7;
// Into this:
let (a, b, c): (Int, Int, Int) = (5, 6, 7);
```

```
let AddTrait := fn(Lhs:Type, Rhs:Type) -> {
    newtype (
        Output:Type,
        add:Fn((Lhs, Rhs), self.Output)
    )
}

technique AddTrait(Lhs:Type, Rhs:Type) 
where 
    Lhs == i32, Rhs == i32 
-> (
    Output:i32,
    add:#internal_add_i32
)

let Piece := (Type, self.0);
let Compound := (Type, self.0, Compound);
let Compound := (Type, self.0, (Type, self.2.0));

let var = expression():type;
let total := fn(A: Type) -> {
    fn(x: A, y: A, z: A) -> {
        add(add(x, y), z)
    }
};

fn where self.Input = 
```

# Take 2?

## Principles:
1. No principle is perfect. Sometimes they are contradictory. They are helpful
   reference points, but any good decision made with them should, in their absense, remain equally good.
   - Example: `declare_a_variable_on_this_particular_line` is a better keyword
     for declaring variables, when 'better' is measured as 'how much it matches
     principle #4.'
2. Unification. Having multiple versions of the same idea in the same program leads to slower development and less flexibility.
    - Example: struct/tuple duality
    - Example: function/template duality.
    - Example: compiler/interpreter duality.
    - Caution: Too much of this principle can contradict principle #4,
      unification should not come at the cost of clarity.
3. Politeness. New features should have as small a surface area as possible.
   Leave room for other things to come in the future. Try to avoid adding
   special cases as much as possible, as these tend to combine multiplicitavely.
   A set of 6 features requiring 3 special cases apiece can result in 3**6=729
   special cases.
    - Example: The empty tuple type `()` already functions like `void`, so we do
      not need to make a seperate datatype for it.
    - Example: Curly braces in this language already work as a superset of
      parenthesis in most languages, so we can reserve the use of parenthesis
      exclusively for tuples - no special cases. This also helps with #4.
4. Write things out the long way. Code is read more than written. Abbreviated names and
   cryptic syntax features should be replaced with tooling and IDE autocomplete.
   Avoid the temptation to give structs tiny names. It's easy to feel like the
   name is obvious when you're writing it - learn to distrust that feeling.
   - Example: `function` instead of `def`
   - Example: `if cond a else b` instead of `cond ? a : b`.
   - Example: No `++counter` or `size--`. These are shorthands for adding
     specific statements above and below a particular line.

## Principles?
5. Provide options only when necessary. Options are good for experimentation,
   but bad for maintaining a cohesive ecosystem. When reasonable, only provide one way to do something.
   - Example: formatting is very strict, almost all files which differ after
     formatting contain differences in what they describe.

Tuples:
```
# The type of a tuple is a tuple of types.
typeof((1, 2, 3)) == (int, int, int)
# Structs are tuples with named elements.
(x: 1, y: 2).x == 1
# The two are interchangeable.
(x: 1, y: 2, "string").2 == "string"
# If numeric literals can be ambiguous:
u8 = 1
i32 = 5
# Then so can string/character literals
string = "a"
char = "a"
```

Modules:
```
The location of a file in the source tree defines what module it is in:
src/
  main.code # Module 'main'
  folder/
    .code  # Module folder? I don't know if I like this 
           # because it makes it hidden on *nix.
    _.code # Don't know if I like this either because 
           # it's very opaque, needs more #4.
    _module.code   # Module folder, and we can use the 
                   # _ syntax later for things like 
                   # var._type, var._size, whatever.
    something.code # Module folder.something
```

Syntax structures:
```
# Everything is an expression, like in Rust.
avalue ← 1 # Result is of type ()
avalue ← bvalue ← 1 # Should cause a warning

if cond a else b
a # 'bare expression', can only contain a few basic syntax features.
# Actually maybe not, maybe require a {} expression.

# If you want to do something like:
if cond if cond2 a2 else b2 else b
# Then that's ambiguous, you have to do it like this:
if cond {if cond2 a2 else b2} else b

# Braces and semicolons can be used to 
# compose multiple expressions together:
{
    avalue ← 1;
    bvalue ← 2;
    avalue + bvalue
}
```

Arrays:
```
# EVERYTHING IS TUPLES
array ← (1, 2, 3, 4)
element ← array.0   # Regular tuple syntax 'static item'
element ← array.{0} # Special array syntax 'dynamic item'
element ← array.{value + 1}
element ← array.value # Doesn't work, tries to get an item named 'value'
# Maybe
array._type = (int; 4)
# Or
array._type = (4*int)
# Multidim
array._type = ((int; 3); 5)
# Maybe
array._type = (3, 5) × int
```

Functions
```
value add_numbers ← function (first: i32, second: i32) => first + second;
```

Operators
```
value number ← 123
number ← number modulo 3
number apply modulo 3
number ⇄ modulo 3
number ⇄ square()
number ⇄ + 1
```

Misc?
```
value lt = start_lifetime();
# Alive -> Dead -> Dropped
lt is Alive;
value data = 12345;
data is Unborrowed == true everywhere;
value ref = borrow(data, for: lt);
# Error: cannot drop data
# -> drop(data) requires `data is unborrowed`
# -> But that can only be true when `lt is not alive`
# drop(data);
end_lifetime(lt);
```

We need functors!
```
Type # is a functor if it can take a function
     # fn(A) -> B and convert it to a function
     # fn(Type<A>) -> Type<B>.
```

We need monads!
```
```

We need math identities!
```
typeclass Add {
    const IDENTITY: Self;

    function add(a: Self, b: Self): Self;

    for (a: Self, b: Self) {
        guarantee Symmetry: add(a, b) ⇔ add(b, a);
        for (c: Self) {
            guarantee Associativity: add(a, add(b, c)) ⇔ add(add(a, b), c);
        }
    }
}
```
```
for (T: Type) {
    pattern Add {
        value IDENTITY: T;
        function add(a: T, b: T): T;
        for (a: T) {
            guarantee Identity: add(a, IDENTITY) ⇔ a
        }
        for (a: T, b: T) {
            guarantee Symmetry: add(a, b) ⇔ add(b, a);
        }
        for (a: T, b: T, c: T) {
            guarantee Associativity: add(a, add(b, c)) ⇔ add(add(a, b), c);
        }
    }
}
Add(i32) := {
    value IDENTITY := 0i32;
}
GuaranteedEquivalent 
    := function (T: Type) 
    := function (a: Expression(T), b: Expression(T))
    := newtype Unconstructable(_)((a, b));

GuaranteedEquivalent(T)(:{add(a, b)}, :{add(b, a)})
```

```
# Let's use angle brackets to indicate symbols/expressions:
<variable> # refers to the code 'variable', not the
           # value it computes.
<a + b> # refers to the code 'a + b', not the value it computes.
<a + b>: Expression
# This might not be consistent.
# $ evaluates the given expression, expecting it to return a value,
# which is then placed where $expr was originally.
b := 12
<$b> := <12>
piece := <a + b + c>
# Oh no!
<$piece + 12> := <<a + b + c> + 12>
# All good. Heh, maybe we could use a euro symbol for this
<eval($piece) + 12> := <eval(<a + b + c>) + 12> := <(a + b + c) + 12>
<€piece + 12> := <(a + b + c) + 12>
```

```
# Type checking everywhere!
1: Integer32;
(1 + 2): Integer32;
func(a, b): String;
func(a: Integer32, b: Float32): String;

# Defining something should work by example.
make a: Integer32 <=> 12;
# Compiler can complain if you leave out type information it wants.
make return_number() <=> 123;
# Or if you give it too complicated an expression.
make a + b <=> 34;
```

```
require a._type <=> (Integer, Integer)
```

```
# Maybe this is useful for 'templates'?
make test_template(A: Type, B: Type): Expression <=> {
    <
        make Pair <=> ($A, $B);
        make pair(a: $A, b: $B) <=> {

        }
    >
}
# The compiler can analyze that and be like, "Oh, if I need a type (X, Y), then
# I can get it by calling this template with (X, Y)."
```

```
# Patterns?
make functor(T: Type): Expression <=> <
    make map_template(U: Type): Expression <=> <
        exist <map(a: $T)>
    >
>
```

```
borrow(data, for: Lifetime)
    if for is alive
    ->
    data._unborrowed ⇄ and !for._alive

end_lifetime(Lifetime)
    if argument is alive
    -> 
    argument._alive ← false
```

Lisp 2.0?
```
(
    avalue ← 1,
    bvalue ← 2,
    avalue + bvalue
)
```

But then
```
if condition (
    thing1,
    thing2,
    result
)
```

You could
```
if condition (
    thing1,
    thing2,
    result
).last
```
but this is now a special case and overunification, I don't like it.

# Take 3? More like rambling.

```
# What does this represent?
1
# It could represent the literal data '1', or it could 
# represent a memory location or register which will
# have the value `1` at runtime...

variable
# This usually represents a memory location to the
# compiler, and data contained in that memory location
# during runtime.

# Given the expression:
variable
# Compiler is like:
# 'oh that's memory location 12345 and it's an integer',
# that's what the code `variable` means to the compiler.
# While at runtime it's like
# 0b10100010101010011110001010001010010001001111
# Could we go in the other direction? Add another layer
# of abstraction on top of the compiler?
# 'oh that's symbol 12345 and it's a Symbol<integer>'

# So we have layers like this:
# variable represents an integer, 42
# @variable represents a Symbol<integer>, id <opaque id>
# @@variable represents a Symbol<Symbol<integer>>, id <opaque id>
# @a + @b represents some instance of writing a + b
# a + b represents some instance of adding a's value and b's value

# a + b is a piece of code
# at runtime, it becomes the computation of adding a's value and b's value

# a + b represents the computation 
# of adding the result of a and the result of b.
# At runtime, this computation is performed.

# a + b is a description of a computation.
# During runtime is when this computation actually happens.
# but (a + b) + c is a description of a computation 
# involving the *result* of a + b
```

A language lets you write a description of a computation and then perform that
computation. There is a direct relationship between the compiler holding the
description of a computation and the actual computation that is performed.
Executing the program uses one form to produce another. (The compiler is just
translating from one description language to another, its job does not involve
lowering.)

What would meta-execution look like? You have a meta-description of a
description. When the meta-description is executed, it performs the description
defined by the meta-description?

Execution turns instructions into operations.

Meta-execution turns meta-instructions into instructions?

My source code:
```
a = make_symbol()
b = make_symbol()
# Denote meta-addition, I.E. an operation which represents 
# writing addition between two symbols.
write(a @+ b)
```
Executing it produces:
```
sym_123 + sym_456
```
We could also turn this around to describe compile-time constants:
```
# This no longer denotes a computation that will happen at runtime.
# It denotes the literal value 79.
$37 $+ $42
```

Maybe use the remaining two brackets to describe raising/lowering code?

```
<42> computes the value 42 at compile time
variable = [a + b] would assign the expression "a + b" 
           to variable at runtime.
```

I think not, because writing:
```
1 + 2
```
should always be equivalent to writing:
```
3
```
And the compiler should be able to detect that without being explicitly told to
execute an expression.

Now for defining things:
```
+ is a thing 
    such that (T, +) forms an abelian group
    such that +(T, AdditiveIdentity()) <=> T
    such that +(AdditiveIdentity(), T) <=> T
    +(AdditiveIdentity(), T) <=> T can be proved 
        given +(T, AdditiveIdentity()) <=> T
        by [abelian group property]
    +(T, AdditiveIdentity()) <=> T can be proved 
        given +(AdditiveIdentity(), T) <=> T
        by [abelian group property]
    such that +(T, -(T)) <=> AdditiveIdentity()
```
You've successfully defined `+` for `T` if all of the "such that"s are true, and
the author of the thing can provide ways to derive some of them from others, so
that you don't need to fully specify everything.

We have a few things going on here:
1. A thing is defined, presumably it has a type.
2. Requirements are defined.
3. Theorems are defined to translate between different requirements.
   
Maybe what's special about defining the thing is that it's where you can give
requirements without having to prove them from other requirements.

That might cause this though:

```
+ is a thing
    such that 1 = 0
```

But that kind of makes sense, it means you can never define + because the
compiler won't let you define + until you've proved 1 = 0.

Maybe it should be more like this:
```
+ is a set of requirements
    require 1 = 0
```
```
abelian_group(T, Op, Ident, Neg) is a set of requirements
    require T Op Ident <=> T
```
So you're not 'defining' something, you're proving a set of requirements.

How does Haskell handle getting the current system time? Looks like it uses IO.
It looks like this type is more general than the name suggests. I think `Action`
would be more fitting. The role of compilation would then be to take an `Action`
and produce machine code which performs that `Action`. Functional source code is
an expression of type `Action`.

I don't like how Haskell's function call syntax can look like a string of words, but
I like the elegant ideas it's built on. Maybe a syntax like this?
```
square@5 <=> 25
difference@(3, 1) <=> 2
```
Or maybe:
```
square of 5 <=> 25
```
What function would go against this grammatical intuition?
```
seed of 0x19501 <=> Action
printString of "alskdjlkadsjf" <=> Action
```
Maybe that's a matter of changing the name of the function to be more accurate?
```
rngSeeder of 0x19501 <=> Action
stringPrinter of "alskdjflakjsdlfk" <=> Action
```
What do we do about something like random generators wanting to set up something
before they can be used? I think this question plays into how to make `Action`
generic enough to be extended in a variety of ways so that all its functionality
does not have to be given by a single author.

We could make `Action` kind of 'pluggable':
```
Action has
    MemoryManager
    IoHandler
    RandomGenerator<IsSetUp=false>
```
And so setting up the RNG would produce an `Action` where `IsSetUp=true`.

This is like storing compile time constants.

Maybe instead, `is_set_up` would be a field on `RandomGenerator`, and to use RNG
we need to ensure that `is_set_up == true`. But wait, this is side effects now.
But actually maybe not?

```
fn (rng: RandomGenerator where is_set_up == false) 
    -> RandomGenerator where is_set_up == true
```

`is_set_up` remains unchanged on the original expression, it is only the
expression formed by applying the function that has the different type.

We need some way to panic, because it's impractical to ask a programmer to
explicitly denote that any function they write could produce an error. For
example:
```
fn sum(a: Int, b: Int) -> Int
```
Could fail if `a+b` overflows. These kinds of things could be either compile
time or runtime errors depending on when the actual computation happens. It
shouldn't violate side effects to have this capability.

It might be nice to show operator associativity by subtly highlighting the
background of different levels of precedence in the editor

Types as values? Each expression performs some operation on the data as well as
the type, the idea being the operation on the type is fully elaborated at
compile time. But Haskell tells us that a lot more than that can happen at
compile time, in general the value of any expression can be computed as long as
it does not monadically depend on external factors:
```
2 + 2 # Gets evaluated to 4, the data type is Int "+" Int = Int
```
So maybe it's more accurate to describe an Int like:
```
(data_type: Int, value: 4)
(data_type: Int, value: 2) + (data_type: Int, value: 2) = (data_type: Int, value: 4)
```
But this strikes me as unhelpfully circular. What's the datatype of each struct?
Maybe it's not a relevant question. But then how should the compiler know the
size of the second field? Well, it could find it by the value of the first
field. But then what does '2' represent? There we go, there's a problem here:
```
(data_type: (data_type: ..., value: (data_type: ..., value: ...)), value: ...)
```
This representation is infinitely large.

Okay, so maybe every expression has some 'second-order data' that places a
constraint on its 'first-order data'. When you apply some function to a piece of
data, it modifies both kinds of data.
```
4 # first-order data: the specific value 4
  # second-order data: it's an integer
```
This seems an interesting direction to explore in because generalizing to higher
orders may provide something useful.

When you write a function, the compiler can determine the second-order data
being produced without knowing what the first-order data will be.

And if you have a function where the second-order data isn't known, that's a
TEMPLATE! So third-order data is TRAITS!

Something you want to specify in third-order data is that two values with
second-order data following this third-order data can be added:
```
trait Add<T> {
    fn add(a: T, b: T) -> T;
}
```
How can this concept be abstracted to all orders? Or maybe, what *exactly* does
the third-order data that tells us two things can be added look like? I don't
quite like the idea of tieing traits to be defined on individual types, but it
may not be all that bad. I know that Rust's usage of this system has limitations
though, like why should the Add trait be defined on the type that is the lhs of
the operation? Shouldn't the concept of 'addition' not be associated with one
and only one of the involved types? But let's do it like that for now:
```
4 is an integer

meaning we can define functions that work for 
all integers and it will work for 4

integer is an add

meaning we can define functions that work for 
all addables and it will work for all integers
```
Maybe the concept of strict orders isn't quite right? Like if we make some other
trait so that `all addables are xyz`, then `xyz` shouldn't have to be *fourth*
order data. Maybe just:
```
all instances of 4 are instances of integers
all instances of integers are instances of addables
```
But then I think this runs into practical issues:
```
# a, b can be different instances of integers.
fn add_ints(a: integer, b: integer) -> integer;
# a, b need to be the same type, but can be different instances of that type.
fn add_any(a: addable, b: addable) -> addable;
```
What we really need is something like this:
```
fn some_weird_thing(t: addable, a: t, b: t) -> t
```
So how do we define the actual implementation of addables? Like
```
integer is addable because
fn add(a: integer, b: integer) -> integer;
```
The second-order data `integer` doesn't directly describe the functions that an
integer can be used in, that's described by other functions.

A thing:
```
publish
translation_of { 
            data_blitter_of, 
    brevity=Tiny,
         ...defaults 
} 
as blitr

publish
data_blitter_of {
    =target :Type, 
    =shader :Type, 
    =data   :Type, 
    =scale  :Type, 
} :Action<{}>
is unordered_sequence_of pixel_group_setters 
where (
   pixel_group_setters 
   is square_grid_pattern_where {
            size=TILE_SIZE, 
       generator=pixel_group_setter_at
   };
   pixel_group_setter_at { x=data_x, y=data_y }
   is unordered_sequence_of pixel_setters
   where ( 
      pixel_setters
      is square_grid_pattern_where {
              size=scale, 
         generator=pixel_setter_at
      };
      pixel_setter_at { x=x_in_group, y=y_in_group }
      is mutator_with { target=pixel, value= }
      where (
         pixel 
         is mutable_pixel_reference_at {
             =target, 
            x=pixel_x, 
            y=pixel_y
         }
         where (
            pixel_x
            is data_x * scale + x_in_group;
            pixel_y
            is data_y * scale + y_in_group;
         );
      );
   );
);
```

So lean has some really nice ideas. It already has the idea of implicit
arguments, which replaces template parameters. Something like this:
```
declare do_something { implicit =T :Type, =value :T } :T
is value
declare something_done :Integer
is do_something { value=0 }
```
But then it also extends this to include type classes, the idea being you can do
something like this:
```
declare sum { implicit =T :Type, implicit =Add :Add{T}, a :T, b :T } :T
is Add.add { a, b }
```
Type classes are *really* similar to structs and I want to find a way to keep
them the same thing.
```lean
-- In lean...
class Add (a : Type) where
    add : a -> a -> a
```
Maybe we can talk about 'strategies', like telling the compiler 'here is how to
go from some function inputs to some outputs'. But that's basically just a
function. What makes it special is that the compiler knows about it? And we
would also have to allow for defining them piecewise, which I don't like.

Okay, how about this:
```
define AddImplementation
is type {
      T = Type,
    add = {T, T} -> T,
}

class Add AddImplementation 
unique_over { T= }

# Or...
class Add {
      T = Type,
    add = {T, T} -> T,
}
unique_over { T= }

define _: implementation of Add { T=MyCustomType }
is {
    add=my_custom_addition,
}

define add_for_integers { 
    T= :Type, 
    _ is implementation of Integer { T= } 
} :implementation of Add { T= }
is {
    add = {T, T} => T + T,
}
```
And the compiler will require that all definitions of `Add{T=}` are equivalent.

Also, backwards DTT? And it would be useful to have `implicit` be a general
property of struct types, and have that behavior be useful in the context of
functions instead of being a function-specific thing
```
define MyStruct
is type {
   implicit T=Type is field._type,
        field,
}
```
It would be useful if we have constraints:
```
define MyStruct
is type {
   implicit           T=Type is field._type,
   implicit upper_bound      is field._upper_bound,
                  field,
}
```
As opposed to:
```
define MyStruct
is type {
    implicit T=Type,
    implicit upper_bound=T,
    field=T where field <= upper_bound,
}
```
But maybe we're going about this wrong:
```
define MyStruct
is type {
    implicit T=Type,
    implicit upper_bound=T,
    field=T,
    field_upper_bound = Proof that field < upper_bound
}
```
This way we don't have to attach any extra baggage to `field` itself.

How would we write an addition function where we can also prove things about the
result?
```
define some_theorem {
    T=   :implicit Type,
    a=   :implicit Expression,
    al=  :implicit T,
    b=   :implicit Expression,
    bl=  :implicit T,
    alp= :Proof that $a <= $al,
    blp= :Proof that $b <= $bl,
} :Proof that $a + $b <= $(al + bl)
is axiom

with array :{Integer; 20} (
   define get_element {
       T= :implicit Type, 
       a= :T, 
       b= :T,
       _  :implicit Proof that a + b < 20,
   }
   is array.(a + b)
   where (
   )
)
```

How does Lean associate theorems with typeclasses? E.G. ring theorems with
rings?

Propositions are types so the theorems are fields in the typeclass, dependent on
the previous  values in the typeclass.

Propositions and typeclasses both have this property that there is at most one
unique value of that type. For propositions, all propositions of the same
statement are equivalent. For typeclasses, you don't want an instance of the
typeclass to have multiple conflicting definitions.

Functions are the same thing as expressions with bound variables. Maybe a 'with'
statement can be used to introduce implicit variables?
```
with {=T :Type} (
    define identity (value :T)
    is value
)
identity of 3
-- 
```
`of` can serve the role of `$` in Haskell.

Maybe it would just be better to be curried by default:
```
define thing a :A b :B

thing b=asdf a=fdsa
```
But then where does the return type go?

```
define thing {=a :A, =b :B} :RT
is whatever {=a} + whatever {=b}
```

Types where all values must be equal should be called 'Unary'.

Use 'newtype' instead of struct, class, type, etc:
```
newtype Thing
is Integer
```
And it works like define:
```
define thing :Thing
is 123
```
Or maybe it's a function:
```
define MyType
is newtype Integer
```
Although this has problems because:
```
define A
is newtype Integer
define B
is newtype Integer
A = B
```
Maybe it should accept the code location as an argument:
```
define A
is newtype (implicit code location) Integer
```

Instead of having functions, have expressions with bound variables:
```
define square
is number * number
with (number :Number)
```
```
define manhattan_distance
is abs dx + abs dy
with (dx :Number; dy: Number)

define example
is manhattan_distance 
where (
  dx is 12; 
  dy is -32;
)
```
This would also mean that functions without arguments are idential to plain
expressions, which plays nicely with the idea of functions being pure.

'where' can still be used to define values without using 'with':
```
define example
is one + two + three
where (
  one is 1;
  two is 2;
  three is 3;
)
```
Bound variables can be implicit. Usually this means they will be inferred based
on their usage in defining the types of other bound variables.
```
define sum
is a + b
where (
  implicit T :Type;
  :AddGroup T;
  a :T;
  b :T;
)
```
Unary types are implicit in special ways. Since all values are equal, they will 
automatically be passed from parent scopes:
```
define sum3
is sum where (a is values.0; b is sum where (a is values.1; b is values.2))
where (
  implicit T :Type;
  :AddGroup T;
  values :{T, T, T};
)
```
(Maybe we can say they are 'anonymous'?) In this case, the AddGroup argument to
`sum` is implicitly the value of the AddGroup argument to `sum3`. In general,
whenever there is a value of a unary type in scope, it will implicitly be passed
to expressions which need it.

A special kind of implicit argument would be `CallingContext`, which would
contain information about where an expression is being called and the values of
all bound variables (including ones unrelated to the specific expression in
question):
```
define newtype
is COMPILER_INTERNAL
with (
  implicit location :CallingContext;
  BaseType :Type;
)
```
This would also enable the creation of useful debug messages:
```
with (T :Type; :HasZero T) (
  define error_example
  is zero / zero
)
error_example Integer
```
```
ERROR: division by zero
  at project.example:3:6 where (
    T is Integer; 
    anonymous (HasZero Integer) at lang.core.has_zero:110:4;
  )
  at project.example:5:1
  at project:generated where (
    some_global_value is something;
    whatever is another_thing;
  )
```

If an expression only has a single variable that needs binding, a shorthand
syntax can be used. I.E., the following are equivalent:
```
sine where (angle is 0.123)
```
```
sine 0.123
```

"with" and "where" should also be usable as statements:
```
with (T :Type) (
  define Default
  is unary_newtype {
    default is T
  };
)
define Default
is unary_newtype { default is T }
with (T :Type)
```
```
where (one is 1) (
  define two
  is one + one;
)
define two
is one + one
where (one is 1);
```
Do we actually need 'define'? How should visibility be managed?
```
namespace some_namespace (
  thing1 is 123
  thing2 is 456
)
```
The generated root of a project should work like an expression. Maybe
'namespace' should work like an expression?
```
some_namespace is namespace (
  thing1 is 123
  thing2 is 456
)
some_namespace.thing1
```
How is this different from a struct? I'm not sure it's different from a struct.
```
some_namespace is {
  public thing1 is 123;
  public thing2 is 456;
}
```
Fields in a struct literal should be private by default but coercable to public
fields. Specifying a public visibility will cause it to not be coercable to 
private.

Should `with` and `where` accept struct literals then?
```
public thing 
is one + one
where {
  one is 1;
};
```
That would unify things nicely I think, it would also draw a direct link between
anonymous values and unnamed fields:
```
public thing
is one + one
with {
  T is Type;
  AddGroup T;
  HasOne T;
}
```
But then how do we mark things implicit? Answer: we make it a feature of structs.
```
public MyTemplateType
is newtype {
  implicit T is Type;
  value is T;
}
```
Will this cause redundancy?
```
public MyTemplateType
is newtype {
  value is T;
}
with {
  T is Type;
}
```
Answer: no, because 1) the context of an expression can be represented as a
struct, and 2) using implicit inside a type definition and inside a with
statement convey two different things?

2) yes, implicit inside `with` only conveys implicitness in relation to other
fields declared inside the `with` clause. If you have a type declared like this:
```
public ExampleType {
  implicit T is Type;
  value is T;
}
```
Then you can only refer to `ExampleType` as the type of something else:
```
with (value is ExampleType) {...}
```
Whereas using it in `with` allows specifying its exact value:
```
public ExampleType {
  value is T;
} with {
  T is Type;
}
```
```
with (value is ExampleType Integer) {...}
```
However, this strikes me as another kind of duplication. Could we have it so
that we can specify the value of `T` in the former case, making the latter
syntax unnecessary?

Maybe we can go in the opposite direction, I.E. expressions which will
eventually resolve to a type are themselves types.
```
Example is newtype {}
where { T is Type; };

Example :Type
Example Integer :Type
```
I guess that means that `typeof(Example Integer) = Example`? That would mean
that `typeof(Example Integer) != Type`, although this has the smell of something
possibly elegant if I chase down all its implications.

I can have a type `MyType with (a is Integer; b is Integer)`. A value `MyType
where (a is 123; b is 456)` would have a type of `MyType where (a is 123; b is
456)`?

```
MyType where (a is 123) is_in MyType
MyType where (b is 456) is_in MyType
MyType where (b is 456) is MyType where (b is 456)
MyType where (a is 123; b is 456) is_in MyType where (a is 123; b is 456)
MyType where (a is 123; b is 456) is MyType where (a is 123; b is 456)
MyType where (a is 123; b is 456) is_in MyType where (a is 123)
MyType where (a is 123; b is 456) is_in MyType where (b is 456)
```

A struct type `{a is A, b is B}` would just be sugar for:
```
with (a is A, b is B) {
  TheType is newtype
}
```
But then what's the type of that struct literal?
```
unit with (a is A, b is B, TheType is Type)
```
How about raw struct literals are all based on the unit value.
```
{} = unit
{ a is 123 } = unit where (a is 123)
```
But wait, that leads to problems if we want to keep using struct literals for
with/where:
```
{ a is 123 } = unit where { a is 123 }
```
Instead of making structs sugar for `with/where`, I would prefer making
`with/where` sugar for structs.

Every expression has 'context', a struct of bound values. Every expression has
'variables', a struct type of bound variables. Example:
```
with { a is Integer; b is String } {
  something is a + b;
};
```
`something where (a is 123)` would have a context of `{ a is 123 }` and variables
`{ b is String }`.

`with` and `where` introduce values and variables into an expression's context:
```
(expr)
(expr where { some_value is 123 })
(where { some_value is 123 } expr)
(expr with { some_value is Integer })
(with { some_value is Integer } expr)
(with { some_value is Integer } expr where { some_other_value is "HI" })
```
Maybe structs should just be sugar for with/where:
```
{ a is 123 } = unit where (a is 123)
```
But then we have this unnecessariness:
```
my_function
is { a is value }
with (value is Integer);

(my_function 123).value = (my_function 123).a
```
And then, what exactly *is* a type? As in, if I want some value where I can do:
```
some_value.a + 123
```
how do I do that? Like this?:
```
(some_value.a + 123)
with (
  some_value is unit with ( a is Integer )
)
```
So then a type would be like this:
```
MyType
is unit with (a is Integer)
```
Explicit consumption of context with newtype?
```
with (A is Type; B is Type; C is Type) {
  MyUnaryType
  is unary_newtype { here; A; B }
  with (some_field is A);
}
```
```
if SomeType A isa SomeType
then (unit where (A is A)) isa unit
So what would (unit with (A isa Something)) be?
Well ((unit with (A isa Something) where (A is A)) isa unit

So should we have (unit isa (unit with (A isa Something))?
or ((unit with (A isa Something)) isa unit)?
```
What if a 'type' is just an expression with bound variables for the runtime data?
e.g.:
```
MyWrapperType
is ...
with (T isa Type; data isa T);
(MyWrapperType Integer) has a bound variable 'data' and a bound value 'T' which is Integer
```
So basically we have it so that a type is a statement of what kind of expression
the data should be generated with. Instead of `isa`, it would be more accurate
to say `from`.
```
my_wrapper
is ...
with (T from unit; data from T)
```
So if we have:
```
my_type
is ...
with (x from Integer; y from Integer);
```
And then:
```
(my_wrapper where (T is my_type))
```
Then `data` has to be from `my_type`, so we know it will have `x` and `y`.
```
wrapped_type
is my_wrapper where (T is my_type);
example_value
is wrapped_type (my_type where (x is 123; y is 456));
example_value.data.x?
example_value.data.y?
```
But then what if we use something where only one of the arguments is supplied?
```
partial
is my_type where (x is 123);
assert partial is_from my_type;
what_happens
is my_wrapper where (
  T is my_type;
  data is partial;
);
```
And what do we do when we *want* to accept functions, when we *want* to have
values that we can plug in later?
```
function_holder_type
is ...
where (
  From from Type;
  To from Type;
  conversion_func from ???;
)
```
With 'isa' this would be simple:
```
function_holder_type
is ...
where (
  From isa Type;
  To isa Type;
  conversion_func isa To where (from isa From); 
)
```
And then we could have visibility to have structs make sense:
```
my_struct
is unit
where (
  intermediate is 123;
  public field1 is intermediate;
  public field2 is "hello";
)
```
What if we go back to structs being the fundamental concept?
```
{
  a isa Number;
  b is a + a;
}
```
`where` acts to combine two structs:
```
({ a is 123 } where { b is 456 }) = { a is 123; b is 456 }
```
Wait no, it should make the fields of a struct available in an expression:
```
(a + a where { a is 123 }) = 246
```
Any expression's "context" is a struct. `where` appends a given struct to an
expression's context. A struct can have fields whose values aren't specified:
```
{
  a isa Number;
  b is a + a;
}
```
`with` concatenates two structs. `where` is equivalent to
`(expr's new context) <- (expr's old context) with rhs`
```
({ a isa Number; b is a + a } with { a is 123 }) = { a is 123; b is 246 }
```
```
((a + a where { a isa Number }) where { a is 123 }) = 246
```
Every field in a struct has the context of the rest of the fields in that struct.

But wait, this is just `where/with` as the fundamental thing again.

A context/struct is a string of `where` expressions:
```
(unit where a isa Number where public b is a + a where a is 123).b = 246
```

What if structs have no special syntax and are just literally collections of items:
```
({ field is a } where (a isa Number)).field = (a where (a isa Number))
```
Then a context's variables are specified by a struct of types and a context's
values are specified by a struct of values. `with` appends to the variables,
`where` appends to the values (potentially removing from the variables.)

I think it's then impossible to make a struct that is dependant on a prior
field, only one that is dependant on values in its context. But wait, it might
be nice to have a value which is computed from another value which is computed
from a variable. Let's start with just a value dependent on a variable.
```
quadruple
is double + double
with {
  single is Number
}
where {
  double is single + single
}
```
Internally we might want this to look like:
```
quadruple: double + double
  variables:
    single: Number
  values:
    double: single + single
```
```
octupled
is quadrupled + quadrupled
with_variables {
  single is Number
}
with_values {
  quadrupled 
  is doubled + doubled
  with_values {
    doubled is single + single
  };
};

demo
is octupled
where {
  single is 123;
};

check_that demo = 123 * 8;
```

Okay, what would some basic programs look like?

```
encapsulated_in_parent

main 
is string_printer "Hello, world!";

verbose_main 
is string_printer 
where {
  string_to_print is "Hello, world!";
};
```

```
main
is string_printer "Enter a string >"
  then input_line_reader
  then with_variables { input is String } (
    string_printer "You entered: "
      then string_printer input
  )
;
```

How should we represent the types of expressions which still have unresolved
variables?
```
(a with_variables { a is Number }): (Number with_variables { a is Number })
```
Is this notation too much?
```
(default with_variables { T is Type; Default T }): (T with_variables { T is Type; Default T })
```
no.

Wait a minute, we need recursion (or looping) to describe a bunch of things.
```
repeat_until_condition where {
  initial_value is 0;
  
}
```


Okay, maybe `where` should allow writing expressions that need to be true.
```
some_value
where (
  some_value is 123;
)
```
```
some_value
where (
  T isa Type;
  default_impl isa Default T;
  some_value is default_impl.default_value
)
```

It's dependant types again!
```
{
  T;
  the Proof T isa Type;
  default_impl;
  the Proof default_impl isa Default T;
  some_value;
  the Proof some_value is default_impl.default_value;
}
```
But wait, we would want the proofs to exist as parameterized over the values?

Or maybe we're told that, if we have a value of that type, then whatever the
values of the other fields, we must also necessarily have the proofs.

Maybe proofs are more baked into the language
```
{
  T; default_impl; some_value;
} guaranteeing {
  self.T isa Type;
  self.default_impl isa Default T;
  self.some_value is default_impl.default_value;
}
```
```
Integer guaranteeing self >= 0
```
```
unit guaranteeing { self isa Integer; self >= 0 }
```
Types are collections of guarantees?


Okay, what if we want to define something recursively?
```
recursive_thing
is (
  recursive_thing where { argument is argument - 1 }
)
where { argument isa Integer }
```
```
recursive_thing: {
  vars: {
    argument: Integer
  },
  values: {},
  value: where {
    expr: use_value recursive_thing,
    var_binds: {
      argument: binary_op (use_var argument) (-) (int_literal 1)
    }
  }
}
```
What bugs me about it is like this:
```
recursive_thing
is recursive_thing
where {
  argument isa Integer;
  argument is argument - 1;
}
```
I think the ambiguity is that `where` can be used both to define values that are
available in an expression, and to bind to variables of an expression.

What if we did something like `with` introduces values and variables, while
`where` binds variables to specific values?
```
recursive_thing
is recursive_thing where { argument is argument - 1; }
with { argument isa Integer; }
```
Is there something more descriptive we could call these?

Maybe `specifying` and `where`?
```
recursive_thing
is recursive_thing specifying { argument is argument - 1; }
where { argument isa Integer; }
```
I like that. It can be nicely abbreviated as `sp` and `wh`.

VERBOSITY

Should be changeable.
```
rover set_verbosity 3
rover verbosity 2
rer v 1
rer v
rer verbosity 3
rer vvv
```
Maybe running a command should show you the standard and fully abbreviated
forms?
```
$ rer v 3
> rover verbosity 3 (abbr: rer vvv)
(Command output...)
```

Let's make some tutorials! 
```
main
is string_printer specifying (
  string_to_print is "Hello, world!";
);
```
```
main
is sprinter "Hello, world!"
```

```
main
is string_printer "Enter some text > "
then line_reader
then line => (
  string_printer "You entered: "
  then string_printer line
)
;
```

```
main
is string_printer "Enter some text > "
then line_reader
then line =>
string_printer "You entered: "
then string_printer line
;
```
```
main
is string_printer "Enter some text > "
then line_reader
then where { line isa String; }
string_printer "You entered: "
then string_printer line
;
```
```
main
is 
(
  (
    string_printer
    specifying 
    (
      string_to_print 
      is 
      "Enter some text > "
      ;
    )
  ) 
  then 
  (
    where 
    ( 
      unused 
      isa 
      (
        structure_type {}
      )
      ;
    ) 
    (
      line_reader
    )
  )
)
then 
(
  where 
  ( 
    line 
    isa 
    String
    ; 
  ) 
  (
    (
      string_printer 
      specifying 
      (
        string_to_print 
        is 
        "You entered: "
        ;
      )
    )
    then 
    (
      where 
      ( 
        unused 
        isa 
        (
          structure_type {}
        )
        ; 
      )
      (
        string_printer
        specifying 
        (
          string_to_print 
          is 
          line
          ;
        )
      )
    )
  )
)
```
less ew than before.

Maybe we need four levels of verbosity? The above would be level four, this
would be three:
```
main
is 
(
  (
    string_printer specifying (
      string_to_print is ("Enter some text > ");
    )
  )
  then line_reader
)
then 
(
  where ( line isa String; ) (
    (
      string_printer specifying (
        string_to_print is "You entered: "
      )
    )
    then 
    (
      string_printer specifying (
        string_to_print is line
      )
    )
  )
)
```
```
something
is [1, 2, 3]
then [4, 5, 6]
then x => # x will be [4, 5, 6] 3 times
;

something 
is [1, 2, 3]
then (
  [4, 5, 6]
  then x => # x will be 4, then 5, then 6
)
```
```
number_asker
is string_printer "Enter a number: "
then line_reader
then line =>
if line.is_integer
  then (resulter line)
  else (
    string_printer "That is not a number.\n"
    then number_asker
  )
```
Types!
```
structure_type {
  x isa Number;
  y isa Number;
}
;
Opt aka Optional
is where (T isa Type;)
enumerated_type {
  Just are T;
  Nothing are {};
}
```
```
number_asker
is string_printer "Enter a number: "
  then line_reader
  then line =>
  cases line.parsed (
  when Success: number => result_action number;
  when Failure: 
    string_printer "That is not a number.\n"
      then number_asker;
  )
```
```
retry_action
is action
  then result => cases result (
    when Success: value => result_action value;
    when Failure: error => error_action error;
  )
  where (
    SuccessType isa Type;
    ErrorType isa Type;
    ActionResult is Fallible where ( 
      SuccessType is SuccessType; 
      ErrorType is ErrorType; 
    );
    action isa Action ActionResult;

    ErrorResult isa Type;
    error_action isa Action ErrorResult;
  )
```
It would be nice if we could represent that something like this:
```
retry_action
is action
  then result => cases result (
    when Success: value => result_action value;
    when Failure: error => error_action error;
  )
  where (
    action isa Action (any Failable);
    error_action isa (any Action);
  )
```
Or maybe just inject magic arguments whenever you use a type which still has
unspecified values.
```
retry_action
is action
  then result => cases result (
    when Success: value => result_action value;
    when Failure: error => error_action error;
  )
  where (
    automatic_var_1 isa Type;
    automatic_var_2 isa Type;
    automatic_var_3 isa Type;
    action isa Action SuccessOrFailure specifying ( SuccessType is automatic_var_1; FailureType is automatic_var_2; );
    error_action isa Action automatic_var_3;
  )
```
Could then just be written as:
```
retry_action
is action
  then result => cases result (
    when Success: value => result_action value;
    when Failure: error => error_action error;
  )
where (
  action isa Action SuccessOrFaiure;
  error_action isa Action;
);
```
Might be able to do:
```
retry_action
is action
  then result => cases result (
    when Success: value => result_action value;
    when Failure: error => error_action error;
  )
where (
  action isa FailableAction
  where (
    FailableAction is Action SuccessOrFailure
  );
  error_action isa Action;
);
```
If we define:
```
FailableAction
is Action SuccessOrFailure;
```
That's equivalent to:
```
FailableAction
is Action (SuccessOrFailure specifying ( SuccessType is hv0; FailureType is hv1; ))
where (
  hv0 isa Type;
  hv1 isa Type;
)
```
Wait a minute, this is just variable propogation!
```
a is b;

b is (some_value isa Integer) => some_value;

#>show_vars a
some_value isa Integer
  > From usage of 'b' at /my/file.rover:1:5
  > From 'some_value isa Integer' at /my/file.rover:3:7
```
```
personalized_greeting
is ( 
  greeting isa String; 
  person isa Person; 
) => format( 
  "{}, {}!"; 
  { greeting; person.first_name; }; 
)

# Vars: greeting isa String; array_length isa 
greet_people
is (
  people isa Array( People; );
) => map( people; personalized_greeting; )
```
```
Never is enumerated_type {}
```
```
Guarantee is (
  proposition isa Boolean
) => unary_newtype(
  here; 
  if proposition (Unit) else (Never);
);
```
This is perfect! We can never create a value of `Guarantee(false)`, so if we
have a value of `Guarantee(arbitrary_expression)`, it means that expression must
be true!

What this really needs is support from the compiler for reasoning about types
using unspecified values.
```
(T: Type) => (
  reflexive
  is (a: T) => axiom: Guarantee(a = a)
)
```
```
readnum_lt10_a aka read_number_less_than_10_action 
is retry_action(
  read_number_action map checked_number;
  error_response is print_string_action("That is not less than 10");
)
where (
  read_number_action is retry_action(
      print_string_action("Enter a number less than 10:")
        then line_reader
        map parsed_line;
      error_response is print_string_action("That is not a valid number");
    )
  where (
    parsed_line is line => line.parse a UnsignedInteger32;
  );
  checked_number 
  is number => if number < 10 (
      Success({number; the Guarantee(number < 10)})
    ) else (
      Failure(void)
    );
);
```
```
the_cumbersome_to_define_thing
is mrepeat(
    start is atry;
    on_error is anotify_error mmap atry;
  )
where (
  atry is mrepeat(start is atry; on_error is anotify_error atry)
    fmap number => if number < 10 (
        Success({number; the Guarantee(number < 10)})
      ) else (
        Failure(void)
      )
  where (
    atry is aprints("Enter a number less than 10:")
      mmap aread_line
      fmap parse(Integer);
    anotify_error is aprints("That is not a valid number")
  );
  anotify_error is aprints("That is not a valid number");
)
```
```
the_thing is aretry(
  aretry(
      aprints("Enter a number less than 10:")
        mmap aread_line
        fmap parse(U32);
      on_error is aprints("That is not a valid number");
    ) 
    fmap number => if number < 10 (
      Success({number; the Guarantee(number < 10)})
    ) else (
      Failure({number; the Guarantee(not(number < 10))})
    );
  on_error is aprints("That is not less than 10.");
);
```
```
get_number_lt10 is 
aretry(
  agetnum fmap check_condition(x => x < 10);
  on_error is aprintln("That is not less than 10.");
)
where (
  agetnum is 
  aretry(
    aprint("Enter a number less than 10: ") mmap aparseln(U32);
    on_error is aprintln("That is not a valid number");
  );
);
```
```c++
unsigned int get_number_lt10() {
  while (true) {
    unsigned int result;
    std::cout << "Enter a number less than 10:";
    std::cin >> result;
    if (cin.bad()) {
      std::cout << "That is not a valid number." << std::endl;
    } else if (result < 10) {
      return result;
    } else {
      std::cout << "That is not less than 10." << std::endl;
    }
  }
}
```
```rust
fn get_number_lt10() -> u32 {
  loop {
    print!("Enter a number less than 10:");
    match input().parse() {
      Ok(value) => if value < 10 {
        return value;
      } else {
        println!("That is not less than 10.");
      },
      Err(..) => println!("That is not a valid number"),
    }
  }
}
```
```
check_condition is
vars (condition isa Boolean)
if condition (
  Success(the Guarantee(condition))
) else (
  Failure(the Guarantee(not(condition)))
)
a SuccessOrFailure(
  SuccessType is Guarantee(condition);
  FailureType is Guarantee(not(condition));
)

check_value is
vars (
  T isa Type;
  the_value isa T;
  condition isa Boolean after_vars (value isa T);
)
if meets_condition (
  Success({value is the_value; the Guarantee(condition(value))})
) else (
  Failure({value is the_value; the Guarantee(not condition(value))})
)
a SuccessOrFailure(
  SuccessType is st {value isa T; Guarantee(condition(value))};
  FailureType is st {value isa T; Guarantee(not condition(value))};
)
```
Lifting:
```
my_expr a Integer after_vars (Integer; String);
mlift(my_expr) a MonadType(Integer) after_vars(MonadType; Monad(MonadType); Monad(Integer); Monad(String));
```
Definition of `a`:
```
a is
vars (
  T isa Type;
  value isa T;
)
value
;
```
No, I don't like that.

Want a special way to do things like lifting, awaiting, type checking, etc. All
special syntactic features that would be nice to have similar semantics to
function calls. What about using a colon?
```
some_int is 
123:a(Integer32)
;
result is 
doubler:mlifted([1; 2; 3])
where (
  doubler is 
  vars (x:a(Integer32))
  [x; x + x]
  ;
)
;
```

```
ReflexivityTheorem is
vars (
  In:a(Type);
  operation:a(
    Boolean
    after_vars (
      self:a(In);
      other:a(In);
    )
  );
)
Proof(x.operation(x))
after_vars (
  x:a(In);
);

SymmetryTheorem is
vars (
  In:a(Type);
  Out:a(Type);
  :a(EqualityRelation(Out));
  operation:a(
    Out
    after_vars (
      self:a(In);
      other:a(In);
    )
  );
)
Proof(x.operation(y) = y.operation(x))
after_vars (
  x:a(In);
  y:a(In);
);

# The abstract mathematical concept
AbstractEquivalenceRelation is
vars (T:a(Type))
st {
  equivalent:a(EquivalenceTest);
  reflexive:a(TheoremProvingReflexivity(equivalent));
}
where (
  EquivalenceTest is 
  Boolean
  after_vars (
    self:a(T); 
    other:a(T);
  )
  ;
  TheoremProvingReflexivity is
  vars (equivalent EquivalenceTest)
  Proof(x.equivalent(x))
  after_vars (
    x:a(T);
  )
  ;
  TheoremProvingSymmetry is
  vars (equivalent EquivalenceTest)
  Proof(x.equivalent(x))
  after_vars (
    x:a(T);
  )
  ;
)
;
```
Lean is smart and uses function types to declare structure types.
```
define struct_type MyStructType
| constructor :: a: A -> b: p a -> MyStructType
```
We could do something like this?
```
struct_type(
  a:a(A);
  b:a(p(A));
)
```
Just reusing the syntax from `vars`. I think property access `value.a + value.b`
should be baked into the language. Don't define special functions `a(value) +
b(value)` because that would make it ugly for namespaces/modules/whatever you
want to call them.
