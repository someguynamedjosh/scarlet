#[macro_export]
macro_rules! combinator_first_of{
    ($Name:ident $($VariantName:ident)*) => {
        paste! {
            enum [<$Name Result>]<'i> {
                $($VariantName($VariantName::ParserResult<'i>),)*
            }
        }

        pub struct $Name;

        impl Transformer for $Name {
            
        }
    }
}


