#![allow(non_snake_case)]

// import the prelude to get access to the `rsx!` macro and the `Element` type
use dioxus::prelude::*;
use dioxus_router::prelude::*;

use log::LevelFilter;

#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    // All routes under the NavBar layout will be rendered inside of the NavBar Outlet
    #[layout(NavBar)]
        #[route("/")]
        Home {},
        #[nest("/blog")]
            #[layout(Blog)]
            #[route("/")]
            BlogList {},
            #[route("/:link")]
            PostBlog { blog: BlogPost, link: String },
            #[end_layout]
        #[end_nest]
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },}

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    // launch the web app
    launch(App);
}

pub fn App() -> Element {
    rsx! { Router::<Route> {} }
}

#[component]
fn Home() -> Element {
    rsx! {
        h1 {
            "Welcome to my Personal Page!",
        }
    }
}
#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}

#[component]
fn NavBar() -> Element {
    rsx! {
        link {
            href: "https://fonts.googleapis.com/icon?family=Material+Icons",
            rel: "stylesheet"
        }
        div {
            class: "bg-surface min-w-full min-h-screen text-body",
            nav {
                div {
                    div {
                        Link { to: Route::Home {}, "Home"}
                    },
                    div {
                        Link { to: Route::BlogList {}, "Blog"}
                    }
                 }
            },
            div {
            }
            Outlet::<Route> {}
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Default)]
pub struct BlogPost {
    date: &'static str,
    title: &'static str,
    description: &'static str,
    link: &'static str,
    content: &'static str,
}

pub const MY_FIRST_POST: BlogPost = BlogPost {
    date: "March 10th, 2024",
    title: "Top Reasons of why Femboy is the best",
    description: "The femboy thesis we've all wished for",
    link: "going-femboy",
    content: include_str!("../pages/page.md"),
};

#[component]
fn BlogList() -> Element {
    rsx! {
        ul {
            li {
                Link {
                    to: Route::PostBlog { blog: MY_FIRST_POST, link: MY_FIRST_POST.link.to_string() },
                    h3 {
                        {MY_FIRST_POST.title}
                    }
                }
            }
        }
    }
}

fn Blog() -> Element {
    rsx! {
        h1 { "Blog" }
        Outlet::<Route> {}
    }
}

#[component]
fn PostBlog(blog: BlogPost, link: String) -> Element {
    let html_content = markdown::to_html(blog.content);
    rsx! {
        div {
            class: "flex items-center justify-center font-sans p-8",
            div {
                class: "m-4",
                div {
                    class: "text-title space-y-1 mb-4",
                    p {
                        class: "basis-4/6 text-4xl font-bold flex items-center justify-center",
                        {blog.title}
                    }
                    p {
                        class: "basis-1/6 text-body",
                        div {
                            class: "flex items-center justify-center",
                            span {
                                class: "material-icons",
                                "calendar_today"
                            },
                            div {
                                class: "ml-1",
                                "{blog.date}"
                            }
                        }
                    }
                }
                div {
                    class: "basis-1/6 text-lg space-y-3 text-body max-w-screen-md",
                    dangerous_inner_html: html_content
                }
            }
        }
    }
}
