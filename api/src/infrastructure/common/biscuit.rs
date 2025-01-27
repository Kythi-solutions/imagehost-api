use biscuit_auth::{
    error,
    macros::{biscuit, biscuit_merge},
    Biscuit, KeyPair,
};

pub fn create_token(root: &KeyPair, user_id: String) -> Result<Biscuit, error::Token> {
    let mut authority = biscuit!(
        r#"
      // parameters can directly reference in-scope variables
      user({user_id});

      // parameters can be manually supplied as well
      right({user_id}, "file1", {operation});
      "#,
        operation = "read",
    );

    // it is possible to modify a builder by adding a datalog snippet
    biscuit_merge!(&mut authority, r#"check if operation("read");"#);

    authority.build(&root)
}
