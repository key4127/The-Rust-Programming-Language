pub mod external_mod {
	pub mod inner_mod {
		pub fn inner_func() {
            println!("This is inner function.");
        }
    }
    pub fn outer_func() {
        println!("This is outer function.");
    }
} 

fn use_mod() {
	crate::external_mod::outer_func();
	crate::external_mod::inner_mod::inner_func();

    external_mod::outer_func();

    use crate::external_mod::inner_mod::inner_func;
    inner_func();
    inner_func();
}

/*mod another_mod {
    fn another_mod_use_original_mod() {
        inner_func();
    }
}*/