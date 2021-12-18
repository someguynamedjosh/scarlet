```
var ns = namespace (
    var a: Integer;
    var b: Integer;

    var c = a + b;
)
var something_else = ns.c specifying (
    ns.a = 1;
    ns.b = 2;
)
```

Why can we say `ns.a = 1` but not `1 = 2`? What efficient system would prevent
us from writing logical contradictions? Because you could imagine a situation
where you'd want to write:
```
some_var specifying (
    a + b is 5;
)
```
How can we ensure this doesn't introduce a logical contradiction?

First, let's ask "what's a simple way this vague idea could lead to a logical
contradiction?"
```
vars a, b: Integer;
var c = a * 0;
c specifying (a * 0 = 1);
```
Let's refine this. Is there any way that writing an equality with only variables
and operators on the left hand side can result in a contradiction?
```
vars a, b: Integer;
var c = a * b;
c specifying (a * b = 5);
```
Since a variable indicates a placeholder for an expression of a particular type,
it should be valid to replace any individual variable with an expression of that
type.
```
vars a, b: Integer;
val c = a * b;
c specifying (a -> 3; b -> a;)
```
All variables are values. Not all values are variables. If `a` and `b` are
variables, `a + b` is a value, but not a variable. Therefore, when `specifying`,
it is valid to replace `a` or `b` with something, but it is not valid to replace
`a + b` with something.
```
vars a, b: Integer;
var: Proof(a + b = 0);
```
Structure types?
```
vars a, b: Integer;
val my_struct_value = {a; b}
typeof(my_struct_value) = st {a isa Integer; b isa Integer}
```
Let's say we have a type representing a rational number:
```
val Rational = st {numerator isa Integer; denominator isa Integer}
```
We want to define equality such that `2/3 = 4/6`. I think this means that if `a`
and `b` are equal, then their numerators have the same gcd as their divisors.
Or alternatively, their reduced forms are equal, with a reduced form being a
rational with numerator and denominator divided by their GCD.
```
var self: Rational;

val reduced := 
new_rational(
    numerator is self.numerator / the_gcd;
    denominator is self.denominator / the_gcd;
) 
where (
    the_gcd is gcd(self.numerator; self.denominator)
);

var a, b: Rational;

val equality = 
new_equality_relation(
    equal =
    (ar.numerator = br.numerator) 
    and (ar.denominator = br.denominator)
    where (
        ar = a.reduced();
        br = b.reduced();
    );
)
```
Maybe we don't need to have equality be this big important thing? We can just
have it so that equality is a function that makes certain guarantees, and some
functions leverage those guarantees while others are outside the scope of those
guarantees.
```
type MyType from { a: Integer; b: Integer };

def the Equal(MyType) is
(
    field equal is
    vars(self: MyType; other: MyType)
    self.a = other.a;

    field preflexive is
    vars(self: MyType)
    Equal(Integer).preflexive(self.a)
    :Proof(self.equal(self));

    field psymmetric is
    vars(self: MyType; other: MyType)
    Equal(Integer).psymmetric(self.a; other.a)
    :Proof(self.equal(other) = other.equal(self))

    field ptransitive is
    vars(
        self: MyType; 
        other: MyType; 
        another: MyType;
        : Proof(self.equal(other));
        : Proof(other.equal(another));
    )
    Equal(Integer).ptransitive(
        self.a;
        other.a;
        another.a;
        the Proof(self.equal(other));
        the Proof(other.equal(another));
    )
    :Proof(another.equal(self))
);

def double_a is
vars(self: MyType)
self.a * 2;

def pdouble_a_eq is (
    var self  :MyType;
    var other :MyType;
    var       :Proof(self = other);
    palg_mul(
        left   is self.a; 
        right  is other.a; 
        factor is 2;
    )
    :Proof(double_a(self) = double_a(other))
)
```
```
def identity is {
    var T    :Type;
    var self :T;
    self;
}

def BinaryOperator is {
    vars Left, Right, Output :Type;

    :var left  :Left;
    :var right :Right;
    is Output;
};

vars A, B :Type;

# The type of binary operators that take two operands and 
# produce a result all of some type A.
def BinaryOperatorAaA is {
    is BinaryOperator(
        Left   is A;
        Right  is A;
        Output is A;
    );
};

# The type of binary operators that take two operands of
# some type A and produce a result of some type B.
def BinaryOperatorAaB is {
    is BinaryOperator(
        Left   is A;
        Right  is A;
        Output is B;
    );
};

# The type of binary operators that take two operands, one
# of some type A and another of some type B, producing a
# result of type B.
def BinaryOperatorAbB is {
    is BinaryOperator(
        Left   is A;
        Right  is B;
        Output is B;
    );
};

aka {Assoc}
unarytype Associative from {
    aka {op_aa}
    var operator_aa :BinaryOperatorAaA;
    aka {op_ab}
    var operator_ab :BinaryOperatorAbB;

    aka {passoc; proof_associative}
    :field passociative :{
        :vars a₁, a₂ :A;
        :vars b      :B;
        is Proof((a₁*a₂)*b = a₁*(a₂*b));

        notation x * y is {
            is operator_aa(x; y);
            is operator_ab(x; y);
            associativity left;
        };
    };
};

aka{SAssoc}
unarytype SimpleAssociative from {
    aka {op}
    var operator :BinaryOperator(AaA);

}

aka (Symm)
unarytype Symmetric from {
    aka (psymm; proof_symmetric)
    :field psymmetric :{
        :vars x, y :A;
        Proof(opaab(x; y) = opaab(y; x))
    };
};

unarytype Multiply from {
    :field multiply :BinaryOperatorAbB;
    :field          :Multiply(A; A);
    operator_ab_b   is multiply;
    operator_aa_a   is the Multiply(A; A).multiply;
    :field          :Associative;

    aka{pzcl; proof_zero_cancel_left}
    :field pzero_cancel_right :{
        :vars a :A; 
        Proof(multiply(a; zero) = zero);
    };
    aka{pzcr; proof_zero_cancel_right}
    :field pzero_cancel_left :{
        :vars b :B; 
        Proof(multiply(zero; b) = zero);
    };

    aka{poil; proof_one_identity_left}
    :field pone_identity_left :{
        :vars self :T; 
        Proof(one * self = self);
    };
    aka{poir; proof_one_identity_right}
    :field pone_identity_right :{
        :vars self :T; 
        Proof(self * one = self);
    };
};
```

Okay look, it would be nice to have a way to have variables associated with some
computation but still be able to refer to them outside that computation.
Something like:
```
def sine is {
    var x :Float32;
    some_implementation;
}

set sine::x to pi;

check sine == 0.0;
```
