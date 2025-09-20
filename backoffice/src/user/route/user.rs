use crate::common::html::context_html::ContextHtmlBuilder;
use crate::common::icon::{key_icon, pencil_square_icon, plus_icon};
use crate::user::model::user_model::UserIdContext;
use crate::user::role::Role;
use crate::user::role::user_role_check::must_be_user;
use crate::user::service::user_manager_service::list_service::ListUserService;
use maud::{Markup, html};
use poem::{Route, get, handler};
use shared::context::Dep;

pub const USER_ROUTE: &str = "/user";

#[handler]
fn list_users(
    Dep(list_user_service): Dep<ListUserService>,
    Dep(context_html_builder): Dep<ContextHtmlBuilder>,
    Dep(user_id_context): Dep<UserIdContext>,
) -> Markup {
    let list_user = list_user_service.list_users();
    let edit_icon = pencil_square_icon();
    let password = key_icon();

    context_html_builder
        .attach_title("List of Users")
        .attach_content(html! {
            h1 { "List of Users" }
            table .table-full {
                thead {
                    tr {
                        th { "ID" }
                        th { "Username" }
                        th { "Role" }
                        @if user_id_context.role == Role::Root {
                            th .action { "Action" }
                        }
                    }
                }
                tbody {
                    @for user in list_user.iter() {
                        tr {
                            td { (user.id) }
                            td { (user.username) }
                            td { (user.role.as_stringed()) }
                            @if user_id_context.role == Role::Root {
                                td .action {
                                    a .icon href=( format!("{}/edit/{}", USER_ROUTE, user.id)) title="Edit User" { (edit_icon) }
                                    " "
                                    a .icon href=( format!("{}/edit-password/{}", USER_ROUTE, user.id)) title="Edit Password" { (password) }
                                }
                            }
                        }
                    }
                }
            }
            div .text-right mt-3 {
                a .inline-block href=( format!("{}/add-user", USER_ROUTE)) title="Add Users" { (plus_icon()) }
            }
        })
        .build()
}

pub fn user_route() -> Route {
    Route::new().at("/", get(must_be_user(list_users)))
}
