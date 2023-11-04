use std::{ops::RangeInclusive, rc::Rc};

use gloo_timers::future::TimeoutFuture;
use leptos::*;
use leptos_router::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlFormElement;

use crate::{
    game_settings::{apply_setting, fetch_setting, Difficulty, Size, Username},
    utils::Title,
};

const USERNAME_BOUNDS: RangeInclusive<usize> = 3..=10;
const DICE_SVG: &str = include_str!("../../svgs/dice.svg");

fn valid_chars(username: &str) -> bool {
    username
        .chars()
        .all(|c| c.is_ascii_alphabetic() || c == '_')
}

/// Renders the home page.
#[component]
pub fn HomePage() -> impl IntoView {
    let (username, set_username) = (
        expect_context::<ReadSignal<Username>>(),
        expect_context::<WriteSignal<Username>>(),
    );
    let (difficulty, set_difficulty) =
        create_signal(fetch_setting::<Difficulty>("difficulty").unwrap_or_default());
    let (size, set_size) = create_signal(fetch_setting::<Size>("size").unwrap_or_default());
    let (form_action, set_form_action) = create_signal("/");

    let username_ref = create_node_ref::<html::Input>();
    let error_ref = create_node_ref::<html::Span>();
    let difficulty_ref = create_node_ref::<html::Select>();
    let size_ref = create_node_ref::<html::Select>();

    let username_error_action = create_action(move |&()| async move {
        let username_input = username_ref.get().expect("noderef assigned");
        let username_input = username_input.prop(
            "style",
            "
            border-color: red;
        ",
        );
        let error_span = error_ref.get().expect("noderef assigned");
        let error_span = error_span.prop(
            "style",
            "
            visibility: visible;
            opacity: 1;
            transition: opacity .2s linear;
        ",
        );
        TimeoutFuture::new(500).await;
        username_input.prop("style", "");
        TimeoutFuture::new(2000).await;
        error_span.prop(
            "style",
            "
            visibility: hidden;
            opacity: 0;
            transition: visibility 0s .2s, opacity .2s linear;
        ",
        );
    });

    let on_username_input = move |ev| {
        let new_name = event_target_value(&ev);
        if new_name.len() <= *USERNAME_BOUNDS.end() && valid_chars(&new_name) {
            set_username(Username::new(new_name));
        } else {
            set_username(username());
            username_error_action.dispatch(());
        }
    };

    let on_settings_submit = move |ev: ev::SubmitEvent| {
        let Username { name, stable } = username();
        if USERNAME_BOUNDS.contains(&name.len()) && valid_chars(&name) {
            if stable {
                apply_setting("username", &name);
            }
        } else {
            ev.prevent_default();
            username_error_action.dispatch(());
            return;
        }

        let difficulty_select = difficulty_ref.get().expect("noderef assigned");
        if let Ok(selected_difficulty) = difficulty_select.value().parse() {
            if difficulty() != selected_difficulty {
                apply_setting("difficulty", &selected_difficulty);
                set_difficulty(selected_difficulty);
            }
        } else {
            ev.prevent_default();
            return;
        }

        let size_select = size_ref.get().expect("noderef assigned");
        if let Ok(selected_size) = size_select.value().parse() {
            if size() != selected_size {
                apply_setting("size", &selected_size);
                set_size(selected_size);
            }
        } else {
            ev.prevent_default();
            return;
        }
        ev.target()
            .unwrap()
            .dyn_into::<HtmlFormElement>()
            .unwrap()
            .set_action(form_action());
    };

    view! {
        <Form
            method="GET"
            action="/"
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
                                prop:value=move || username().name
                                size="12"
                                node_ref=username_ref
                                on:input=on_username_input
                            />
                            <span
                                class="random-name"
                                on:click=move |_| set_username(Username::random())
                                inner_html=DICE_SVG
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
                                        {curr_difficulty.title()}
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
                                        {curr_size.title()}
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
                    <input
                        type="submit"
                        value="New Game"
                        on:click=move |_| set_form_action("/game")
                    />
                </div>
                <div class="btn">
                    <input
                        type="submit"
                        value="Scores"
                        on:click=move |_| set_form_action("/scores")
                    />
                </div>
            </div>
        </Form>
    }
}
