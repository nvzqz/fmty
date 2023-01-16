//! Internal helper macros.

/// Invokes a macro after removing the first identifier.
macro_rules! peel {
    ($mac:ident: $x:ident $(, $rest:ident)*) => {
        $mac! { $($rest),* }
    };
}
