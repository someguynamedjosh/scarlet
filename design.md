# Constructs

## Root
- any
- builtin_item
- identifier
- u8
- variant
  
## Postfix
- defining
- FromItems
- member
- replacing
- type_is (M2)

# Things there can be

## Item/ItemId
- Fields:
  - value: `Value`
  - type: `Option<ItemId>`
  - defined_in: `Option<ScopeId>`
  - cached_reduction: `Option<ItemId>`
- Type is either `self.type` or the inferred type of `value`.
  - This is the definition of `type_of(item_id)`
- Reduction takes an Item and produces a new ItemId according to an
implementation from its value.

## Scope/ScopeId
- Fields:
  - definition: `ItemId` pointing to a `Defining`
  - parent: `Option<ScopeId>`

## Value

### Any
- Fields:
  - variable: `VariableId`
- Inferred type:
  - `reduce(variable.original_type)` with deps of type and self as from
  variables
- Reduction:
  - self
- Code:
  - any{`name_or_code(variable.original_type)`}

### BuiltinOperation
- Fields:
  - operation: `BuiltinOperation`
- Inferred type:
  - `operation.inferred_type()`
- Code:
  - builtin_operation{`name` `name_or_code(args)`}

### BuiltinValue
- Fields:
  - value: `BuiltinValue`
- Inferred type:
  - `value.inferred_type()`
- Reduction:
  - `self`

### Defining
- Fields:
  - base: `ItemId`
  - definitions: `Vec<(String, ItemId)>`
  - child_scope: `ScopeId`
- Inferred type:
  - `type_of(base)`
- Reduction:
  - reduce base
  - reduce definitions
- Code:
  - `code_or_name(base)` defining{ `name` is .. }

### FromItems
- Fields:
  - base: `ItemId`
  - items: `Vec<ItemId>`
- Inferred type:
  - use `FromVars` instead.
- Reduction:
  - Gets the dependencies of all items.
  - `reduce(FromVars { base, variables: dependencies })`
  - If dependencies is empty, return `reduce(base)`
- Code:
  - Use reduced form

### FromVars
- Fields:
  - base: `ItemId`
  - variables: `Vec<VariableId>`
- Inferred type:
  - `reduce(type_of(base))`
  - If that is `FromVars`, remove from it any variables that appear in
  `self.variables`.
- Reduction:
  - Reduce base
  - If dependencies is empty, return `reduce(base)`
- Code:
  - `name_or_code(base)` From{ `name_or_code(item)` }

### Identifier
- Fields:
  - name: `String`
- Inferred type:
  - `type_of(reduce(self))`
- Reduction:
  - Find a definition of `name` in containing scope or any of its parents.
- Code:
  - Use reduced form

### Item
- Fields:
  - item: `ItemId`
- Inferred type:
  - `type_of(item)`
- Reduction:
  - `reduce(item)`
- Code:
  - Use reduced form

### Member
- Fields:
  - base: `ItemId`
  - member: `String`
- Inferred type:
  - `type_of(reduce(self))`
- Reduction:
  - Get `reduce(base)`
  - Find a definition of `name` in reduced base's `member_scopes`
- Code:
  - Use reduced form

### Replacing
- Fields:
  - base: `ItemId`
  - replacements: `Vec<(VariableId, ItemId)>`
- Inferred type:
  - `type_of(base)`
  - then perform replacement
  - then reduce
- Replacement:
  - Converts an `Item` into another `Item` where all instances of variables have
  been replaced with other items according to `self.replacements` in itself and
  its type (including members.)
- Reduction:
  - Reduce base
  - Reduce replacements
  - Apply replacements to base
  - Reduce replaced base
  - Calculate dependencies of replaced base
  - Create a new Replacing which keeps replacements that the base is still
  dependant on.
  - If none, returns the reduced base.
- Code:
  - `name_or_code(base)` replacing{`name(var)` with `name_or_code(val)`}

### TypeIs (M2)
- Fields:
  - base_type_only: `bool`
  - base: `ItemId`
  - typee: `ItemId`
- Inferred type:
  - If `base_type_only`, `type_of(base)` replacing base type with `typee`.
  - Else, `typee`.
- Reduction:
  - `reduce(base)` with type replaced with inferred type of self.

### Variant
- Fields:
  - variant: `VariantId`
- Inferred type:
  - `variant.original_type`
- Reduction:
  - Inserts a copy of this item with the type reduced.
- Code:
  - anonymous

## Variable/VariableId
- Fields:
  - definition: `ItemId`
  - original_type: `ItemId`

## Variant/VariantId
- Fields:
  - definition: `ItemId`
  - original_type: `ItemId`
