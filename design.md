# Constructs

## Root
- any
  - Produces `Any` value, `Empty` namespace.
- builtin_item
  - Produces `BuiltinValue` or `BuiltinOperator` value, `Empty` namespace.
- identifier
  - Produces `Identifier` value and namespace.
- u8
  - Produces `BuiltinValue` value, `Empty` namespace.
- variant
  - Produces `Variant` value, `Empty` namespace.
  
## Postfix
- defining
  - Value: identity
  - Namespace: 
    ```rust
    let id = next_ns_id();
    Defining { 
      child: base.namespace_id, 
      definitions: body.map(
        todo!()
        parent: next_ns_id
      ),
      parent: context.containing_namespace 
    }
    ```
- FromValues
- member
- replacing
- type_is (M2)

# Stage 2

## Item
- Fields:
  - namespace_id: `NamespaceId`
  - value_id: `ValueId`

## Namespace/NamespaceId

### Defining
- Fields:
  - child: `NamespaceId`
  - definitions: `Definitions` aka `OrderedMap<String, Item>`
  - parent: `NamespaceId`

### Empty
- No fields

### Identifier
- Fields:
  - name: String

### Replacing
- Fields:
  - replacements: `VariableReplacements` aka `OrderedMap<VariableId, ValueId>`
  - source: `NamespaceId`

## Value/ValueId

- Any
  - variable: `VariableId`
- BuiltinOperation
  - operation: `BuiltinOperation`
- BuiltinValue
  - value: `BuiltinValue`
- FromValues
  - base: `ValueId`
  - items: `Vec<ValueId>`
- Identifier
  - name: `String`
  - in_namespace: `NamespaceId`
- Item
  - item: `ValueId`
- Member
  - base: `NamespaceId`
  - member: `String`
- Replacing
  - base: `ValueId`
  - replacements: `Vec<(ValueId, ValueId)>`
- TypeIs (M2)
  - base_type_only: `bool`
  - base: `ValueId`
  - typee: `ValueId`
- Variant
  - variant: `VariantId`

## Variable/VariableId
- Fields:
  - definition: `ValueId`
  - original_type: `ValueId`

## Variant/VariantId
- Fields:
  - definition: `ValueId`
  - original_type: `ValueId`

# Stage 3

## Value/ValueId

- Any
  - variable: `VariableId`
- BuiltinOperation
  - operation: `BuiltinOperation`
- BuiltinValue
  - value: `BuiltinValue`
- From
  - base: `ValueId`
  - variables: `OrderedSet<VariableId>`
- Replacing
  - base: `ValueId` 
  - replacements: `Vec<(ValueId, ValueId)>`
- TypeIs (M2)
  - base_type_only: `bool`
  - base: `ValueId`
  - typee: `ValueId`
- Variant
  - variant: `VariantId`

## Variable/VariableId
- Fields:
  - definition: `ValueId`
  - original_type: `ValueId`

## Variant/VariantId
- Fields:
  - definition: `ValueId`
  - original_type: `ValueId`
