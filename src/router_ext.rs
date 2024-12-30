/// Naive macro to add a route and its trailing slash variant to the router
macro_rules! add_route {
    ($router:ident, $route:literal) => {
        assert!(
            !$route.ends_with("/"),
            "URLs passed to add_route must not end in /"
        );
        $router.insert($route, ()).unwrap();
        $router.insert(concat!($route, "/"), ()).unwrap();
    };
}

// Required for the macro to be visible outside this module
pub(crate) use add_route;
