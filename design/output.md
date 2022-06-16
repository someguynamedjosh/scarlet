struct:
    "{"
    multiple constructs:
        multiple constructs:
            is:
                identifier:
                    "basics"
                "IS"
                struct:
                    "{"
                    multiple constructs:
                        multiple constructs:
                            multiple constructs:
                                multiple constructs:
                                    multiple constructs:
                                        multiple constructs:
                                            multiple constructs:
                                                is:
                                                    identifier:
                                                        "true"
                                                    "IS"
                                                    as language item:
                                                        UNIQUE:
                                                            "UNIQUE"
                                                        " AS_LANGUAGE_ITEM"
                                                        "["
                                                        identifier:
                                                            "true"
                                                        "]"
                                                ","
                                                is:
                                                    identifier:
                                                        "false"
                                                    "IS"
                                                    as language item:
                                                        UNIQUE:
                                                            "UNIQUE"
                                                        " AS_LANGUAGE_ITEM"
                                                        "["
                                                        identifier:
                                                            "false"
                                                        "]"
                                            ","
                                            is:
                                                identifier:
                                                    "void"
                                                "IS"
                                                as language item:
                                                    UNIQUE:
                                                        "UNIQUE"
                                                    " AS_LANGUAGE_ITEM"
                                                    "["
                                                    identifier:
                                                        "void"
                                                    "]"
                                        ","
                                        is:
                                            identifier:
                                                "x"
                                            "IS"
                                            as language item:
                                                variable:
                                                    "VAR"
                                                    "["
                                                    multiple constructs:
                                                        identifier:
                                                            "ORD"
                                                        ","
                                                        identifier:
                                                            "32"
                                                    "]"
                                                " AS_LANGUAGE_ITEM"
                                                "["
                                                identifier:
                                                    "x"
                                                "]"
                                    ","
                                    is:
                                        identifier:
                                            "fx"
                                        "IS"
                                        variable:
                                            "VAR"
                                            "["
                                            multiple constructs:
                                                multiple constructs:
                                                    multiple constructs:
                                                        identifier:
                                                            "DEP"
                                                        ","
                                                        identifier:
                                                            "x"
                                                    ","
                                                    identifier:
                                                        "ORD"
                                                ","
                                                identifier:
                                                    "32"
                                            "]"
                                ","
                                is:
                                    identifier:
                                        "a"
                                    "IS"
                                    variable:
                                        "VAR"
                                        "["
                                        decision:
                                            "DECISION"
                                            "["
                                            multiple constructs:
                                                multiple constructs:
                                                    multiple constructs:
                                                        identifier:
                                                            "SELF"
                                                        ","
                                                        identifier:
                                                            "true"
                                                    ","
                                                    identifier:
                                                        "true"
                                                ","
                                                equal:
                                                    identifier:
                                                        "SELF"
                                                    "="
                                                    identifier:
                                                        "false"
                                            "]"
                                        "]"
                            ","
                            is:
                                identifier:
                                    "b"
                                "IS"
                                variable:
                                    "VAR"
                                    "["
                                    decision:
                                        "DECISION"
                                        "["
                                        multiple constructs:
                                            multiple constructs:
                                                multiple constructs:
                                                    identifier:
                                                        "SELF"
                                                    ","
                                                    identifier:
                                                        "true"
                                                ","
                                                identifier:
                                                    "true"
                                            ","
                                            equal:
                                                identifier:
                                                    "SELF"
                                                "="
                                                identifier:
                                                    "false"
                                        "]"
                                    "]"
                        ","
                        is:
                            identifier:
                                "and"
                            "IS"
                            as language item:
                                parentheses:
                                    "("
                                    decision:
                                        "DECISION"
                                        "["
                                        multiple constructs:
                                            multiple constructs:
                                                multiple constructs:
                                                    identifier:
                                                        "a"
                                                    ","
                                                    identifier:
                                                        "true"
                                                ","
                                                identifier:
                                                    "b"
                                            ","
                                            identifier:
                                                "false"
                                        "]"
                                    ")"
                                " AS_LANGUAGE_ITEM"
                                "["
                                identifier:
                                    "and"
                                "]"
                    "}"
            ","
            is:
                identifier:
                    "misc"
                "IS"
                struct:
                    "{"
                    multiple constructs:
                        multiple constructs:
                            multiple constructs:
                                multiple constructs:
                                    multiple constructs:
                                        multiple constructs:
                                            multiple constructs:
                                                multiple constructs:
                                                    multiple constructs:
                                                        multiple constructs:
                                                            is:
                                                                identifier:
                                                                    "true"
                                                                "IS"
                                                                member access:
                                                                    identifier:
                                                                        "basics"
                                                                    "."
                                                                    identifier:
                                                                        "true"
                                                            ","
                                                            is:
                                                                identifier:
                                                                    "false"
                                                                "IS"
                                                                member access:
                                                                    identifier:
                                                                        "basics"
                                                                    "."
                                                                    identifier:
                                                                        "false"
                                                        ","
                                                        is:
                                                            identifier:
                                                                "void"
                                                            "IS"
                                                            member access:
                                                                identifier:
                                                                    "basics"
                                                                "."
                                                                identifier:
                                                                    "void"
                                                    ","
                                                    is:
                                                        identifier:
                                                            "and"
                                                        "IS"
                                                        member access:
                                                            identifier:
                                                                "basics"
                                                            "."
                                                            identifier:
                                                                "and"
                                                ","
                                                is:
                                                    identifier:
                                                        "t_just"
                                                    "IS"
                                                    variable:
                                                        "VAR"
                                                        "["
                                                        identifier:
                                                            "SELF"
                                                        "]"
                                            ","
                                            is:
                                                identifier:
                                                    "asdf"
                                                "IS"
                                                UNIQUE:
                                                    "UNIQUE"
                                        ","
                                        is:
                                            identifier:
                                                "a"
                                            "IS"
                                            variable:
                                                "VAR"
                                                "["
                                                missing
                                                "]"
                                    ","
                                    is:
                                        identifier:
                                            "x"
                                        "IS"
                                        variable:
                                            "VAR"
                                            "["
                                            missing
                                            "]"
                                ","
                                is:
                                    identifier:
                                        "Bool"
                                    "IS"
                                    variable:
                                        "VAR"
                                        "["
                                        decision:
                                            "DECISION"
                                            "["
                                            multiple constructs:
                                                multiple constructs:
                                                    multiple constructs:
                                                        identifier:
                                                            "SELF"
                                                        ","
                                                        identifier:
                                                            "true"
                                                    ","
                                                    identifier:
                                                        "true"
                                                ","
                                                equal:
                                                    identifier:
                                                        "SELF"
                                                    "="
                                                    identifier:
                                                        "false"
                                            "]"
                                        "]"
                            ","
                            is:
                                identifier:
                                    "is_bool"
                                "IS"
                                from:
                                    identifier:
                                        "x"
                                    "FROM"
                                    identifier:
                                        "Bool"
                        ","
                        substitution:
                            identifier:
                                "t_just"
                            "["
                            substitution:
                                identifier:
                                    "is_bool"
                                "["
                                equal:
                                    identifier:
                                        "x"
                                    "="
                                    identifier:
                                        "void"
                                "]"
                            "]"
                    "}"
        ","
        is:
            identifier:
                "theorems"
            "IS"
            struct:
                "{"
                multiple constructs:
                    multiple constructs:
                        multiple constructs:
                            multiple constructs:
                                multiple constructs:
                                    multiple constructs:
                                        multiple constructs:
                                            multiple constructs:
                                                multiple constructs:
                                                    multiple constructs:
                                                        multiple constructs:
                                                            as language item:
                                                                UNIQUE:
                                                                    "UNIQUE"
                                                                " AS_LANGUAGE_ITEM"
                                                                "["
                                                                identifier:
                                                                    "t_inv_eq_statement"
                                                                "]"
                                                            ","
                                                            as language item:
                                                                UNIQUE:
                                                                    "UNIQUE"
                                                                " AS_LANGUAGE_ITEM"
                                                                "["
                                                                identifier:
                                                                    "t_decision_eq_statement"
                                                                "]"
                                                        ","
                                                        as language item:
                                                            UNIQUE:
                                                                "UNIQUE"
                                                            " AS_LANGUAGE_ITEM"
                                                            "["
                                                            identifier:
                                                                "t_decision_neq_statement"
                                                            "]"
                                                    ","
                                                    is:
                                                        identifier:
                                                            "true"
                                                        "IS"
                                                        member access:
                                                            identifier:
                                                                "basics"
                                                            "."
                                                            identifier:
                                                                "true"
                                                ","
                                                is:
                                                    identifier:
                                                        "false"
                                                    "IS"
                                                    member access:
                                                        identifier:
                                                            "basics"
                                                        "."
                                                        identifier:
                                                            "false"
                                            ","
                                            is:
                                                identifier:
                                                    "x"
                                                "IS"
                                                member access:
                                                    identifier:
                                                        "basics"
                                                    "."
                                                    identifier:
                                                        "x"
                                        ","
                                        is:
                                            identifier:
                                                "fx"
                                            "IS"
                                            member access:
                                                identifier:
                                                    "basics"
                                                "."
                                                identifier:
                                                    "fx"
                                    ","
                                    is:
                                        identifier:
                                            "t_refl"
                                        "IS"
                                        as auto theorem:
                                            value access:
                                                struct:
                                                    "{"
                                                    multiple constructs:
                                                        multiple constructs:
                                                            axiom:
                                                                "AXIOM"
                                                                "["
                                                                identifier:
                                                                    "t_refl"
                                                                "]"
                                                            ","
                                                            as language item:
                                                                parentheses:
                                                                    "("
                                                                    equal:
                                                                        identifier:
                                                                            "a"
                                                                        "="
                                                                        identifier:
                                                                            "a"
                                                                    ")"
                                                                " AS_LANGUAGE_ITEM"
                                                                "["
                                                                identifier:
                                                                    "t_refl_statement"
                                                                "]"
                                                        ","
                                                        is:
                                                            identifier:
                                                                "a"
                                                            "IS"
                                                            variable:
                                                                "VAR"
                                                                "["
                                                                missing
                                                                "]"
                                                    "}"
                                                ".VALUE"
                                            " AS_AUTO_THEOREM"
                                ","
                                is:
                                    identifier:
                                        "t_decision_eq_inv"
                                    "IS"
                                    as auto theorem:
                                        value access:
                                            struct:
                                                "{"
                                                multiple constructs:
                                                    multiple constructs:
                                                        multiple constructs:
                                                            multiple constructs:
                                                                multiple constructs:
                                                                    axiom:
                                                                        "AXIOM"
                                                                        "["
                                                                        identifier:
                                                                            "t_trivial"
                                                                        "]"
                                                                    ","
                                                                    as language item:
                                                                        decision:
                                                                            "DECISION"
                                                                            "["
                                                                            multiple constructs:
                                                                                multiple constructs:
                                                                                    multiple constructs:
                                                                                        identifier:
                                                                                            "a"
                                                                                        ","
                                                                                        identifier:
                                                                                            "b"
                                                                                    ","
                                                                                    identifier:
                                                                                        "u"
                                                                                ","
                                                                                identifier:
                                                                                    "v"
                                                                            "]"
                                                                        " AS_LANGUAGE_ITEM"
                                                                        "["
                                                                        identifier:
                                                                            "t_trivial_statement"
                                                                        "]"
                                                                ","
                                                                is:
                                                                    identifier:
                                                                        "a"
                                                                    "IS"
                                                                    variable:
                                                                        "VAR"
                                                                        "["
                                                                        missing
                                                                        "]"
                                                            ","
                                                            is:
                                                                identifier:
                                                                    "b"
                                                                "IS"
                                                                variable:
                                                                    "VAR"
                                                                    "["
                                                                    equal:
                                                                        identifier:
                                                                            "a"
                                                                        "="
                                                                        identifier:
                                                                            "SELF"
                                                                    "]"
                                                        ","
                                                        is:
                                                            identifier:
                                                                "u"
                                                            "IS"
                                                            variable:
                                                                "VAR"
                                                                "["
                                                                identifier:
                                                                    "SELF"
                                                                "]"
                                                    ","
                                                    is:
                                                        identifier:
                                                            "v"
                                                        "IS"
                                                        variable:
                                                            "VAR"
                                                            "["
                                                            missing
                                                            "]"
                                                "}"
                                            ".VALUE"
                                        " AS_AUTO_THEOREM"
                            ","
                            is:
                                identifier:
                                    "t_decision_neq_inv"
                                "IS"
                                as auto theorem:
                                    value access:
                                        struct:
                                            "{"
                                            multiple constructs:
                                                multiple constructs:
                                                    multiple constructs:
                                                        multiple constructs:
                                                            multiple constructs:
                                                                axiom:
                                                                    "AXIOM"
                                                                    "["
                                                                    identifier:
                                                                        "t_invariant_truth"
                                                                    "]"
                                                                ","
                                                                as language item:
                                                                    decision:
                                                                        "DECISION"
                                                                        "["
                                                                        multiple constructs:
                                                                            multiple constructs:
                                                                                multiple constructs:
                                                                                    identifier:
                                                                                        "a"
                                                                                    ","
                                                                                    identifier:
                                                                                        "b"
                                                                                ","
                                                                                identifier:
                                                                                    "u"
                                                                            ","
                                                                            identifier:
                                                                                "v"
                                                                        "]"
                                                                    " AS_LANGUAGE_ITEM"
                                                                    "["
                                                                    identifier:
                                                                        "t_invariant_truth_statement"
                                                                    "]"
                                                            ","
                                                            is:
                                                                identifier:
                                                                    "a"
                                                                "IS"
                                                                variable:
                                                                    "VAR"
                                                                    "["
                                                                    missing
                                                                    "]"
                                                        ","
                                                        is:
                                                            identifier:
                                                                "b"
                                                            "IS"
                                                            variable:
                                                                "VAR"
                                                                "["
                                                                decision:
                                                                    "DECISION"
                                                                    "["
                                                                    multiple constructs:
                                                                        multiple constructs:
                                                                            multiple constructs:
                                                                                identifier:
                                                                                    "a"
                                                                                ","
                                                                                identifier:
                                                                                    "SELF"
                                                                            ","
                                                                            identifier:
                                                                                "false"
                                                                        ","
                                                                        identifier:
                                                                            "true"
                                                                    "]"
                                                                "]"
                                                    ","
                                                    is:
                                                        identifier:
                                                            "u"
                                                        "IS"
                                                        variable:
                                                            "VAR"
                                                            "["
                                                            missing
                                                            "]"
                                                ","
                                                is:
                                                    identifier:
                                                        "v"
                                                    "IS"
                                                    variable:
                                                        "VAR"
                                                        "["
                                                        identifier:
                                                            "SELF"
                                                        "]"
                                            "}"
                                        ".VALUE"
                                    " AS_AUTO_THEOREM"
                        ","
                        is:
                            identifier:
                                "t_decision_by_parts_ext"
                            "IS"
                            as auto theorem:
                                value access:
                                    struct:
                                        "{"
                                        multiple constructs:
                                            multiple constructs:
                                                multiple constructs:
                                                    multiple constructs:
                                                        multiple constructs:
                                                            multiple constructs:
                                                                multiple constructs:
                                                                    axiom:
                                                                        "AXIOM"
                                                                        "["
                                                                        identifier:
                                                                            "t_eq_ext_rev"
                                                                        "]"
                                                                    ","
                                                                    as language item:
                                                                        identifier:
                                                                            "conclusion"
                                                                        " AS_LANGUAGE_ITEM"
                                                                        "["
                                                                        identifier:
                                                                            "t_eq_ext_rev_statement"
                                                                        "]"
                                                                ","
                                                                is:
                                                                    identifier:
                                                                        "conclusion"
                                                                    "IS"
                                                                    substitution:
                                                                        identifier:
                                                                            "fx"
                                                                        "["
                                                                        identifier:
                                                                            "inner"
                                                                        "]"
                                                            ","
                                                            is:
                                                                identifier:
                                                                    "inner"
                                                                "IS"
                                                                decision:
                                                                    "DECISION"
                                                                    "["
                                                                    multiple constructs:
                                                                        multiple constructs:
                                                                            multiple constructs:
                                                                                identifier:
                                                                                    "a"
                                                                                ","
                                                                                identifier:
                                                                                    "b"
                                                                            ","
                                                                            identifier:
                                                                                "c"
                                                                        ","
                                                                        identifier:
                                                                            "d"
                                                                    "]"
                                                        ","
                                                        is:
                                                            identifier:
                                                                "a"
                                                            "IS"
                                                            variable:
                                                                "VAR"
                                                                "["
                                                                missing
                                                                "]"
                                                    ","
                                                    is:
                                                        identifier:
                                                            "b"
                                                        "IS"
                                                        variable:
                                                            "VAR"
                                                            "["
                                                            missing
                                                            "]"
                                                ","
                                                is:
                                                    identifier:
                                                        "c"
                                                    "IS"
                                                    variable:
                                                        "VAR"
                                                        "["
                                                        substitution:
                                                            identifier:
                                                                "fx"
                                                            "["
                                                            identifier:
                                                                "c"
                                                            "]"
                                                        "]"
                                            ","
                                            is:
                                                identifier:
                                                    "d"
                                                "IS"
                                                variable:
                                                    "VAR"
                                                    "["
                                                    substitution:
                                                        identifier:
                                                            "fx"
                                                        "["
                                                        identifier:
                                                            "d"
                                                        "]"
                                                    "]"
                                        "}"
                                    ".VALUE"
                                " AS_AUTO_THEOREM"
                    ","
                    is:
                        identifier:
                            "t_decision_by_parts"
                        "IS"
                        as auto theorem:
                            value access:
                                struct:
                                    "{"
                                    multiple constructs:
                                        multiple constructs:
                                            multiple constructs:
                                                multiple constructs:
                                                    multiple constructs:
                                                        multiple constructs:
                                                            multiple constructs:
                                                                axiom:
                                                                    "AXIOM"
                                                                    "["
                                                                    identifier:
                                                                        "t_invariant_truth_rev"
                                                                    "]"
                                                                ","
                                                                as language item:
                                                                    identifier:
                                                                        "conclusion"
                                                                    " AS_LANGUAGE_ITEM"
                                                                    "["
                                                                    identifier:
                                                                        "t_invariant_truth_rev_statement"
                                                                    "]"
                                                            ","
                                                            is:
                                                                identifier:
                                                                    "conclusion"
                                                                "IS"
                                                                identifier:
                                                                    "inner"
                                                        ","
                                                        is:
                                                            identifier:
                                                                "inner"
                                                            "IS"
                                                            decision:
                                                                "DECISION"
                                                                "["
                                                                multiple constructs:
                                                                    multiple constructs:
                                                                        multiple constructs:
                                                                            identifier:
                                                                                "a"
                                                                            ","
                                                                            identifier:
                                                                                "b"
                                                                        ","
                                                                        identifier:
                                                                            "c"
                                                                    ","
                                                                    identifier:
                                                                        "d"
                                                                "]"
                                                    ","
                                                    is:
                                                        identifier:
                                                            "a"
                                                        "IS"
                                                        variable:
                                                            "VAR"
                                                            "["
                                                            missing
                                                            "]"
                                                ","
                                                is:
                                                    identifier:
                                                        "b"
                                                    "IS"
                                                    variable:
                                                        "VAR"
                                                        "["
                                                        missing
                                                        "]"
                                            ","
                                            is:
                                                identifier:
                                                    "c"
                                                "IS"
                                                variable:
                                                    "VAR"
                                                    "["
                                                    identifier:
                                                        "c"
                                                    "]"
                                        ","
                                        is:
                                            identifier:
                                                "d"
                                            "IS"
                                            variable:
                                                "VAR"
                                                "["
                                                identifier:
                                                    "d"
                                                "]"
                                    "}"
                                ".VALUE"
                            " AS_AUTO_THEOREM"
                "}"
    "}"