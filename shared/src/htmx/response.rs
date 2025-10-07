use poem::http::HeaderValue;
use poem::{IntoResponse, Response};

#[derive(Default)]
pub struct HtmxResponse {
    response: Response,
    location_header: Option<String>,
    push_url_header: Option<String>,
    redirect_header: Option<String>,
    refresh_header: bool,
    replace_url_header: Option<String>,
    reswap_header: Option<String>,
    retarget_header: Option<String>,
    reselect_header: Option<String>,
    trigger_header: Option<String>,
    trigger_after_settle_header: Option<String>,
    trigger_after_swap_header: Option<String>,
}

impl HtmxResponse {
    pub fn location(mut self, location: &str) -> Self {
        self.location_header = Some(location.to_string());
        self
    }

    pub fn push_url(mut self, push_url: &str) -> Self {
        self.push_url_header = Some(push_url.to_string());
        self
    }

    pub fn redirect(mut self, redirect: &str) -> Self {
        self.redirect_header = Some(redirect.to_string());
        self
    }

    pub fn refresh(mut self) -> Self {
        self.refresh_header = true;
        self
    }

    pub fn replace_url(mut self, replace_url: &str) -> Self {
        self.replace_url_header = Some(replace_url.to_string());
        self
    }

    pub fn reswap(mut self, reswap: &str) -> Self {
        self.reswap_header = Some(reswap.to_string());
        self
    }

    pub fn retarget(mut self, retarget: &str) -> Self {
        self.retarget_header = Some(retarget.to_string());
        self
    }

    pub fn reselect(mut self, reselect: &str) -> Self {
        self.reselect_header = Some(reselect.to_string());
        self
    }

    pub fn trigger(mut self, trigger: &str) -> Self {
        self.trigger_header = Some(trigger.to_string());
        self
    }

    pub fn trigger_after_settle(mut self, trigger_after_settle: &str) -> Self {
        self.trigger_after_settle_header = Some(trigger_after_settle.to_string());
        self
    }

    pub fn trigger_after_swap(mut self, trigger_after_swap: &str) -> Self {
        self.trigger_after_swap_header = Some(trigger_after_swap.to_string());
        self
    }
}

pub trait HtmxResponseExt {
    fn htmx_response(self) -> HtmxResponse;
}

impl<T: IntoResponse> HtmxResponseExt for T {
    fn htmx_response(self) -> HtmxResponse {
        let mut htmx_response = HtmxResponse::default();
        htmx_response.response = self.into_response();
        htmx_response
    }
}

impl IntoResponse for HtmxResponse {
    fn into_response(self) -> Response {
        let mut res = self.response;
        let header = res.headers_mut();
        if let Some(location) = self.location_header {
            header.insert(
                "HX-Location",
                HeaderValue::from_str(&location).expect("Invalid Header Value"),
            );
        }
        if let Some(push_url) = self.push_url_header {
            header.insert(
                "HX-Push-Url",
                HeaderValue::from_str(&push_url).expect("Invalid Header Value"),
            );
        }
        if let Some(redirect) = self.redirect_header {
            header.insert(
                "HX-Redirect",
                HeaderValue::from_str(&redirect).expect("Invalid Header Value"),
            );
        }
        if self.refresh_header {
            header.insert("HX-Refresh", HeaderValue::from_static("true"));
        }
        if let Some(replace_url) = self.replace_url_header {
            header.insert(
                "HX-Replace-Url",
                HeaderValue::from_str(&replace_url).expect("Invalid Header Value"),
            );
        }
        if let Some(reswap) = self.reswap_header {
            header.insert(
                "HX-Reswap",
                HeaderValue::from_str(&reswap).expect("Invalid Header Value"),
            );
        }
        if let Some(retarget) = self.retarget_header {
            header.insert(
                "HX-Retarget",
                HeaderValue::from_str(&retarget).expect("Invalid Header Value"),
            );
        }
        if let Some(reselect) = self.reselect_header {
            header.insert(
                "HX-Reselect",
                HeaderValue::from_str(&reselect).expect("Invalid Header Value"),
            );
        }
        if let Some(trigger) = self.trigger_header {
            header.insert(
                "HX-Trigger",
                HeaderValue::from_str(&trigger).expect("Invalid Header Value"),
            );
        }
        if let Some(trigger_after_settle) = self.trigger_after_settle_header {
            header.insert(
                "HX-Trigger-After-Settle",
                HeaderValue::from_str(&trigger_after_settle).expect("Invalid Header Value"),
            );
        }
        if let Some(trigger_after_swap) = self.trigger_after_swap_header {
            header.insert(
                "HX-Trigger-After-Swap",
                HeaderValue::from_str(&trigger_after_swap).expect("Invalid Header Value"),
            );
        }

        res
    }
}
