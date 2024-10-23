use crate::ruma_route;

ruma_route!(
    get_info => |_state, request| {
        &request.event_id;

        Ok(Ra(get_info::Response::new()))
    }
);

pub(crate) mod get_info {
    use ruma::{
        api::{request, response, Metadata},
        metadata, OwnedEventId, OwnedRoomId,
    };

    const METADATA: Metadata = metadata! {
        method: GET,
        rate_limited: false,
        authentication: AccessToken,
        history: {
            unstable => "/_matrix/client/v3/rooms/{room_id}/info",
        }
    };

    /// Request type for the `get_info` endpoint.
    #[request]
    pub(crate) struct Request {
        #[ruma_api(path)]
        pub(crate) room_id: OwnedRoomId,
        #[ruma_api(query)]
        pub(crate) event_id: OwnedEventId,
        #[ruma_api(query)]
        pub(crate) child: OwnedRoomId,
    }

    /// Response type for the `get_info` endpoint.
    #[response]
    #[derive(Default)]
    pub(crate) struct Response {}

    impl Request {
        /// Creates a new empty `Request`.
        pub(crate) fn new(
            room_id: OwnedRoomId,
            event_id: OwnedEventId,
            child: OwnedRoomId,
        ) -> Self {
            Self {
                room_id,
                event_id,
                child,
            }
        }
    }

    impl Response {
        /// Creates a new empty `Response`.
        pub(crate) fn new() -> Self {
            Self::default()
        }
    }
}
