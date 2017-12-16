//! # HTTP-API-PROBLEM
//!
//! [![crates.io](https://img.shields.io/crates/v/http-api-problem.svg)](https://crates.io/crates/http-api-problem)
//! [![docs.rs](https://docs.rs/http-api-problem/badge.svg)](https://docs.rs/http-api-problem)
//! [![downloads](https://img.shields.io/crates/d/http-api-problem.svg)](https://crates.io/crates/http-api-problem)
//! [![Build Status](https://travis-ci.org/chridou/http-api-problem.svg?branch=master)](https://travis-ci.
//! org/chridou/http-api-problem)
//! [![license-mit](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.
//! com/chridou/http-api-problem/blob/master/LICENSE-MIT)
//! [![license-apache](http://img.shields.io/badge/license-APACHE-blue.svg)]
//! (https://github.com/chridou/http-api-problem/blob/master/LICENSE-APACHE)
//!
//! A library to create HTTP response content for APIs based on
//! [RFC7807](https://tools.ietf.org/html/rfc7807).
//!
//! This library depends on [serde](https://serde.rs/).
//!
//! The `HttpApiProblem` struct implements `Serialize` and `Deserialize`.
//!
//! ## Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! http-api-problem = "0.2"
//! ```
//!
//! Add this crate root:
//!
//! ```rust
//! extern crate http_api_problem;
//! ```
//!
//! ## Features
//!
//! To directly construct from `[iron] StatusCode` the `feature` `iron` implements `From`
//! for `HttpStatusCode` of this library.
//!
//! ## Example
//!
//! ```rust
//! use http_api_problem::*;
//!
//! let p =
//!     HttpApiProblem::with_title_and_type_from_status(428)
//!     .set_detail("detailed explanation")
//!     .set_instance("/on/1234/do/something");
//!
//! assert_eq!(Some("https://httpstatuses.com/428".to_string()), p.type_url);
//! assert_eq!(Some(428), p.status);
//! assert_eq!("Precondition Required".to_string(), p.title);
//! assert_eq!(Some("detailed explanation".to_string()), p.detail);
//! assert_eq!(Some("/on/1234/do/something".to_string()), p.instance);
//! ```
//!
//! ## License
//!
//! `http-api-problem` is primarily distributed under the terms of both the MIT license and the
//! Apache License (Version 2.0).
//!
//! Copyright (c) 2017 Christian Douven.

extern crate serde;
#[macro_use]
extern crate serde_derive;

#[cfg(feature = "iron")]
extern crate iron;

use std::fmt;

/// The recommended media type when serialized to JSON
pub static PROBLEM_JSON_MEDIA_TYPE: &'static str = "application/problem+json";

/// The recommended media type when serialized to XML
pub static PROBLEM_XML_MEDIA_TYPE: &'static str = "application/problem+xml";

/// Description of a problem that can be returned by an HTTP API
/// based on [RFC7807](https://tools.ietf.org/html/rfc7807)
///
/// # Example
///
/// ```javascript
/// {
///    "type": "https://example.com/probs/out-of-credit",
///    "title": "You do not have enough credit.",
///    "detail": "Your current balance is 30, but that costs 50.",
///    "instance": "/account/12345/msgs/abc",
/// }
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct HttpApiProblem {
    /// A URI reference [RFC3986](https://tools.ietf.org/html/rfc3986) that identifies the
    /// problem type.  This specification encourages that, when
    /// dereferenced, it provide human-readable documentation for the
    /// problem type (e.g., using HTML [W3C.REC-html5-20141028]).  When
    /// this member is not present, its value is assumed to be
    /// "about:blank".
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_url: Option<String>,
    /// The HTTP status code [RFC7231, Section 6](https://tools.ietf.org/html/rfc7231#section-6)
    /// generated by the origin server for this occurrence of the problem.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<u16>,
    /// A short, human-readable summary of the problem
    /// type. It SHOULD NOT change from occurrence to occurrence of the
    /// problem, except for purposes of localization (e.g., using
    /// proactive content negotiation;
    /// see [RFC7231, Section 3.4](https://tools.ietf.org/html/rfc7231#section-3.4).
    ///
    /// This is the only mandatory field.
    pub title: String,
    /// A human-readable explanation specific to this
    /// occurrence of the problem.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// A URI reference that identifies the specific
    /// occurrence of the problem.  It may or may not yield further
    /// information if dereferenced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
}

impl HttpApiProblem {
    /// Creates a new instance with the given `title`.
    ///
    /// #Example
    ///
    /// ```rust
    /// use http_api_problem::*;
    ///
    /// let p = HttpApiProblem::new("Internal Error");
    ///
    /// assert_eq!(None, p.type_url);
    /// assert_eq!(None, p.status);
    /// assert_eq!("Internal Error", p.title);
    /// assert_eq!(None, p.detail);
    /// assert_eq!(None, p.instance);
    /// ```
    pub fn new<T: Into<String>>(title: T) -> HttpApiProblem {
        HttpApiProblem {
            type_url: None,
            status: None,
            title: title.into(),
            detail: None,
            instance: None,
        }
    }

    /// Creates a new instance with the `title` and `type_url` derived from the `status`.
    ///
    /// #Example
    ///
    /// ```rust
    /// use http_api_problem::*;
    ///
    /// let p = HttpApiProblem::with_title_and_type_from_status(503);
    ///
    /// assert_eq!(Some("https://httpstatuses.com/503".to_string()), p.type_url);
    /// assert_eq!(Some(503), p.status);
    /// assert_eq!("Service Unavailable", &p.title);
    /// assert_eq!(None, p.detail);
    /// assert_eq!(None, p.instance);
    /// ```
    pub fn with_title_and_type_from_status<T: Into<HttpStatusCode>>(status: T) -> HttpApiProblem {
        let status = status.into();
        HttpApiProblem {
            type_url: Some(format!("https://httpstatuses.com/{}", status.to_u16())),
            status: Some(status.to_u16()),
            title: status.title().to_string(),
            detail: None,
            instance: None,
        }
    }

    /// Creates a new instance with `title` derived from `status`.
    ///
    /// #Example
    ///
    /// ```rust
    /// use http_api_problem::*;
    ///
    /// let p = HttpApiProblem::with_title_from_status(404);
    ///
    /// assert_eq!(None, p.type_url);
    /// assert_eq!(Some(404), p.status);
    /// assert_eq!("Not Found", p.title);
    /// assert_eq!(None, p.detail);
    /// assert_eq!(None, p.instance);
    /// ```
    pub fn with_title_from_status<T: Into<HttpStatusCode>>(status: T) -> HttpApiProblem {
        let status = status.into();
        HttpApiProblem {
            type_url: None,
            status: Some(status.to_u16()),
            title: status.title().to_string(),
            detail: None,
            instance: None,
        }
    }

    /// Sets the `type_url`
    ///
    /// #Example
    ///
    /// ```rust
    /// use http_api_problem::*;
    ///
    /// let p =
    ///     HttpApiProblem::new("Error")
    ///     .set_type_url("http://example.com/my/real_error");
    ///
    /// assert_eq!(Some("http://example.com/my/real_error".to_string()), p.type_url);
    /// assert_eq!(None, p.status);
    /// assert_eq!("Error", p.title);
    /// assert_eq!(None, p.detail);
    /// assert_eq!(None, p.instance);
    /// ```
    pub fn set_type_url<T: Into<String>>(self, type_url: T) -> HttpApiProblem {
        let mut s = self;
        s.type_url = Some(type_url.into());
        s
    }


    /// Sets the `status`
    ///
    /// #Example
    ///
    /// ```rust
    /// use http_api_problem::*;
    ///
    /// let p = HttpApiProblem::new("Error").set_status(404);
    ///
    /// assert_eq!(None, p.type_url);
    /// assert_eq!(Some(404), p.status);
    /// assert_eq!("Error", p.title);
    /// assert_eq!(None, p.detail);
    /// assert_eq!(None, p.instance);
    /// ```
    pub fn set_status<T: Into<HttpStatusCode>>(self, status: T) -> HttpApiProblem {
        let status = status.into();
        let mut s = self;
        s.status = Some(status.to_u16());
        s
    }

    /// Sets the `title`
    ///
    /// #Example
    ///
    /// ```rust
    /// use http_api_problem::*;
    ///
    /// let p = HttpApiProblem::new("Error").set_title("Another Error");
    ///
    /// assert_eq!(None, p.type_url);
    /// assert_eq!(None, p.status);
    /// assert_eq!("Another Error", p.title);
    /// assert_eq!(None, p.detail);
    /// assert_eq!(None, p.instance);
    /// ```
    pub fn set_title<T: Into<String>>(self, title: T) -> HttpApiProblem {
        let mut s = self;
        s.title = title.into();
        s
    }

    /// Sets the `detail`
    ///
    /// #Example
    ///
    /// ```rust
    /// use http_api_problem::*;
    ///
    /// let p =
    ///     HttpApiProblem::new("Error")
    ///     .set_detail("a detailed description");
    ///
    /// assert_eq!(None, p.type_url);
    /// assert_eq!(None, p.status);
    /// assert_eq!("Error", p.title);
    /// assert_eq!(Some("a detailed description".to_string()), p.detail);
    /// assert_eq!(None, p.instance);
    /// ```
    pub fn set_detail<T: Into<String>>(self, detail: T) -> HttpApiProblem {
        let mut s = self;
        s.detail = Some(detail.into());
        s
    }

    /// Sets the `instance`
    ///
    /// #Example
    ///
    /// ```rust
    /// use http_api_problem::*;
    ///
    /// let p =
    ///     HttpApiProblem::new("Error")
    ///     .set_instance("/account/1234/withdraw");
    ///
    /// assert_eq!(None, p.type_url);
    /// assert_eq!(None, p.status);
    /// assert_eq!("Error", p.title);
    /// assert_eq!(None, p.detail);
    /// assert_eq!(Some("/account/1234/withdraw".to_string()), p.instance);
    /// ```
    pub fn set_instance<T: Into<String>>(self, instance: T) -> HttpApiProblem {
        let mut s = self;
        s.instance = Some(instance.into());
        s
    }
}

impl From<HttpStatusCode> for HttpApiProblem {
    fn from(status: HttpStatusCode) -> HttpApiProblem {
        HttpApiProblem::with_title_from_status(status)
    }
}

/// An HTTP status code (`status-code` in RFC 7230 et al.).
///
/// This enum contains all common status codes and an Unregistered
/// extension variant. It allows status codes in the range [0, 65535], as any
/// `u16` integer may be used as a status code for XHR requests. It is
/// recommended to only use values between [100, 599], since only these are
/// defined as valid status codes with a status class by HTTP.
///
/// If you encounter a status code that you do not know how to deal with, you
/// should treat it as the `x00` status code—e.g. for code 123, treat it as
/// 100 (Continue). This can be achieved with
/// `self.class().default_code()`:
///
/// IANA maintain the [Hypertext Transfer Protocol (HTTP) Status Code
/// Registry](http://www.iana.org/assignments/http-status-codes/http-status-codes.xhtml) which is
/// the source for this enum (with one exception, 418 I'm a teapot, which is
/// inexplicably not in the register).#[derive(Debug, Clone, Copy, PartialEq, Eq)]
///
/// Shamelessly copied from [iron](http://ironframework.io/)
pub enum HttpStatusCode {
    Continue,
    SwitchingProtocols,
    Processing,
    Ok,
    Created,
    Accepted,
    NonAuthoritativeInformation,
    NoContent,
    ResetContent,
    PartialContent,
    MultiStatus,
    AlreadyReported,
    ImUsed,
    MultipleChoices,
    MovedPermanently,
    Found,
    SeeOther,
    NotModified,
    UseProxy,
    TemporaryRedirect,
    PermanentRedirect,
    BadRequest,
    Unauthorized,
    PaymentRequired,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    ProxyAuthenticationRequired,
    RequestTimeout,
    Conflict,
    Gone,
    LengthRequired,
    PreconditionFailed,
    PayloadTooLarge,
    UriTooLong,
    UnsupportedMediaType,
    RangeNotSatisfiable,
    ExpectationFailed,
    ImATeapot,
    MisdirectedRequest,
    UnprocessableEntity,
    Locked,
    FailedDependency,
    UpgradeRequired,
    PreconditionRequired,
    TooManyRequests,
    RequestHeaderFieldsTooLarge,
    UnavailableForLegalReasons,
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
    HttpVersionNotSupported,
    VariantAlsoNegotiates,
    InsufficientStorage,
    LoopDetected,
    NotExtended,
    NetworkAuthenticationRequired,
    Unregistered(u16),
}

impl HttpStatusCode {
    pub fn title(&self) -> &'static str {
        use HttpStatusCode::*;
        match *self {
            Continue => "Continue",
            SwitchingProtocols => "Switching Protocols",
            Processing => "Processing",
            Ok => "Ok",
            Created => "Created",
            Accepted => "Accepted",
            NonAuthoritativeInformation => "Non Authoritative Information",
            NoContent => "No Content",
            ResetContent => "Reset Content",
            PartialContent => "Partial Content",
            MultiStatus => "Multi Status",
            AlreadyReported => "Already Reported",
            ImUsed => "Im Used",
            MultipleChoices => "Multiple Choices",
            MovedPermanently => "Moved Permanently",
            Found => "Found",
            SeeOther => "See Other",
            NotModified => "Not Modified",
            UseProxy => "Use Proxy",
            TemporaryRedirect => "Temporary Redirect",
            PermanentRedirect => "Permanent Redirect",
            BadRequest => "Bad Request",
            Unauthorized => "Unauthorized",
            PaymentRequired => "Payment Required",
            Forbidden => "Forbidden",
            NotFound => "Not Found",
            MethodNotAllowed => "Method Not Allowed",
            NotAcceptable => "Not Acceptable",
            ProxyAuthenticationRequired => "Proxy Authentication Required",
            RequestTimeout => "Request Timeout",
            Conflict => "Conflict",
            Gone => "Gone",
            LengthRequired => "Length Required",
            PreconditionFailed => "Precondition Failed",
            PayloadTooLarge => "Payload Too Large",
            UriTooLong => "Uri Too Long",
            UnsupportedMediaType => "Unsupported Media Type",
            RangeNotSatisfiable => "Range Not Satisfiable",
            ExpectationFailed => "Expectation Failed",
            ImATeapot => "Im A Teapot",
            MisdirectedRequest => "Misdirected Request",
            UnprocessableEntity => "Unprocessable Entity",
            Locked => "Locked",
            FailedDependency => "Failed Dependency",
            UpgradeRequired => "Upgrade Required",
            PreconditionRequired => "Precondition Required",
            TooManyRequests => "Too Many Requests",
            RequestHeaderFieldsTooLarge => "Request Header Fields Too Large",
            UnavailableForLegalReasons => "Unavailable For Legal Reasons",
            InternalServerError => "Internal Server Error",
            NotImplemented => "Not Implemented",
            BadGateway => "Bad Gateway",
            ServiceUnavailable => "Service Unavailable",
            GatewayTimeout => "Gateway Timeout",
            HttpVersionNotSupported => "HTTP Version Not Supported",
            VariantAlsoNegotiates => "Variant Also Negotiates",
            InsufficientStorage => "Insufficient Storage",
            LoopDetected => "Loop Detected",
            NotExtended => "Not Extended",
            NetworkAuthenticationRequired => "Network Authentication Required",
            Unregistered(code) => {
                if code / 100 == 4 {
                    "<Unregistered Client Error>"
                } else if code / 100 == 5 {
                    "<Unregistered Server Error>"
                } else {
                    "<Unregistered Status Code>"
                }
            }
        }
    }

    pub fn to_u16(&self) -> u16 {
        use HttpStatusCode::*;
        match *self {
            Continue => 100,
            SwitchingProtocols => 101,
            Processing => 102,
            Ok => 200,
            Created => 201,
            Accepted => 202,
            NonAuthoritativeInformation => 203,
            NoContent => 204,
            ResetContent => 205,
            PartialContent => 206,
            MultiStatus => 207,
            AlreadyReported => 208,
            ImUsed => 226,
            MultipleChoices => 300,
            MovedPermanently => 301,
            Found => 302,
            SeeOther => 303,
            NotModified => 304,
            UseProxy => 305,
            TemporaryRedirect => 307,
            PermanentRedirect => 308,
            BadRequest => 400,
            Unauthorized => 401,
            PaymentRequired => 402,
            Forbidden => 403,
            NotFound => 404,
            MethodNotAllowed => 405,
            NotAcceptable => 406,
            ProxyAuthenticationRequired => 407,
            RequestTimeout => 408,
            Conflict => 409,
            Gone => 410,
            LengthRequired => 411,
            PreconditionFailed => 412,
            PayloadTooLarge => 413,
            UriTooLong => 414,
            UnsupportedMediaType => 415,
            RangeNotSatisfiable => 416,
            ExpectationFailed => 417,
            ImATeapot => 418,
            MisdirectedRequest => 421,
            UnprocessableEntity => 422,
            Locked => 423,
            FailedDependency => 424,
            UpgradeRequired => 426,
            PreconditionRequired => 428,
            TooManyRequests => 429,
            RequestHeaderFieldsTooLarge => 431,
            UnavailableForLegalReasons => 451,
            InternalServerError => 500,
            NotImplemented => 501,
            BadGateway => 502,
            ServiceUnavailable => 503,
            GatewayTimeout => 504,
            HttpVersionNotSupported => 505,
            VariantAlsoNegotiates => 506,
            InsufficientStorage => 507,
            LoopDetected => 508,
            NotExtended => 510,
            NetworkAuthenticationRequired => 511,
            Unregistered(n) => n,
        }
    }
}

impl From<u16> for HttpStatusCode {
    fn from(n: u16) -> HttpStatusCode {
        use HttpStatusCode::*;
        match n {
            100 => Continue,
            101 => SwitchingProtocols,
            102 => Processing,
            200 => Ok,
            201 => Created,
            202 => Accepted,
            203 => NonAuthoritativeInformation,
            204 => NoContent,
            205 => ResetContent,
            206 => PartialContent,
            207 => MultiStatus,
            208 => AlreadyReported,
            226 => ImUsed,
            300 => MultipleChoices,
            301 => MovedPermanently,
            302 => Found,
            303 => SeeOther,
            304 => NotModified,
            305 => UseProxy,
            307 => TemporaryRedirect,
            308 => PermanentRedirect,
            400 => BadRequest,
            401 => Unauthorized,
            402 => PaymentRequired,
            403 => Forbidden,
            404 => NotFound,
            405 => MethodNotAllowed,
            406 => NotAcceptable,
            407 => ProxyAuthenticationRequired,
            408 => RequestTimeout,
            409 => Conflict,
            410 => Gone,
            411 => LengthRequired,
            412 => PreconditionFailed,
            413 => PayloadTooLarge,
            414 => UriTooLong,
            415 => UnsupportedMediaType,
            416 => RangeNotSatisfiable,
            417 => ExpectationFailed,
            418 => ImATeapot,
            421 => MisdirectedRequest,
            422 => UnprocessableEntity,
            423 => Locked,
            424 => FailedDependency,
            426 => UpgradeRequired,
            428 => PreconditionRequired,
            429 => TooManyRequests,
            431 => RequestHeaderFieldsTooLarge,
            451 => UnavailableForLegalReasons,
            500 => InternalServerError,
            501 => NotImplemented,
            502 => BadGateway,
            503 => ServiceUnavailable,
            504 => GatewayTimeout,
            505 => HttpVersionNotSupported,
            506 => VariantAlsoNegotiates,
            507 => InsufficientStorage,
            508 => LoopDetected,
            510 => NotExtended,
            511 => NetworkAuthenticationRequired,
            _ => Unregistered(n),
        }
    }
}

impl fmt::Display for HttpStatusCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{} {}", self.to_u16(), self.title())
    }
}

#[cfg(feature = "iron")]
impl From<::iron::status::StatusCode> for HttpStatusCode {
    fn from(iron_status: ::iron::status::StatusCode) -> HttpStatusCode {
        iron_status.to_u16().into()
    }
}