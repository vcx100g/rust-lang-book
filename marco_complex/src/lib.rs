#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// use proc_macro;
//
// #[proc_macro]
// pub fn some_name(input: TokenStream) -> TokenStream {
// }