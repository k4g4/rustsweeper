use leptos::*;
use leptos_router::*;

use crate::game_logic::{Difficulty, Size};

/// Renders the home page.
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <h1>"Rustsweeper"</h1>
        <Form method="GET" action="game">
            <div class="settings">
                <h4>"Settings"</h4>
                <table>
                    <tr class="setting difficulty">
                        <td>
                            <label for="difficulty">"Difficulty:"</label>
                        </td>
                        <td>
                            <select name="difficulty">
                                <option value=|| Difficulty::Easy.to_string()>"Easy"</option>
                                <option value=|| Difficulty::Medium.to_string()>"Medium"</option>
                                <option value=|| Difficulty::Hard.to_string()>"Hard"</option>
                            </select>
                        </td>
                    </tr>
                    <tr class="setting size">
                        <td>
                            <label for="size">"Board Size:"</label>
                        </td>
                        <td>
                            <select name="size">
                                <option value=|| Size::Small.to_string()>"Small"</option>
                                <option value=|| Size::Medium.to_string()>"Medium"</option>
                                <option value=|| Size::Large.to_string()>"Large"</option>
                            </select>
                        </td>
                    </tr>
                </table>
            </div>
            <div class="buttons">
                <div class="button-item">
                    <input type="submit" value="New Game" />
                </div>
            </div>
        </Form>
    }
}
