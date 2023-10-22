use std::rc::Rc;

use leptos::*;
use leptos_router::*;

use crate::app_settings::Settings;
use crate::game_logic::{Difficulty, Size};

/// Renders the home page.
#[component]
pub fn HomePage() -> impl IntoView {
    let (settings, set_settings) =
        expect_context::<(ReadSignal<Settings>, WriteSignal<Settings>)>();

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
                    <tr class="setting difficulty">
                        <td>
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
                        <td>
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
            <div>
                <div class="btn">
                    <input type="submit" value="New Game" />
                </div>
            </div>
        </Form>
    }
}
