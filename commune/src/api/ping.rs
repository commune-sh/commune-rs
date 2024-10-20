use crate::ruma_route;

ruma_route!(
    appservice::ping = send_ping @ v1 => |_state, request| {
        &request.transaction_id;

        Ok(Ra(send_ping::v1::Response::new()))
    }
);
