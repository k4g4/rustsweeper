use std::rc::Rc;

use gloo_timers::future::TimeoutFuture;
use leptos::*;
use leptos_router::*;
use rand::seq::SliceRandom;
use wasm_bindgen::JsCast;

use crate::app_settings::Settings;
use crate::game_logic::{Difficulty, Size};

/// Renders the home page.
#[component]
pub fn HomePage() -> impl IntoView {
    let (settings, set_settings) =
        expect_context::<(ReadSignal<Settings>, WriteSignal<Settings>)>();

    let (username, set_username) = create_signal({
        let names = include!("../../names.json");
        names.choose(&mut rand::thread_rng()).unwrap().to_string()
    });

    view! {
        <Form
            method="GET"

            action="game"

            on_form_data=Rc::new(move |form_data| {
                if let Some(difficulty) = form_data.get("difficulty").as_string() {
                    if let Ok(difficulty) = difficulty.parse() {
                        settings.with(|settings| {
                            if settings.difficulty != difficulty {
                                set_settings.update(|settings| {
                                    settings.difficulty = difficulty;
                                });

                                Settings::set("difficulty", &difficulty);
                            }
                        });
                    }
                }
                if let Some(size) = form_data.get("size").as_string() {
                    if let Ok(size) = size.parse() {
                        settings.with(|settings| {
                            if settings.size != size {
                                set_settings.update(|settings| {
                                    settings.size = size;
                                });

                                Settings::set("size", &size);
                            }
                        });
                    }
                }
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
                                // pattern="[a-zA-Z0-9_]{3,10}"
                                size="12"
                                on:input=move |ev| {
                                    let old_name = username();
                                    let new_name = event_target_value(&ev);
                                    if new_name.len() <= 10
                                        && new_name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
                                    {
                                        set_username(new_name);
                                    } else {
                                        set_username(old_name);

                                        // invalid input flashes the text box red
                                        if let Ok(elem) = ev.target().unwrap().dyn_into::<web_sys::HtmlElement>() {
                                            create_action(|elem: &web_sys::HtmlElement| {
                                                let elem = elem.clone();
                                                async move {
                                                    let flash_css = "
                                                        border-color: red;
                                                    ";
                                                    elem.style().set_css_text(flash_css);
                                                    TimeoutFuture::new(200).await;
                                                    elem.style().set_css_text("");
                                                }
                                            }).dispatch(elem);
                                        }
                                    }
                                }
                            />
                        </td>
                    </tr>
                    <tr class="setting difficulty">
                        <td class="setting-label">
                            <label for="difficulty">"Difficulty:"</label>
                        </td>
                        <td>
                            <select name="difficulty">
                            {
                                [
                                    Difficulty::Easy,
                                    Difficulty::Normal,
                                    Difficulty::Hard,
                                ].iter().map(|difficulty| {
                                    view! {
                                        <option
                                            value=difficulty.to_string()
                                            selected=move || {
                                                settings.with(|settings|
                                                    settings.difficulty == *difficulty)
                                            }
                                        >
                                        {
                                            let mut inner = difficulty.to_string();
                                            inner[..1].make_ascii_uppercase();
                                            inner
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
                            <select name="size">
                            {
                                [
                                    Size::Small,
                                    Size::Medium,
                                    Size::Large,
                                ].iter().map(|size| {
                                    view! {
                                        <option
                                            value=size.to_string()
                                            selected=move || {
                                                settings.with(|settings|
                                                    settings.size == *size)
                                            }
                                        >
                                        {
                                            let mut inner = size.to_string();
                                            inner[..1].make_ascii_uppercase();
                                            inner
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
