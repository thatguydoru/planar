use rinja::Template;

#[derive(Template, Default)]
#[template(path = "auth/partial/signup.html")]
pub struct SignupPartial<'a> {
    pub username: Option<&'a str>,
}
