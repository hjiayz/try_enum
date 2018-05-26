
#[macro_export]
macro_rules! try_enum {
    ($var:expr,$ident:path,$($other:tt)*) => (
        {
            if let $ident (result) = $var {
                try_enum!(result,$($other)*)
            }
            else {
                None
            }
        }
    );
    ($var:expr,$ident:path[$field:ident],$($other:tt)*) => (
        {
            if let $ident {$field:result,..} = $var {
                try_enum!(result,$($other)*)
            }
            else {
                None
            }
        }
    );
    ($var:expr,$ident:path[$field:ident]) => (
        if let $ident {$field:result,..} = $var {
            Some(result)
        }
        else {
            None
        }
    );
    ($var:expr,$ident:path) => (
        if let $ident (result) = $var {
            Some(result)
        }
        else {
            None
        }
    )
}

