#![allow(non_snake_case)]

use std::{convert::Infallible, fmt::Display, str::FromStr};

// import the prelude to get access to the `rsx!` macro and the `Element` type
use dioxus::prelude::*;
use dioxus_router::prelude::*;

use log::LevelFilter;

const _INPUT_CSS: &str = manganis::mg!(file("input.css"));

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
                #[route("/:blog")]
                PostBlog { blog: BlogPost },
            #[end_layout]
        #[end_nest]
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    // launch the web app
    launch(App);
}

pub fn App() -> Element {
    rsx! {
        link {
            rel: "preconnect",
            href: "https://rsms.me"
        }
        link {
            rel: "stylesheet",
            href: "https:/rsms.me/inter/inter.css"
        }
        Router::<Route> {}
    }
}

fn Home() -> Element {
    let content = include_str!("../pages/self_introduction.md");
    rsx! {
        div { class: "flex flex-col items-center justify-center text-6xl font-bold",
            "Self Introduction"
        }
        div {
            class: "flex flex-col items-center justify-center mt-8",
            MarkdownRenderer {content}
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

fn NavBar() -> Element {
    let links = [(Route::Home {}, "Home"), (Route::BlogList {}, "Blog")];
    rsx! {
        link {
            href: "https://fonts.googleapis.com/icon?family=Material+Icons",
            rel: "stylesheet"
        }
        div { class: "bg-surface min-w-full min-h-screen text-body",
            nav {
                div { class: "flex-row space-x-5 p-6 pl-12 mb-8 bg-gray-800 backdrop-blur-xl opacity-80",
                    for link in links {
                        Link {
                            class: "bg-gray-700 transition hover:text-hover rounded-2xl drop-shadow-lg backdrop-blur-xl p-4",
                            to: link.0,
                            {link.1}
                        }
                    }
                }
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
    tags: &'static [&'static str],
}

impl Display for BlogPost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.link)
    }
}

impl FromStr for BlogPost {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x = POSTS.iter().find(|x| x.0 == s).map(|x| x.1).unwrap();
        Ok(x)
    }
}

impl BlogPost {
    const fn link_post(self) -> (&'static str, Self) {
        (self.link, self)
    }
}

pub const POSTS: &[(&str, BlogPost)] = &[
    BlogPost {
        date: "March 10th, 2024",
        title: "Top Reasons of why Femboy is the Best",
        description: "The femboy thesis we've all wished for",
        link: "going-femboy",
        content: include_str!("../pages/femboy.md"),
        tags: &["humerous", "rant", "very funny"],
    }
    .link_post(),
    BlogPost {
        date: "March 11th, 2024",
        title: "My journey with bevy",
        description: "Building a simple game using bevy 0.12",
        link: "building-tetris-in-bevy",
        content: include_str!("../pages/bevy.md"),
        tags: &["fun", "programming", "sadness", "anguish"],
    }
    .link_post(),
];

#[component]
fn BlogList() -> Element {
    rsx! {
        div { class: "flex flex-col space-y-7 text-xl mt-6",
            for post in POSTS.iter() {
                Link { to: Route::PostBlog { blog: post.1 },
                    div {
                        div { class: "transition hover:text-hover hover:shadow-lg",
                            { post.1.title }
                        }
                        div { class: "text-sm text-slate-400", { post.1.date } }
                    }
                }
            }
        }
    }
}

fn Blog() -> Element {
    rsx! {
        div { class: "flex flex-col items-center justify-center font-sans",
            p { class: "text-6xl font-bold", "Blog" }
            Outlet::<Route> {}
        }
    }
}

#[component]
fn PostBlog(blog: BlogPost) -> Element {
    let content = blog.content;
    rsx! {
        div { class: "flex flex-col items-center justify-center font-sans p-8",
            div { class: "flex flex-col items-center space-y-2 mb-6",
                p { class: "basis-4/6 text-4xl font-bold text-title", {blog.title} }
                p { class: "basis-1/6 text-body",
                    div { class: "inline-flex",
                        span { class: "material-icons", "calendar_today" }
                        div { class: "ml-1 text-slate-400", "{blog.date}" }
                        div { class: "ml-5 flex space-x-1",
                            for tag in blog.tags {
                                div { class: "bg-[#1598D3] rounded-full px-2 text-title",
                                    {tag}
                                }
                            }
                        }
                    }
                }
            }
            div {
                MarkdownRenderer {content}
            }
        }
    }
}

#[component]
fn MarkdownRenderer(content: String) -> Element {
    let html_content = markdown::to_html(&*content);
    rsx! {
        script {
            src: "/prism/prism.js"
        }
        link {
            rel: "stylesheet",
            r#type: "text/css",
            href: "/prism/prism.css"
        }
        div {
            class: "markdown-body basis-1/6 text-lg space-y-3 text-body text-lg max-w-prose border-none",
            dangerous_inner_html: html_content,
        }
    }
}
