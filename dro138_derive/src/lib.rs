extern crate proc_macro;

use proc_macro::TokenStream;
use quote::format_ident;
use quote::quote;
use syn;

use regex::Regex;

#[proc_macro_derive(Stm32GpioOutputPin)]
pub fn stm32_gpio_output_pin_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_stm32_gpio_output_pin(&ast)
}

fn impl_stm32_gpio_output_pin(ast: &syn::DeriveInput) -> TokenStream {
    let name = ast.ident.to_string();
    let re = Regex::new(r"^P([A-G])([0-9]{1,2})$").unwrap();

    let captures = re.captures_iter(&name).nth(0).expect("derive(Stm32GpioOutputPin) expects struct name of the form P<A-G><0-99>");
    let (x, n) = (&captures[1], &captures[2]);

    // Which GPIO
    let gpiox = format_ident!("GPIO{}", x);

    // Method to set correct bit in the GPIO's output data register
    let bsn = format_ident!("bs{}", n);

    let name = &ast.ident;
    let gen = quote! {
	impl embedded_hal::digital::v2::OutputPin for #name {
	    type Error = !;

            fn set_low(&mut self) -> Result<(), Self::Error> {
		unsafe { (*stm32f1::stm32f103::#gpiox::ptr()).bsrr.write(|w| w.#bsn().clear_bit()) };
		Ok(())
	    }

	    fn set_high(&mut self) -> Result<(), Self::Error> {
		unsafe { (*stm32f1::stm32f103::#gpiox::ptr()).bsrr.write(|w| w.#bsn().set_bit()) };
		Ok(())
	    }
	}
    };

    gen.into()
}
