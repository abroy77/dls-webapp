pub mod dls;
use dls::MAX_OVERS;
use leptos::{mount::mount_to_body, prelude::*};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! { <DLSCalculator /> }
}

// #[component]
// fn InterruptionComponent() -> impl IntoView {
//     let (int_getter, int_setter) = signal(Interruption::new());
// }

#[component]
pub fn DLSCalculator() -> impl IntoView {
    // Only ask for the four required inputs
    let (team_1_score, set_team_1_score) = signal(String::new());
    let (overs_played, set_overs_played) = signal(String::new());
    let (wickets_lost, set_wickets_lost) = signal(String::new());
    let (new_total_overs, set_new_total_overs) = signal(String::new());

    let parse_usize = |label: &'static str, s: &str| {
        s.trim()
            .parse::<usize>()
            .map_err(|e| format!("{label} '{s}' is not an integer: {e}"))
    };

    let parsed = Memo::new(move |_| {
        // Check if all fields are non-empty first
        let t1_str = team_1_score.get();
        let op_str = overs_played.get();
        let wl_str = wickets_lost.get();
        let nto_str = new_total_overs.get();

        let t1 = t1_str.trim();
        let op = op_str.trim();
        let wl = wl_str.trim();
        let nto = nto_str.trim();

        if t1.is_empty() || op.is_empty() || wl.is_empty() || nto.is_empty() {
            return Err("Please fill in all fields".to_string());
        }

        let team_1_score = parse_usize("Team 1 Score", t1)?;
        let overs_played = parse_usize("Overs played before interruption", op)?;
        let wickets_lost = parse_usize("Wickets Lost", wl)?;
        let new_total_overs = parse_usize("New total overs after interruption", nto)?;

        if wickets_lost >= 10 {
            return Err("Wickets Lost must be between 0 and 9".to_string());
        }
        if new_total_overs > MAX_OVERS {
            return Err("New total overs after interruption must be <= 50".to_string());
        }
        if overs_played > new_total_overs {
            return Err("Overs played cannot exceed new total overs".to_string());
        }

        Ok((team_1_score, overs_played, wickets_lost, new_total_overs))
    });

    let updated_target = Memo::new(move |_| match parsed.get() {
        Ok((team_1_score, overs_played, wickets_lost, new_total_overs)) => Ok(
            dls::get_target_score_simple(team_1_score, overs_played, wickets_lost, new_total_overs),
        ),
        Err(e) => Err(e),
    });

    view! {
        <div style="max-width: 400px; margin: 2rem auto; padding: 2rem; background: #f8f9fa; border-radius: 1rem; box-shadow: 0 2px 12px #0001;">
            <h2 style="text-align: center; margin-bottom: 1.5rem; color: #2c3e50;">DLS Target Calculator</h2>
            <div style="display: flex; flex-direction: column; gap: 1.2rem;">
                <div>
                    <label for="t1" style="font-weight: 500;">Team 1 Score</label><br />
                    <input
                        id="t1"
                        type="number"
                        min="0"
                        step="1"
                        bind:value=(team_1_score, set_team_1_score)
                        name="team_1_score"
                        style="width: 100%; padding: 0.5rem; border-radius: 0.5rem; border: 1px solid #ccc;"
                    />
                </div>
                <div>
                    <label for="overs_played" style="font-weight: 500;">Overs Played (before interruption)</label><br />
                    <input
                        id="overs_played"
                        type="number"
                        min="0"
                        max="50"
                        step="1"
                        bind:value=(overs_played, set_overs_played)
                        name="overs_played"
                        style="width: 100%; padding: 0.5rem; border-radius: 0.5rem; border: 1px solid #ccc;"
                    />
                </div>
                <div>
                    <label for="wickets_lost" style="font-weight: 500;">Wickets Lost (at interruption)</label><br />
                    <input
                        id="wickets_lost"
                        type="number"
                        min="0"
                        max="9"
                        step="1"
                        bind:value=(wickets_lost, set_wickets_lost)
                        name="wickets_lost"
                        style="width: 100%; padding: 0.5rem; border-radius: 0.5rem; border: 1px solid #ccc;"
                    />
                </div>
                <div>
                    <label for="new_total_overs" style="font-weight: 500;">New Total Overs (after interruption)</label><br />
                    <input
                        id="new_total_overs"
                        type="number"
                        min="0"
                        max="50"
                        step="1"
                        bind:value=(new_total_overs, set_new_total_overs)
                        name="new_total_overs"
                        style="width: 100%; padding: 0.5rem; border-radius: 0.5rem; border: 1px solid #ccc;"
                    />
                </div>
            </div>
            <div style="margin-top: 2rem;">
                {move || {
                    match updated_target.get() {
                        Ok(Ok(target)) => view! {
                            <p style="color: #27ae60; background: #eafaf1; padding: 1rem; border-radius: 0.5rem; text-align: center; font-size: 1.2rem;">
                                "Updated Target: "<strong>{target}</strong>
                            </p>
                        }.into_any(),
                        Ok(Err(e)) => view! {
                            <p style="color: #c0392b; background: #fff3f3; padding: 0.75rem; border-radius: 0.5rem; text-align: center;">
                                {e}
                            </p>
                        }.into_any(),
                        Err(e) if e != "Please fill in all fields" => view! {
                            <p style="color: #c0392b; background: #fff3f3; padding: 0.75rem; border-radius: 0.5rem; text-align: center;">
                                {e}
                            </p>
                        }.into_any(),
                        _ => view! { <div></div> }.into_any(),
                    }
                }}
            </div>
        </div>
    }
}
