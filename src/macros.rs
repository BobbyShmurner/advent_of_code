macro_rules! err_from_str {
    ($($args:tt)*) => {
        Box::new(simple_error::SimpleError::new(format!($($args)*)))
    };
}

macro_rules! convert_to_err {
    ($err:tt) => {{
        trait AlreadyError {
            fn convert_to_error(self) -> Self
			where
				Self: Sized {
                self
            }
        }

        impl<T: ?Sized> AlreadyError for Box<T>
        where
            T: std::error::Error,
        {
        }

        trait CreateErrFromString {
            fn convert_to_error(&self) -> Box<simple_error::SimpleError>
            where
                Self: std::fmt::Display,
            {
                err_from_str!("{}", self)
            }
        }
        impl CreateErrFromString for &str {}
		impl CreateErrFromString for String {}

        $err.convert_to_error()
    }};

	($($args:tt)*) => {
		err_from_str!($($args)*)
	}
}

macro_rules! return_err {
    ($($args:tt)*) => {
        return Err((convert_to_err!($($args)*)))
    };
}

macro_rules! unwrap_or_else_custom {
    ($val:expr, $ok:tt, $err:pat, $code:block) => {
        match $val {
            $ok(val) => val,
            $err => $code,
        }
    };
}

macro_rules! unwrap_or_return_custom {
    ($val:expr, $ok:tt, $err:pat, $($args:expr),*) => {
		unwrap_or_else_custom!($val, $ok, $err, {
			return_err!($($args),*)
		})
	};
}

macro_rules! unwrap_or_else {
    ($val:expr, error: $err:tt, $code:block) => {
        unwrap_or_else_custom!($val, Ok, Err($err), $code)
    };
    ($val:expr, $code:block) => {
        unwrap_or_else_custom!($val, Ok, Err(_), $code)
    };
}

macro_rules! unwrap_or_return {
	($val:expr, error: $err:tt, $($args:expr),*) => {
		unwrap_or_return_custom!($val, Ok, Err($err), $($args),*)
	};
    ($val:expr, $($args:expr),*) => {
		unwrap_or_return_custom!($val, Ok, Err(_), $($args),*)
	};
	($val:expr) => {
		unwrap_or_return_custom!($val, Ok, Err(e), e)
	};
}

macro_rules! _unwrap_or_else_option {
    ($val:expr, $($args:expr),*) => {
		unwrap_or_else_custom!($val, Some, None, $($args),*)
	};
}

macro_rules! unwrap_or_return_option {
    ($val:expr, $($args:expr),*) => {
		unwrap_or_return_custom!($val, Some, None, $($args),*)
	};
}

pub(crate) use convert_to_err;
pub(crate) use err_from_str;
pub(crate) use return_err;
pub(crate) use unwrap_or_else;
pub(crate) use unwrap_or_else_custom;
pub(crate) use unwrap_or_return;
pub(crate) use unwrap_or_return_custom;
pub(crate) use unwrap_or_return_option;
