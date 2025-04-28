/// Easier [`Parameters`] construction.
///
/// [`Parameters`]: super::Parameters
///
/// ```rust
/// # use vicardi::parameters;
/// let vec = vec!["".into()];
/// parameters! {
///     "language" => "en",
///     String::from("sort-as") => [String::from("foo"), "bar"],
///     "vec" => vec,
/// };
/// ```
#[macro_export]
macro_rules! parameters {
    (
        $($key:expr => $value:tt),* $(,)?
    ) => {{
        let mut m = $crate::Parameters::new();
        $(
            let v = parameters!(@val $value);
            m.insert($key.to_string(), v);
        )*
        m
    }};

    (@val [$($val:expr),* $(,)?]) => {{
        let val: Vec<String> = vec![$($val.into()),*];
        val
    }};

    (@val $val:expr) => {{
        use $crate::macros::Veclike;
        Veclike($val).to_vec()
    }};
}

pub struct Veclike<T>(pub T);

impl Veclike<String> {
    pub fn to_vec(self) -> Vec<String> {
        vec![self.0]
    }
}

impl Veclike<&str> {
    pub fn to_vec(self) -> Vec<String> {
        vec![self.0.to_string()]
    }
}

impl Veclike<Vec<String>> {
    pub fn to_vec(self) -> Vec<String> {
        self.0
    }
}
