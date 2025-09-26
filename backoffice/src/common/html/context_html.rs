use crate::common::html::HtmlBuilder;
use crate::common::html::locale::top::TopBuildLocale;
use crate::user::pointer::user_pointer::UserPointer;
use crate::user::role::Role;
use crate::user::route::login::LOGIN_ROUTE;
use crate::user::route::user::USER_ROUTE;
use error_stack::Report;
use maud::{Markup, PreEscaped, html};
use poem::i18n::Locale;
use shared::context::{Context, ContextError, FromContext};
use shared::flash::{Flash, FlashMessageHtml};
use shared::locale::LocaleExt;
use std::sync::RwLock;

pub struct NavigationItem {
    name: String,
    url: String,
    tag: String,
    locale: String,
    role: Role,
}

impl NavigationItem {
    fn navigations() -> Box<[Self]> {
        [
            Self {
                name: "Home".to_string(),
                url: "/".to_string(),
                tag: "home".to_string(),
                locale: "top-navigation-home".to_string(),
                role: Role::Visitor,
            },
            Self {
                name: "URL Redirect".to_string(),
                url: "/shorty".to_string(),
                tag: "shorty".to_string(),
                locale: "top-navigation-url".to_string(),
                role: Role::User,
            },
            Self {
                name: "User".to_string(),
                url: "/user".to_string(),
                tag: "user".to_string(),
                locale: "top-navigation-user".to_string(),
                role: Role::User,
            },
            Self {
                name: "Stack".to_string(),
                url: "/stack".to_string(),
                tag: "stack".to_string(),
                locale: "top-navigation-stack".to_string(),
                role: Role::Root,
            },
        ]
        .into()
    }
}

struct ContextHtmlCellData {
    title: Option<String>,
    content: Option<Markup>,
    head: Option<Markup>,
    footer: Option<Markup>,
    current_tag: String,
}

pub struct ContextHtmlBuilder {
    flash: Option<Flash>,
    user_id_context: UserPointer,
    data: RwLock<ContextHtmlCellData>,
    pub locale: Locale,
}

impl ContextHtmlBuilder {
    pub fn new(flash: Option<Flash>, locale: Locale, user_id_context: UserPointer) -> Self {
        Self {
            flash,
            user_id_context,
            data: RwLock::new(ContextHtmlCellData {
                title: None,
                content: None,
                head: None,
                footer: None,
                current_tag: "".to_string(),
            }),
            locale,
        }
    }

    pub fn attach_title(&self, title: &str) -> &Self {
        match self.data.try_write() {
            Ok(mut data) => {
                data.title = Some(title.to_string());
            }
            Err(_) => {}
        }
        self
    }

    pub fn attach_content(&self, content: Markup) -> &Self {
        match self.data.try_write() {
            Ok(mut data) => {
                data.content = Some(content);
            }
            Err(_) => {}
        }
        self
    }

    #[allow(dead_code)]
    pub fn attach_head(&self, head: Markup) -> &Self {
        match self.data.try_write() {
            Ok(mut data) => {
                data.head = Some(head);
            }
            Err(_) => {}
        }
        self
    }

    #[allow(dead_code)]
    pub fn attach_footer(&self, footer: Markup) -> &Self {
        match self.data.try_write() {
            Ok(mut data) => {
                data.footer = Some(footer);
            }
            Err(_) => {}
        }
        self
    }

    pub fn set_current_tag(&self, tag: &str) -> &Self {
        match self.data.try_write() {
            Ok(mut data) => {
                data.current_tag = tag.to_string();
            }
            Err(_) => {}
        }
        self
    }

    pub fn build(&self) -> Markup {
        match self.data.try_read() {
            Ok(data) => {
                let title = data.title.clone().unwrap_or_else(|| "Untitled".to_string());
                let content = data.content.clone().unwrap_or_else(|| html! {});
                let head = data.head.clone().unwrap_or_else(|| html! {});
                let footer = data.footer.clone().unwrap_or_else(|| html! {});
                let current_tag = data.current_tag.clone();

                let new_content = html! {
                    (self.flash.flash_message_html())
                    (self.build_navigation(current_tag))
                    div .content-wrapper {
                        div .container .main-content {
                            (content)
                        }
                    }
                };

                HtmlBuilder::new(title, new_content)
                    .attach_head(head)
                    .attach_footer(footer)
                    .build()
            }
            Err(_) => {
                html! {}
            }
        }
    }

    fn build_navigation(&self, tag: String) -> Markup {
        let user_context = &self.user_id_context;
        let top_build_locale = TopBuildLocale::new(&self.locale, &user_context.username);
        html! {
            nav .nav-content {
                span .nav-home {
                    a href="/" { "Rusty Shorty" }
                }
                (self.parse_navigation(tag))
                span .nav-user {
                    @if user_context.role >= Role::User {
                        a href=(USER_ROUTE.to_owned() + "/") { (top_build_locale.hello) }
                        " "
                        a href=(LOGIN_ROUTE.to_owned() + "/logout") { (top_build_locale.hello_logout) }
                    } @else {
                        a href=(LOGIN_ROUTE.to_owned() + "/") { (top_build_locale.visitor) }
                    }
                }
            }
        }
    }

    fn parse_navigation(&self, tag: String) -> Markup {
        let mut output = "".to_string();
        for item in NavigationItem::navigations() {
            if self.user_id_context.role < item.role {
                continue;
            }
            let html = if item.tag == tag {
                html! {
                    span .nav-item .nav-item-active {
                        a href=(item.url) {
                            (self.locale.text_with_default(item.locale.as_str(), &item.name))
                        }
                    }
                }
            } else {
                html! {
                    span .nav-item {
                        a href=(item.url) {
                            (self.locale.text_with_default(item.locale.as_str(), &item.name))
                        }
                    }
                }
            };
            output.push_str(html.into_string().as_str());
        }
        PreEscaped(output)
    }
}

impl FromContext for ContextHtmlBuilder {
    async fn from_context(ctx: &'_ Context<'_>) -> Result<Self, Report<ContextError>> {
        Ok(Self::new(
            ctx.inject().await?,
            ctx.inject().await?,
            ctx.inject().await?,
        ))
    }
}
