- Values can be dependant on other values.
  - The values something depends on are called "dependencies".
- A variable is dependant on the dependencies of its type and then itself.
- A variant is dependant on its type's dependencies and then its values' dependencies.
- A primitive type or value is not dependant on anything.
- A primitive operation is dependant on its inputs.
- A From type is dependant on first its values' dependencies, then its base's
  dependencies.
- A replacing value is dependant on its base's dependencies, with each
  dependency potentially being replaced. Specifically, replacements are applied
  from original sources to value dependencies. Examples:
    - Starting with dependency A and replacing A with a value dependant on B
      results in the entire value being dependant on B.
    - Starting with dependencies A and B and replacing C leaves no changes.
    - Starting with dependencies A and B and replacing B with a value dependant
      on C and D results in A, C, D.

- How to compute the type of any value:
  - First, compute its base type.
    - For variables, this is its type without from constructs.
    - 


LEXICON:
- Value: 4, "hello!", Void::void, any{Integer32}, etc.
- Item: A value with zero or more members.
- Member: an item declared inside another item.
- Variant: a value that is an instance of an inductive type.
  - Void::void, Optional::just(123), etc.