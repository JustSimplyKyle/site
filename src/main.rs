#![allow(non_snake_case)]

pub mod blog;

use blog::blog::{Blog, BlogList, BlogPost, PostBlog};
// import the prelude to get access to the `rsx!` macro and the `Element` type
use dioxus::prelude::*;

const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));

#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    // All routes under the NavBar layout will be rendered inside of the NavBar Outlet
    // #[nest("/site")]
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
    // launch the web app
    launch(App);
}

pub fn App() -> Element {
    rsx! {
        head {
            link { href: "https://fonts.googleapis.com", rel: "preconnect" }
            link {
                crossorigin: "false",
                rel: "preconnect",
                href: "https://fonts.gstatic.com"
            }
            link {
                rel: "stylesheet",
                href: "https://fonts.googleapis.com/css2?family=Rubik:ital,wght@0,300..900;1,300..900&display=swap"
            }
        }
        link { rel: "preconnect", href: "https://rsms.me" }
        link { rel: "stylesheet", href: "https:/rsms.me/inter/inter.css" }
        div { class: "bg-surface font-rubik min-w-full min-h-screen text-body",
            div { Router::<Route> {} }
            div { class: "min-h-32" }
        }
    }
}

fn Home() -> Element {
    let content = include_str!("../pages/self_introduction.md");
    rsx! {
        div { class: "flex flex-col items-center justify-center text-6xl font-bold",
            "Hi I'm Kyle!"
        }
        div { class: "flex flex-col items-center justify-center mt-8 mx-6",
            MarkdownRenderer { content }
        }
    }
}
#[component]
fn PageNotFound(route: ReadOnlySignal<Vec<String>>) -> Element {
    let output = String::from("/") + &route().join("/");
    rsx! {
        div { class: "flex flex-col items-center",
            p { class: "text-4xl text-bold my-8", "Page not found" }
            div { class: "space-y-1 animate-floatUp",
                p { "We are terribly sorry, but the page you requested doesn't exist." }
                div { class: "text-red-300",
                    p { class: "text-xl", "Log:" }
                    p { "Attempt to navigate to: {output}" }
                }
            }
        }
    }
}

fn NavBar() -> Element {
    let links = [(Route::Home {}, "KYLE"), (Route::BlogList {}, "Blog")];
    rsx! {
        link {
            href: "https://fonts.googleapis.com/icon?family=Material+Icons",
            rel: "stylesheet"
        }
        div {
            nav { class: "sticky top-0 min-w-full z-20 px-8",
                div {
                    class: "grid grid-flow-col h-full w-full text-xl items-center gap-24",
                    div {
                        class: "justify-self-start h-full flex items-center",
                        for link in links {
                            Link {
                                class: "py-8 transition hover:text-hover h-full px-4 font-bold",
                                to: link.0,
                                {link.1}
                            }
                        }
                    }
                }
            }
            div { class: "z-10", Outlet::<Route> {} }
        }
    }
}

#[component]
fn MarkdownRenderer(content: ReadOnlySignal<String>) -> Element {
    let html_content = markdown::to_html(&*content());
    rsx! {
        script { src: "/prism/prism.js" }
        link { rel: "stylesheet", r#type: "text/css", href: "/prism/prism.css" }
        div {
            class: "*:animate-floatUp *:font-rubik markdown-body text-lg space-y-3 text-body text-lg w-full max-w-[70ch] border-none",
            dangerous_inner_html: html_content
        }
    }
}
