```rs
UNIQUE.SHOWN

anon0
USING {
    anon0 IS UNIQUE
}



VAR[].SHOWN

anon0
USING {
    anon0 IS VAR[]
}



thing IS UNIQUE.SHOWN

anon0
USING {
    anon0 IS UNIQUE
}



thing IS UNIQUE
thing.SHOWN

thing



thing IS UNIQUE
asdfasdfasdf IS thing
thing.SHOWN

thing



thing IS { UNIQUE }
thing.VALUE.SHOWN

thing.VALUE



thing IS { UNIQUE }
thing.VALUE.SHOWN

thing.VALUE



reallylongnameohmygoditssolong IS { UNIQUE }
reallylongnameohmygoditssolong.VALUE.SHOWN

reallylongnameohmygoditssolong.VALUE



{ UNIQUE }.VALUE.SHOWN

anon0
USING {
    anon0 IS UNIQUE
}



module IS { a IS 123   b IS 456 }
module.a.SHOWN

module.a



module IS { a IS 123   b IS 456 }
module.VALUE.SHOWN

module.a
```