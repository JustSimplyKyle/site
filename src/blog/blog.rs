use dioxus::prelude::*;
use std::{fmt::Display, str::FromStr};

use crate::{MarkdownRenderer, Route};

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
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        POSTS
            .iter()
            .find(|x| x.0 == s)
            .map(|x| x.1)
            .ok_or("Can't find blog post".into())
    }
}

impl BlogPost {
    const fn link_post(self) -> (&'static str, Self) {
        (self.link, self)
    }
}

pub const POSTS: &[(&str, BlogPost)] = &[
    BlogPost {
        date: "March 11th, 2024",
        title: "My journey with bevy",
        description: "Building a simple game using bevy 0.12",
        link: "building-tetris-in-bevy",
        content: include_str!("../../pages/bevy.md"),
        tags: &["fun", "programming", "sadness"],
    }
    .link_post(),
    BlogPost {
        date: "March 10th, 2024",
        title: "Top Reasons of why Femboy is the Best",
        description: "The femboy thesis we've all wished for",
        link: "going-femboy",
        content: include_str!("../../pages/femboy.md"),
        tags: &["humerous", "rant", "very funny"],
    }
    .link_post(),
];

#[component]
pub fn BlogList() -> Element {
    rsx! {
        div {
            div {
                class: "flex flex-col space-y-7 text-xl mt-6",
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
}

pub fn Blog() -> Element {
    rsx! {
        div { class: "flex flex-col items-center",
            p { class: "text-6xl font-bold", "Blog" }
            div {
                class: "animate-floatUp",
                Outlet::<Route> {}
            }
        }
    }
}

#[component]
pub fn PostBlog(blog: BlogPost) -> Element {
    let content = blog.content;
    rsx! {
        div { class: "flex flex-col items-center justify-center font-sans p-8 w-fit",
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
            div { MarkdownRenderer { content } }
        }
    }
}
