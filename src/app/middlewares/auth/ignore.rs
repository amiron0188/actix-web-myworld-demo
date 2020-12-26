lazy_static! {
    pub static ref IGNORE_AUTH_ROUTES: Vec<String>  = {
        return vec![
            "POST /api/auth/login".to_string(),
            "POST /api/user/register".to_string()
        ];
    };
}
