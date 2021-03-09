//!
//! Defines macros for easily exporting functions
//!

#[macro_export]
macro_rules! export {

    ($(
        $(#[$func_meta:meta])*
        fn $name:ident($( $arg:ident : $atype:ty ),*) -> $ret:ty $code:block
    )*) => (
        $(
            #[allow(non_snake_case)]
            $(#[$func_meta])*
            fn $name($( $arg: $atype ),*) -> $ret $code
        )*

        register_module!(mut m, {
            $(
                m.export_function(stringify!($name), |mut cx| {
                    // Can be done away with a fancier macro
                    let mut _arg_index = 0;

                    $(
                        let $arg = cx.argument_opt(_arg_index);
                        let $arg: $atype =
                            match $crate::from_value_opt(&mut cx, $arg) {
                                Err(err) => return cx.throw_error(format!("On arg_index: {}, got an error deserializing, was: {}", _arg_index, err)),
                                Ok(value) => value,
                        };
                        _arg_index += 1;
                    )*

                    let result = $name($( $arg ),*);
                    let handle =
                        match $crate::to_value(&mut cx, &result) {
                            Err(err) => return cx.throw_error(format!("On result, got an error serializing, was: {}", err)),
                            Ok(handle_value) => handle_value,
                    };
                    Ok(handle)
                })?;
            )*
            Ok(())
        });
    )
}
