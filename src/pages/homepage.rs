use std::{ops::RangeInclusive, rc::Rc};

use gloo_timers::future::TimeoutFuture;
use leptos::*;
use leptos_router::*;
use rand::seq::SliceRandom;

use crate::app_settings::{apply_setting, fetch_setting, Difficulty, Size};

const USERNAME_BOUNDS: RangeInclusive<usize> = 3..=10;

fn valid_chars(username: &str) -> bool {
    username
        .chars()
        .all(|c| c.is_ascii_alphabetic() || c == '_')
}

/// Renders the home page.
#[component]
pub fn HomePage() -> impl IntoView {
    let (username, set_username) = create_signal(fetch_setting("username").unwrap_or_else(|| {
        let names = include!("../../names.json");
        names.choose(&mut rand::thread_rng()).unwrap().to_string()
    }));
    let (difficulty, set_difficulty) =
        create_signal(fetch_setting::<Difficulty>("difficulty").unwrap_or_default());
    let (size, set_size) = create_signal(fetch_setting::<Size>("size").unwrap_or_default());

    let username_ref = create_node_ref::<html::Input>();
    let error_ref = create_node_ref::<html::Span>();
    let difficulty_ref = create_node_ref::<html::Select>();
    let size_ref = create_node_ref::<html::Select>();

    let on_username_input = move |ev| {
        let old_name = username();
        let new_name = event_target_value(&ev);
        if new_name.len() <= *USERNAME_BOUNDS.end() && valid_chars(&new_name) {
            set_username(new_name);
        } else {
            set_username(old_name);

            create_action(move |&()| async move {
                let username_input = username_ref.get().unwrap();

                let name_input = username_input.prop(
                    "style",
                    "
                    border-color: red;
                ",
                );

                let error_span = error_ref.get().unwrap();

                let error_span = error_span.prop(
                    "style",
                    "
                    visibility: visible;
                    opacity: 1;
                    transition: opacity .2s linear;
                ",
                );

                TimeoutFuture::new(500).await;

                name_input.prop("style", "");

                TimeoutFuture::new(2000).await;

                error_span.prop(
                    "style",
                    "
                    visibility: hidden;
                    opacity: 0;
                    transition: visibility 0s .2s, opacity .2s linear;
                ",
                );
            })
            .dispatch(());
        }
    };

    let on_settings_submit = move |ev: ev::SubmitEvent| {
        let username = username_ref.get().unwrap().value();
        if USERNAME_BOUNDS.contains(&username.len()) && valid_chars(&username) {
            apply_setting("username", &username);
            set_username(username);
        } else {
            ev.prevent_default();
            return;
        }

        let difficulty_select = difficulty_ref.get().unwrap();
        if let Ok(selected_difficulty) = difficulty_select.value().parse() {
            if difficulty() != selected_difficulty {
                apply_setting("difficulty", &selected_difficulty);
                set_difficulty(selected_difficulty);
            }
        } else {
            ev.prevent_default();
            return;
        }

        let size_select = size_ref.get().unwrap();
        if let Ok(selected_size) = size_select.value().parse() {
            if size() != selected_size {
                apply_setting("size", &selected_size);
                set_size(selected_size);
            }
        } else {
            ev.prevent_default();
            return;
        }
    };

    view! {
        <Form
            method="GET"
            action="game"
            on:submit=on_settings_submit
            on_form_data=Rc::new(move |form_data| {
                form_data.delete("username"); //don't need this in the query
            })
        >
            <div class="settings">
                <div class="settings-label">"Settings"</div>
                <table class="settings-table">
                    <tr class="setting name">
                        <td class="setting-label">
                            <label for="username">"Name:"</label>
                        </td>
                        <td>
                            <input
                                type="text"
                                name="username"
                                prop:value=username
                                size="12"
                                node_ref=username_ref
                                on:input=on_username_input
                            />
                            <div class="username-error-container">
                                <span class="username-error" node_ref=error_ref>
                                    "Name must be 3-10 alphanumeric characters and underscores"
                                </span>
                            </div>
                        </td>
                    </tr>

                    <tr class="setting difficulty">
                        <td class="setting-label">
                            <label for="difficulty">"Difficulty:"</label>
                        </td>
                        <td>
                            <select name="difficulty" node_ref=difficulty_ref>
                            {
                                [
                                    Difficulty::Easy,
                                    Difficulty::Normal,
                                    Difficulty::Hard,
                                ].iter().map(|curr_difficulty| {
                                    view! {
                                        <option
                                            value=curr_difficulty.to_string()
                                            selected=move || difficulty() == *curr_difficulty
                                        >
                                        {
                                            let mut difficulty_text = curr_difficulty.to_string();
                                            difficulty_text[..1].make_ascii_uppercase();
                                            difficulty_text
                                        }
                                        </option>
                                    }
                                }).collect_view()
                            }
                            </select>
                        </td>
                    </tr>

                    <tr class="setting size">
                        <td class="setting-label">
                            <label for="size">"Board Size:"</label>
                        </td>
                        <td>
                            <select name="size" node_ref=size_ref>
                            {
                                [
                                    Size::Small,
                                    Size::Medium,
                                    Size::Large,
                                ].iter().map(|curr_size| {
                                    view! {
                                        <option
                                            value=curr_size.to_string()
                                            selected=move || size() == *curr_size
                                        >
                                        {
                                            let mut size_text = curr_size.to_string();
                                            size_text[..1].make_ascii_uppercase();
                                            size_text
                                        }
                                        </option>
                                    }
                                }).collect_view()
                            }
                            </select>
                        </td>
                    </tr>
                </table>
            </div>

            <div class="btns">
                <div class="btn">
                    <input type="submit" value="New Game" />
                </div>
            </div>
        </Form>
    }
}
