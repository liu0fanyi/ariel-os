/// Starts the `fn_name` function in a dedicated thread at startup.
///
/// The thread is given a `stacksize`-byte stack, and has priority `priority`.
#[doc(hidden)]
#[macro_export]
macro_rules! autostart_thread {
    ($fn_name:ident, stacksize = $stacksize:expr, priority = $priority:expr, affinity = $affinity:expr) => {
        $crate::macro_reexports::paste::paste! {
            #[allow(non_snake_case)]
            #[$crate::macro_reexports::linkme::distributed_slice($crate::THREAD_FNS)]
            #[linkme(crate = $crate::macro_reexports::linkme)]
            fn [<__start_thread_ $fn_name>] () {
                use $crate::macro_reexports::static_cell::ConstStaticCell;
                static STACK: ConstStaticCell<[u8; $stacksize]> = ConstStaticCell::new([0u8; $stacksize]);
                $crate::create_noarg($fn_name, STACK.take(), $priority, $affinity);
            }
        }
    };
}
