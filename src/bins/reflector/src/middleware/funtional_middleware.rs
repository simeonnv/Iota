#[macro_export]
macro_rules! funtional_middleware {
    ($middleware_func:ident) => {{
        actix_web::middleware::from_fn($middleware_func)
    }};
    ($middleware_func:ident, $($arg:expr),+) => {{
        let args = ($($arg),+);
        actix_web::middleware::from_fn(move |req, next| {
            let args = args.clone();
            $middleware_func(req, next, args)
        })
    }};
}
