macro_rules! ruma_route {
    ($( $path:ident )::* = $action:ident @ $ver:ident => $block:block) => {
        use $($path::)*$action;
        use $crate::api::ruma::{Ar, Ra};

        pub(crate) async fn $action(
            #[allow(unused_variables)]
            request: Ar<$action::$ver::Request>,
        ) -> Result<
            Ra<$action::$ver::Response>, $crate::Error
        > $block
    };
}

ruma_route!(
    ruma::api::appservice::ping = send_ping @ v1 => {
       Ok(Ra(send_ping::v1::Response::new()))
    }
);
