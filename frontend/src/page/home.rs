#![allow(non_snake_case)]

pub mod bookmarked;
pub mod liked;

use crate::prelude::*;

use dioxus::prelude::*;

pub fn Home(cx: Scope) -> Element {
    let toaster = use_toaster(cx);
    let api_client = ApiClient::global();
    let post_manager = use_post_manager(cx);
    let router = use_router(cx);

    let _fetch_posts = {
        to_owned![api_client, toaster, post_manager];
        use_future(cx, (), |_| async move {
            use uchat_endpoint::post::endpoint::{HomePosts, HomePostsOk};
            toaster
                .write()
                .info("Retrieving posts...", chrono::Duration::seconds(3));
            post_manager.write().clear();
            let response = fetch_json!(<HomePostsOk>, api_client, HomePosts);
            match response {
                Ok(res) => post_manager.write().populate(res.posts.into_iter()),
                Err(e) => toaster.write().error(
                    format!("Failed to retrieve posts: {e}"),
                    chrono::Duration::seconds(3),
                ),
            }
        })
    };

    let Posts = {
        let posts = post_manager.read().all_to_public();
        if posts.is_empty() {
            let TrendingLink = rsx! {
                a {
                    class: "link",
                    onclick: move |_| {
                        router.navigate_to(page::POSTS_TRENDING);
                    },
                    "trending"
                },
            };
            rsx! {
                div {
                    class: "flex flex-col text-center justify-center
                    h-[calc(100vh_-_var(--navbar-height)_-_var(--appbar-height))]",
                    span {
                        "Check out what's ", TrendingLink ", and follow some users to get started."
                    }
                }
            }
        } else {
            rsx! { posts.into_iter() }
        }
    };

    cx.render(rsx! {
        Appbar {
            title: "Home",
            AppbarImgButton {
                click_handler: move |_| router.navigate_to(page::HOME_LIKED),
                img: "/static/icons/icon-like.svg",
                label: "Liked",
                title: "Show liked posts",
            },
            AppbarImgButton {
                click_handler: move |_| router.navigate_to(page::HOME_BOOKMARKED),
                img: "/static/icons/icon-bookmark.svg",
                label: "Saved",
                title: "Show bookmarked posts",
            },
            AppbarImgButton {
                click_handler: move |_| (),
                img: "/static/icons/icon-home.svg",
                label: "Home",
                title: "Go to the home page",
                disabled: true,
                append_class: appbar::BUTTON_SELECTED,
            },

        },

        Posts
    })
}