use crate::ruma_route;

ruma_route!(
    appservice::event = push_events @ v1 => |state, request| {
        &request.inner.events;

        Ok(Ra(push_events::v1::Response::new()))
    }
);
