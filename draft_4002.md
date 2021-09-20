
```
take T           :Type
take             :Equality(T)
give BinaryOperation is T From{take left, right :T}

take operation :BinaryOperation

notation (x * y) {
    extends rover.core.operators.(x * y);
    is operation(x; y);
}

aka{;EqPr ;EqualityPreservation}
utype EqPreservation from Record{
    aka{;ppreq ;proof_preserves_equality}
    take ppreserves_eq 
    :Proof(l1 * r1 = l2 * r2) From{
        take l1, l2, r1, r2 :T
        take                :Proof(l1 = l2)
        take                :Proof(r1 = r2)
    }
}

aka{;Assoc}
utype Associativity from Record{
    aka{;passoc ;proof_associative}
    take passociative 
    :Proof(a * b * c = a * (b * c)) From{take a, b, c :T}
}

aka{;Comm}
utype Commutativity from Record{
    aka{;pcomm ;proof_commutative}
    take pcommutative :Proof(a * b = b * a) From{take a, b :T}
}

aka{;Ident}
utype Identity from Record{
    aka{;ident}
    take identity :T
    aka{;pidentl ;proof_identity_left}
    take pidentity_left
    :Proof(identity * x = x) From{take x :T}
    aka{;pidentr ;proof_identity_right}
    take pidentity_right
    :Proof(x * identity = x) From{take x :T}
}

aka{;Inv}
utype Invertibility from Record{
    take          :Identity
    take inverse  :T From{take x :T}
    aka{;pinv ;proof_inverse}
    take pinverse 
    :Proof(x * inverse(x) = identity) From{take x :T} with{
        give identity is the Identity.identity
    }
}

utype Magma from Record{
    extend EqPreservation
}

utype Semigroup from Record{
    extend Magma
}
```

```
give main is aprint("Hello world!")
```
```
give message is "Hello world!"
give main is aprint(message)
```
```
take message :String;
set message is "Hello world!"
give main is aprint(message)
```
```
give main is aprint(set printed_value is "Hello world!")
```
```
give aread_number is mcompose{
    ;line from aread_line
    ;match line.parse(Integer32) {
        on success, munit
        on failure, mcompose{
            ;aprintln("That is not a valid number, please try again.")
            ;aread_number
        }
    }
}
give main is mcompose{
    ;aprint("Input a number: ")
    ;number from aread_number
    ;aprint("You put: {}".format(number))
}
```
```
give aread_number_lt10 is 
mcompose{
    ;line from aread_line
    ;match line.parse{
        on success, aconfirm_number
        on failure, anot_a_number
    }
}
where{
    give aconfirm_number is 
    if num < 10 munit((;num ;the Proof(num < 10))) 
    else afails_requirements
    where{
        take num :Integer32

        give afails_requirements is
        mcompose{
            ;aprintln("That is not less than 10, please try again.")
            ;aread_number_lt10
        }
    }

    give anot_a_number is
    mcompose{
        ;aprintln("That is not a valid number, please try again.")
        ;aread_number_lt10
    }
}
```
As opposed to:
```
fn aread_number_lt10() -> i32 {
    loop {
        let line = read();
        match line.parse() {
            Ok(num) => if num < 10 {
                return num;
            } else {
                println!("That is not less than 10, please try again.");
            },
            Err(_) => {
                println!("That is not a valid number, please try again.");
            }
        }
    }
}
```
So, mutations?
```
give u is ureplace(x => x + 1)
check u.apply(3) = 4
give T is Record{
    take x, y :Integer
}
take v :T
give u2 is v.ux(uchange(x => ))
```
Mutations are functions
```
give m1 is x => x + 1
give c: Context(;I32 ;Void) is
    cvoid()
    capply x => 0
    capply m1
check c.value = 1
```
```
give flipx is mutator{
    mutates p :Point
    to p.x applies mset(-p.x)
}
check record{;x is 5 ;y is 3} after flipx
    = record{;x is -5 ;y is 3}
give rect_flipx is mutator{
    mutates r :Rect
    to r.p1 applies flipx
    to r.p2 applies flipx
}
give :Proof((original after rect_flipx).p1 = original.p1 after flipx)
give push2 is mutator{
    mutates a :Array{;N ;T}
    to a applies push(v0) then push(v1)
}
give minsert_if_none is mutator{
    take value_to_insert :SomeType
    mutates target       :Optional
    to target applies 
    match target {
        on Optional.some(_), midentity,
        on Optional.none, mreplace(Optional.some(value_to_insert))
    }
}

give :Proof(target_is_some) is _
where{
    take target :SomeType

    give target_is_some is 
    match target after minsert_if_none {
        on Optional.some(_), true
        on Optional.none, false
    }
}
```
```
give mreplace is builtin
give :Proof(original after mreplace(func) = func(original))
```

Okay, inductive types are a good idea.
```rust
type Never constructors{}

# A witness to the fact that an instance of Type can never be constructed. This
# is proven by providing a way to convert an instance of Type to an instance of
# Never, which by definition can never be constructed.
aka{NWit}
type NeverWitness
!from{
    take Type :TYPE
}
!constructors{
    give new
    :NeverWitness from{take proof :Never after{:Type}}
}

aka{EWit}
stype EqualityWitness 
!from{
    take Type  :TYPE
    take left  :Type
    take right :Type
} 
!constructors{
    give new
    :EqualityWitness(;left is x ;right is x)
    from{take x :Type}
}

aka{Opt}
type Optional 
from{
    take Type :TYPE
} 
constructors{
    give just    :Optional from{take value :Type}
    give nothing :Optional
}

type Both 
from{
    take First, Second :TYPE
} 
constructors{
    give new 
    :Both 
    from{
        take first :First 
        take second :Second
    }
}

type Either 
from{
    take First, Second :TYPE
}
constructors{
    give first  :Either from{take first :First}
    give second :Either from{take second :Second}
}

stype TruthWitness 
from{
    take condition :Boolean
}
is EqualityWitness(
    ;left is condition 
    ;right is true
)

stype TruthWitness 
from{
    take condition :Boolean
}
constructors{
    give new :TruthWitness(true)
}
```
We can consume one of these with a match (cases?) expression
```rust
take either :Either(;Integer ;Integer)

give the EqualityWitness(Either::first(v).first, v)

give something is 
match either {
    on first, either.first
    on second, either.second
}

give something is 
either!match{
    on first, either.first
    on second, either.second
}

give is_first is either!matches{first}

take never :Never
give something is never!match{} :ArbitraryType
give something is never!match{}!type_is{ArbitraryType}
```
```rust
select{
    if condition0, value0
    elif condition1, value1
    else value2
}
```
```rust
def ureplace
:Mutator{;target :Type}
 !from{
    var Type      :TYPE 
    var new_value :Type!from{var old_value :Type}
 }
```
```ru
type Fallible
!with{
    take Success, Failure :TYPE
}
!constructors{
    give success :Fallible!from{var success_value :Success}
    give failure :Fallible!from{var failure_value :Failure}

    give some_helper_constructor is success(123)
}

give a_success 
is Fallible::success(123)
is Fallible!member{success}!specifying{123}

# You can only do this if you have in scope a value of type
# TruthWitness(success_value!matches{success})
give success_value
is a_success.success_value
is a_success!field{success_value}
```

Maybe modules don't have to be some special thing?
```
give mod is 
module!where{
    publish give thing1 is 123
    publish give thing2 is 456
}

mod::thing1
```
Because this works nicely for referring to variables that aren't defined outside
of an expression:
```
publish give sine is
blahblah
!where {
    var x :Float32
}

publish give something is
sine
!where{
    specify sine::x is 123.456
}
```
Which means types don't have to be defined with a special statement!
```ru
publish give Fallible is type
!where {
    take Success, Failure :TYPE
}
!constructors {
    give success :Self!from{take success_value :Success}
}
```
```ru
publish give Fallible is 
type{
    take Success, Failure :TYPE
}
!constructors {
    give success :Self!from{take success_value :Success}
}
```
```ru
publish give Fallible is 
type{
    take Success, Failure :TYPE

    constructor success :Self!from{take success_value :Success}
    constructor failure :Self!from{take failure_value :Success}
}
```
