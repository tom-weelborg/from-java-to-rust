// #region content
pub mod container {
    pub struct AccessModifiers;
    impl AccessModifiers {
        // Visible everywhere and to everyone
        pub fn public_method() {
            println!("This is public.");
        }
        
        // Visible to the current crate
        pub(crate) fn crate_public_method() {
            println!("This is public to the create.");
        }
        
        // Visible within the parent module and its descendents
        pub(super) fn parent_public_method() {
            println!("This is public to the parent module.");
        }
        
        // Visible within the current module and its descendents
        pub(self) fn module_public_method() {
            println!("This is public to the current module.");
        }
        
        // Visible within a given module and its descendents
        pub(in container) fn path_public_method() {
            println!("This is public to a given module.");
        }
        
        // equivalent to `pub(self)` (default)
        fn private_method() {
            println!("This is private.");
        }
    }
}
// #endregion content

pub fn main() {}
