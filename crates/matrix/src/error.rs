//! Errors that can be sent from the homeserver.
//!
//! reference: https://docs.rs/ruma/latest/ruma/api/client/error/index.html

use std::{collections::BTreeMap, fmt, sync::Arc, time::Duration};

use as_variant::as_variant;
use bytes::{BufMut, Bytes};
use ruma_common::{
    api::{
        error::{FromHttpResponseError, IntoHttpError, MatrixErrorBody},
        EndpointError, OutgoingResponse,
    },
    RoomVersionId,
};
use serde::{Deserialize, Serialize};
use serde_json::{from_slice as from_json_slice, Value as JsonValue};

use std::{borrow::Cow, collections::btree_map::Entry};

use ruma_common::serde::{DeserializeFromCowStr, FromString};
use serde::{
    de::{self, Deserializer, MapAccess, Visitor},
    ser::{self, SerializeMap, Serializer},
};
use serde_json::from_value as from_json_value;

#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorKind {
    Forbidden,

    UnknownToken {
        soft_logout: bool,
    },

    MissingToken,

    BadJson,

    NotJson,

    NotFound,

    LimitExceeded {
        retry_after_ms: Option<Duration>,
    },

    Unknown,

    Unrecognized,

    Unauthorized,

    UserDeactivated,

    UserInUse,

    InvalidUsername,

    RoomInUse,

    InvalidRoomState,

    ThreepidInUse,

    ThreepidNotFound,

    ThreepidAuthFailed,

    ThreepidDenied,

    ServerNotTrusted,

    UnsupportedRoomVersion,

    IncompatibleRoomVersion {
        room_version: RoomVersionId,
    },

    BadState,

    GuestAccessForbidden,

    CaptchaNeeded,

    CaptchaInvalid,

    MissingParam,

    InvalidParam,

    TooLarge,

    Exclusive,

    ResourceLimitExceeded {
        admin_contact: String,
    },

    CannotLeaveServerNoticeRoom,

    WeakPassword,

    UnableToAuthorizeJoin,

    UnableToGrantJoin,

    BadAlias,

    DuplicateAnnotation,

    NotYetUploaded,

    CannotOverwriteMedia,

    UrlNotSet,

    BadStatus {
        status: Option<http::StatusCode>,

        body: Option<String>,
    },

    ConnectionFailed,

    ConnectionTimeout,

    WrongRoomKeysVersion {
        current_version: Option<String>,
    },

    _Custom {
        errcode: PrivOwnedStr,
        extra: Extra,
    },
}

#[doc(hidden)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Extra(BTreeMap<String, JsonValue>);

impl AsRef<str> for ErrorKind {
    fn as_ref(&self) -> &str {
        match self {
            Self::Forbidden => "M_FORBIDDEN",
            Self::UnknownToken { .. } => "M_UNKNOWN_TOKEN",
            Self::MissingToken => "M_MISSING_TOKEN",
            Self::BadJson => "M_BAD_JSON",
            Self::NotJson => "M_NOT_JSON",
            Self::NotFound => "M_NOT_FOUND",
            Self::LimitExceeded { .. } => "M_LIMIT_EXCEEDED",
            Self::Unknown => "M_UNKNOWN",
            Self::Unrecognized => "M_UNRECOGNIZED",
            Self::Unauthorized => "M_UNAUTHORIZED",
            Self::UserDeactivated => "M_USER_DEACTIVATED",
            Self::UserInUse => "M_USER_IN_USE",
            Self::InvalidUsername => "M_INVALID_USERNAME",
            Self::RoomInUse => "M_ROOM_IN_USE",
            Self::InvalidRoomState => "M_INVALID_ROOM_STATE",
            Self::ThreepidInUse => "M_THREEPID_IN_USE",
            Self::ThreepidNotFound => "M_THREEPID_NOT_FOUND",
            Self::ThreepidAuthFailed => "M_THREEPID_AUTH_FAILED",
            Self::ThreepidDenied => "M_THREEPID_DENIED",
            Self::ServerNotTrusted => "M_SERVER_NOT_TRUSTED",
            Self::UnsupportedRoomVersion => "M_UNSUPPORTED_ROOM_VERSION",
            Self::IncompatibleRoomVersion { .. } => "M_INCOMPATIBLE_ROOM_VERSION",
            Self::BadState => "M_BAD_STATE",
            Self::GuestAccessForbidden => "M_GUEST_ACCESS_FORBIDDEN",
            Self::CaptchaNeeded => "M_CAPTCHA_NEEDED",
            Self::CaptchaInvalid => "M_CAPTCHA_INVALID",
            Self::MissingParam => "M_MISSING_PARAM",
            Self::InvalidParam => "M_INVALID_PARAM",
            Self::TooLarge => "M_TOO_LARGE",
            Self::Exclusive => "M_EXCLUSIVE",
            Self::ResourceLimitExceeded { .. } => "M_RESOURCE_LIMIT_EXCEEDED",
            Self::CannotLeaveServerNoticeRoom => "M_CANNOT_LEAVE_SERVER_NOTICE_ROOM",
            Self::WeakPassword => "M_WEAK_PASSWORD",
            Self::UnableToAuthorizeJoin => "M_UNABLE_TO_AUTHORISE_JOIN",
            Self::UnableToGrantJoin => "M_UNABLE_TO_GRANT_JOIN",
            Self::BadAlias => "M_BAD_ALIAS",
            Self::DuplicateAnnotation => "M_DUPLICATE_ANNOTATION",
            Self::NotYetUploaded => "M_NOT_YET_UPLOADED",
            Self::CannotOverwriteMedia => "M_CANNOT_OVERWRITE_MEDIA",
            Self::UrlNotSet => "M_URL_NOT_SET",
            Self::BadStatus { .. } => "M_BAD_STATUS",
            Self::ConnectionFailed => "M_CONNECTION_FAILED",
            Self::ConnectionTimeout => "M_CONNECTION_TIMEOUT",
            Self::WrongRoomKeysVersion { .. } => "M_WRONG_ROOM_KEYS_VERSION",
            Self::_Custom { errcode, .. } => &errcode.0,
        }
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Clone)]
#[allow(clippy::exhaustive_enums)]
pub enum ErrorBody {
    Standard {
        kind: ErrorKind,

        message: String,
    },

    Json(JsonValue),

    #[non_exhaustive]
    NotJson {
        bytes: Bytes,

        deserialization_error: Arc<serde_json::Error>,
    },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[allow(clippy::exhaustive_structs)]
pub struct StandardErrorBody {
    #[serde(flatten)]
    pub kind: ErrorKind,

    #[serde(rename = "error")]
    pub message: String,
}

#[derive(Debug, Clone)]
#[allow(clippy::exhaustive_structs)]
pub struct Error {
    pub status_code: http::StatusCode,

    pub body: ErrorBody,
}

impl Error {
    pub fn error_kind(&self) -> Option<&ErrorKind> {
        as_variant!(&self.body, ErrorBody::Standard { kind, .. } => kind)
    }
}

impl EndpointError for Error {
    fn from_http_response<T: AsRef<[u8]>>(response: http::Response<T>) -> Self {
        let status = response.status();

        let body_bytes = &response.body().as_ref();
        let error_body: ErrorBody = match from_json_slice(body_bytes) {
            Ok(StandardErrorBody { kind, message }) => ErrorBody::Standard { kind, message },
            Err(_) => match MatrixErrorBody::from_bytes(body_bytes) {
                MatrixErrorBody::Json(json) => ErrorBody::Json(json),
                MatrixErrorBody::NotJson {
                    bytes,
                    deserialization_error,
                    ..
                } => ErrorBody::NotJson {
                    bytes,
                    deserialization_error,
                },
            },
        };

        error_body.into_error(status)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status_code = self.status_code.as_u16();
        match &self.body {
            ErrorBody::Standard { kind, message } => {
                write!(f, "[{status_code} / {kind}] {message}")
            }
            ErrorBody::Json(json) => write!(f, "[{status_code}] {json}"),
            ErrorBody::NotJson { .. } => write!(f, "[{status_code}] <non-json bytes>"),
        }
    }
}

impl std::error::Error for Error {}

impl ErrorBody {
    pub fn into_error(self, status_code: http::StatusCode) -> Error {
        Error {
            status_code,
            body: self,
        }
    }
}

impl OutgoingResponse for Error {
    fn try_into_http_response<T: Default + BufMut>(
        self,
    ) -> Result<http::Response<T>, IntoHttpError> {
        let builder = http::Response::builder()
            .header(http::header::CONTENT_TYPE, "application/json")
            .status(self.status_code);

        builder
            .body(match self.body {
                ErrorBody::Standard { kind, message } => {
                    ruma_common::serde::json_to_buf(&StandardErrorBody { kind, message })?
                }
                ErrorBody::Json(json) => ruma_common::serde::json_to_buf(&json)?,
                ErrorBody::NotJson { .. } => {
                    return Err(IntoHttpError::Json(serde::ser::Error::custom(
                        "attempted to serialize ErrorBody::NotJson",
                    )));
                }
            })
            .map_err(Into::into)
    }
}

pub trait FromHttpResponseErrorExt {
    fn error_kind(&self) -> Option<&ErrorKind>;
}

impl FromHttpResponseErrorExt for FromHttpResponseError<Error> {
    fn error_kind(&self) -> Option<&ErrorKind> {
        as_variant!(self, Self::Server)?.error_kind()
    }
}

enum Field<'de> {
    ErrCode,
    SoftLogout,
    RetryAfterMs,
    RoomVersion,
    AdminContact,
    Status,
    Body,
    CurrentVersion,
    Other(Cow<'de, str>),
}

impl<'de> Field<'de> {
    fn new(s: Cow<'de, str>) -> Field<'de> {
        match s.as_ref() {
            "errcode" => Self::ErrCode,
            "soft_logout" => Self::SoftLogout,
            "retry_after_ms" => Self::RetryAfterMs,
            "room_version" => Self::RoomVersion,
            "admin_contact" => Self::AdminContact,
            "status" => Self::Status,
            "body" => Self::Body,
            "current_version" => Self::CurrentVersion,
            _ => Self::Other(s),
        }
    }
}

impl<'de> Deserialize<'de> for Field<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Field<'de>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FieldVisitor;

        impl<'de> Visitor<'de> for FieldVisitor {
            type Value = Field<'de>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("any struct field")
            }

            fn visit_str<E>(self, value: &str) -> Result<Field<'de>, E>
            where
                E: de::Error,
            {
                Ok(Field::new(Cow::Owned(value.to_owned())))
            }

            fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Field<'de>, E>
            where
                E: de::Error,
            {
                Ok(Field::new(Cow::Borrowed(value)))
            }

            fn visit_string<E>(self, value: String) -> Result<Field<'de>, E>
            where
                E: de::Error,
            {
                Ok(Field::new(Cow::Owned(value)))
            }
        }

        deserializer.deserialize_identifier(FieldVisitor)
    }
}

struct ErrorKindVisitor;

impl<'de> Visitor<'de> for ErrorKindVisitor {
    type Value = ErrorKind;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("enum ErrorKind")
    }

    fn visit_map<V>(self, mut map: V) -> Result<ErrorKind, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut errcode = None;
        let mut soft_logout = None;
        let mut retry_after_ms = None;
        let mut room_version = None;
        let mut admin_contact = None;
        let mut status = None;
        let mut body = None;
        let mut current_version = None;
        let mut extra = BTreeMap::new();

        macro_rules! set_field {
            (errcode) => {
                set_field!(@inner errcode)
            };
            ($field:ident) => {
                match errcode {
                    Some(set_field!(@variant_containing $field)) | None => {
                        set_field!(@inner $field)
                    }
                    // if we already know we're deserializing a different variant to the one
                    // containing this field, ignore its value.
                    Some(_) => {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    },
                }
            };
            (@variant_containing soft_logout) => { ErrCode::UnknownToken };
            (@variant_containing retry_after_ms) => { ErrCode::LimitExceeded };
            (@variant_containing room_version) => { ErrCode::IncompatibleRoomVersion };
            (@variant_containing admin_contact) => { ErrCode::ResourceLimitExceeded };
            (@variant_containing status) => { ErrCode::BadStatus };
            (@variant_containing body) => { ErrCode::BadStatus };
            (@variant_containing current_version) => { ErrCode::WrongRoomKeysVersion };
            (@inner $field:ident) => {
                {
                    if $field.is_some() {
                        return Err(de::Error::duplicate_field(stringify!($field)));
                    }
                    $field = Some(map.next_value()?);
                }
            };
        }

        while let Some(key) = map.next_key()? {
            match key {
                Field::ErrCode => set_field!(errcode),
                Field::SoftLogout => set_field!(soft_logout),
                Field::RetryAfterMs => set_field!(retry_after_ms),
                Field::RoomVersion => set_field!(room_version),
                Field::AdminContact => set_field!(admin_contact),
                Field::Status => set_field!(status),
                Field::Body => set_field!(body),
                Field::CurrentVersion => set_field!(current_version),
                Field::Other(other) => match extra.entry(other.into_owned()) {
                    Entry::Vacant(v) => {
                        v.insert(map.next_value()?);
                    }
                    Entry::Occupied(o) => {
                        return Err(de::Error::custom(format!("duplicate field `{}`", o.key())));
                    }
                },
            }
        }

        let errcode = errcode.ok_or_else(|| de::Error::missing_field("errcode"))?;
        let extra = Extra(extra);

        Ok(match errcode {
            ErrCode::Forbidden => ErrorKind::Forbidden,
            ErrCode::UnknownToken => ErrorKind::UnknownToken {
                soft_logout: soft_logout
                    .map(from_json_value)
                    .transpose()
                    .map_err(de::Error::custom)?
                    .unwrap_or_default(),
            },
            ErrCode::MissingToken => ErrorKind::MissingToken,
            ErrCode::BadJson => ErrorKind::BadJson,
            ErrCode::NotJson => ErrorKind::NotJson,
            ErrCode::NotFound => ErrorKind::NotFound,
            ErrCode::LimitExceeded => ErrorKind::LimitExceeded {
                retry_after_ms: retry_after_ms
                    .map(from_json_value::<u64>)
                    .transpose()
                    .map_err(de::Error::custom)?
                    .map(Into::into)
                    .map(Duration::from_millis),
            },
            ErrCode::Unknown => ErrorKind::Unknown,
            ErrCode::Unrecognized => ErrorKind::Unrecognized,
            ErrCode::Unauthorized => ErrorKind::Unauthorized,
            ErrCode::UserDeactivated => ErrorKind::UserDeactivated,
            ErrCode::UserInUse => ErrorKind::UserInUse,
            ErrCode::InvalidUsername => ErrorKind::InvalidUsername,
            ErrCode::RoomInUse => ErrorKind::RoomInUse,
            ErrCode::InvalidRoomState => ErrorKind::InvalidRoomState,
            ErrCode::ThreepidInUse => ErrorKind::ThreepidInUse,
            ErrCode::ThreepidNotFound => ErrorKind::ThreepidNotFound,
            ErrCode::ThreepidAuthFailed => ErrorKind::ThreepidAuthFailed,
            ErrCode::ThreepidDenied => ErrorKind::ThreepidDenied,
            ErrCode::ServerNotTrusted => ErrorKind::ServerNotTrusted,
            ErrCode::UnsupportedRoomVersion => ErrorKind::UnsupportedRoomVersion,
            ErrCode::IncompatibleRoomVersion => ErrorKind::IncompatibleRoomVersion {
                room_version: from_json_value(
                    room_version.ok_or_else(|| de::Error::missing_field("room_version"))?,
                )
                .map_err(de::Error::custom)?,
            },
            ErrCode::BadState => ErrorKind::BadState,
            ErrCode::GuestAccessForbidden => ErrorKind::GuestAccessForbidden,
            ErrCode::CaptchaNeeded => ErrorKind::CaptchaNeeded,
            ErrCode::CaptchaInvalid => ErrorKind::CaptchaInvalid,
            ErrCode::MissingParam => ErrorKind::MissingParam,
            ErrCode::InvalidParam => ErrorKind::InvalidParam,
            ErrCode::TooLarge => ErrorKind::TooLarge,
            ErrCode::Exclusive => ErrorKind::Exclusive,
            ErrCode::ResourceLimitExceeded => ErrorKind::ResourceLimitExceeded {
                admin_contact: from_json_value(
                    admin_contact.ok_or_else(|| de::Error::missing_field("admin_contact"))?,
                )
                .map_err(de::Error::custom)?,
            },
            ErrCode::CannotLeaveServerNoticeRoom => ErrorKind::CannotLeaveServerNoticeRoom,
            ErrCode::WeakPassword => ErrorKind::WeakPassword,
            ErrCode::UnableToAuthorizeJoin => ErrorKind::UnableToAuthorizeJoin,
            ErrCode::UnableToGrantJoin => ErrorKind::UnableToGrantJoin,
            ErrCode::BadAlias => ErrorKind::BadAlias,
            ErrCode::DuplicateAnnotation => ErrorKind::DuplicateAnnotation,
            ErrCode::NotYetUploaded => ErrorKind::NotYetUploaded,
            ErrCode::CannotOverwriteMedia => ErrorKind::CannotOverwriteMedia,
            ErrCode::UrlNotSet => ErrorKind::UrlNotSet,
            ErrCode::BadStatus => ErrorKind::BadStatus {
                status: status
                    .map(|s| {
                        from_json_value::<u16>(s)
                            .map_err(de::Error::custom)?
                            .try_into()
                            .map_err(de::Error::custom)
                    })
                    .transpose()?,
                body: body
                    .map(from_json_value)
                    .transpose()
                    .map_err(de::Error::custom)?,
            },
            ErrCode::ConnectionFailed => ErrorKind::ConnectionFailed,
            ErrCode::ConnectionTimeout => ErrorKind::ConnectionTimeout,
            ErrCode::WrongRoomKeysVersion => ErrorKind::WrongRoomKeysVersion {
                current_version: from_json_value(
                    current_version.ok_or_else(|| de::Error::missing_field("current_version"))?,
                )
                .map_err(de::Error::custom)?,
            },
            ErrCode::_Custom(errcode) => ErrorKind::_Custom { errcode, extra },
        })
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PrivOwnedStr(Box<str>);

impl fmt::Debug for PrivOwnedStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(FromString, DeserializeFromCowStr)]
#[ruma_enum(rename_all = "M_MATRIX_ERROR_CASE")]
enum ErrCode {
    Forbidden,
    UnknownToken,
    MissingToken,
    BadJson,
    NotJson,
    NotFound,
    LimitExceeded,
    Unknown,
    Unrecognized,
    Unauthorized,
    UserDeactivated,
    UserInUse,
    InvalidUsername,
    RoomInUse,
    InvalidRoomState,
    ThreepidInUse,
    ThreepidNotFound,
    ThreepidAuthFailed,
    ThreepidDenied,
    ServerNotTrusted,
    UnsupportedRoomVersion,
    IncompatibleRoomVersion,
    BadState,
    GuestAccessForbidden,
    CaptchaNeeded,
    CaptchaInvalid,
    MissingParam,
    InvalidParam,
    TooLarge,
    Exclusive,
    ResourceLimitExceeded,
    CannotLeaveServerNoticeRoom,
    WeakPassword,
    UnableToAuthorizeJoin,
    UnableToGrantJoin,
    BadAlias,
    DuplicateAnnotation,
    #[ruma_enum(alias = "FI.MAU.MSC2246_NOT_YET_UPLOADED")]
    NotYetUploaded,
    #[ruma_enum(alias = "FI.MAU.MSC2246_CANNOT_OVERWRITE_MEDIA")]
    CannotOverwriteMedia,
    UrlNotSet,
    BadStatus,
    ConnectionFailed,
    ConnectionTimeout,
    WrongRoomKeysVersion,
    _Custom(PrivOwnedStr),
}

impl<'de> Deserialize<'de> for ErrorKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(ErrorKindVisitor)
    }
}

impl Serialize for ErrorKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_map(None)?;
        st.serialize_entry("errcode", self.as_ref())?;
        match self {
            Self::UnknownToken { soft_logout: true } => {
                st.serialize_entry("soft_logout", &true)?;
            }
            Self::LimitExceeded {
                retry_after_ms: Some(duration),
            } => {
                st.serialize_entry(
                    "retry_after_ms",
                    &u64::try_from(duration.as_millis()).map_err(ser::Error::custom)?,
                )?;
            }
            Self::IncompatibleRoomVersion { room_version } => {
                st.serialize_entry("room_version", room_version)?;
            }
            Self::ResourceLimitExceeded { admin_contact } => {
                st.serialize_entry("admin_contact", admin_contact)?;
            }
            Self::_Custom { extra, .. } => {
                for (k, v) in &extra.0 {
                    st.serialize_entry(k, v)?;
                }
            }
            _ => {}
        }
        st.end()
    }
}
