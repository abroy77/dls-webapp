fn get_resource(overs_left: usize, wickets_lost: usize) -> Result<f32, &'static str> {
    let overs_played = MAX_OVERS
        .checked_sub(overs_left)
        .ok_or("Overs left greater than 50 not allowed")?;
    if overs_played < RESOURCE_TABLE.len() && wickets_lost < RESOURCE_TABLE[0].len() {
        Ok(RESOURCE_TABLE[overs_played][wickets_lost])
    } else {
        Err("Out of bounds wickets taken or overs lost")
    }
}

pub fn get_target_score_simple(
    team_1_score: usize,
    overs_played: usize,
    wickets_lost: usize,
    new_total_overs: usize,
) -> Result<usize, &'static str> {
    // Log input parameters
    web_sys::console::log_4(
        &"DLS Input:".into(),
        &format!("team_1_score={}", team_1_score).into(),
        &format!("overs_played={}", overs_played).into(),
        &format!("wickets_lost={}", wickets_lost).into(),
    );
    web_sys::console::log_1(&format!("new_total_overs={}", new_total_overs).into());

    // Team 1 resource (full 50 overs, 0 wickets lost)
    let r1 = get_resource(MAX_OVERS, 0)?;
    web_sys::console::log_1(&format!("r1 (Team 1 resource)={}", r1).into());

    // Validation
    if overs_played > new_total_overs {
        return Err("Overs played cannot exceed new total overs");
    }
    if wickets_lost > 9 {
        return Err("Wickets lost cannot exceed 9");
    }
    if new_total_overs > MAX_OVERS {
        return Err("New total overs cannot exceed 50");
    }

    // Team 2's resource calculation:
    // What they have remaining after the interruption
    let overs_remaining = new_total_overs - overs_played;
    web_sys::console::log_1(&format!("overs_remaining={}", overs_remaining).into());

    let resource_at_suspension = get_resource(MAX_OVERS - overs_played, wickets_lost)?;
    web_sys::console::log_1(
        &format!(
            "r2 (Team 2 resource as suspensioln)={}",
            resource_at_suspension
        )
        .into(),
    );
    let resource_after_suspension = get_resource(overs_remaining, wickets_lost)?;
    web_sys::console::log_1(
        &format!(
            "r2 (Team 2 resource after suspension)={}",
            resource_after_suspension
        )
        .into(),
    );
    let resource_lost = resource_at_suspension - resource_after_suspension;
    web_sys::console::log_1(&format!("r2 (Team 2 resource lost)={}", resource_lost).into());
    let r2 = r1 - resource_lost;

    web_sys::console::log_1(&format!("r2 (Team 2 resource)={}", r2).into());

    // DLS target calculation with overflow protection
    let target_f32 = team_1_score as f32 * r2 / r1;
    web_sys::console::log_1(
        &format!(
            "target_f32 = {} * {} / {} = {}",
            team_1_score, r2, r1, target_f32
        )
        .into(),
    );

    if target_f32.is_nan() || target_f32.is_infinite() || target_f32 < 0.0 {
        return Err("Invalid calculation result");
    }

    let target_floor = target_f32.floor();
    web_sys::console::log_1(&format!("target_floor={}", target_floor).into());

    let target = target_floor as usize + 1;
    web_sys::console::log_1(&format!("final target={}", target).into());

    Ok(target)
}

#[allow(dead_code)]
fn get_target_score(
    team_1_score: usize,
    team_1_innings: TeamInnings,
    team_2_innings: TeamInnings,
) -> Result<usize, &'static str> {
    let r1 = team_1_innings.resources()?;
    let r2 = team_2_innings.resources()?;

    if r1 == r2 {
        Ok(team_1_score + 1)
    } else if r1 > r2 {
        Ok((team_1_score as f32 * r2 / r1).floor() as usize + 1)
    } else {
        Ok(team_1_score + 1 + ((r2 - r1) * (G50 as f32 / 100.0)).floor() as usize)
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Interruption {
    overs_left_pre_suspension: usize,
    wickets_lost: usize,
    overs_left_post_suspension: usize,
}

#[allow(dead_code)]
struct TeamInnings {
    interruptions: Vec<Interruption>,
}

#[allow(dead_code)]
impl TeamInnings {
    pub fn new() -> Self {
        TeamInnings {
            interruptions: vec![],
        }
    }

    pub fn add_interruption(&mut self, interruption: Interruption) -> Result<(), &'static str> {
        // check that the interruption is valid given the ones before it
        if let Some(last_interruption) = self.interruptions.last()
            && last_interruption.overs_left_post_suspension
                <= interruption.overs_left_pre_suspension
        {
            Err(
                "New interruption must have pre_suspension overs less than that of the latest one in the innings",
            )
        } else {
            self.interruptions.push(interruption);
            Ok(())
        }
    }
    pub fn resources(&self) -> Result<f32, &'static str> {
        let net_resource_lost: Result<f32, &'static str> = self
            .interruptions
            .iter()
            .try_fold(0.0, |acc, item| Ok(acc + item.resource_lost()?));

        Ok(100.0 - net_resource_lost?)
    }
}

impl Interruption {
    pub fn new(
        overs_played: usize,
        wickets_lost: usize,
        reduced_total_overs: usize,
    ) -> Result<Self, &'static str> {
        // some guard clauses to make sure this is done right
        if reduced_total_overs >= MAX_OVERS {
            return Err("reduced total overs cannot be more than 50");
        }
        let overs_left_pre_suspension = MAX_OVERS
            .checked_sub(overs_played)
            .ok_or(" Overs Played cannot be greater than max overs")?;
        let overs_left_post_suspension = reduced_total_overs
            .checked_sub(overs_played)
            .ok_or("Reduced total overs cannot be less than overs played")?;

        if overs_left_pre_suspension <= overs_left_post_suspension {
            Err(
                "Can't have overs left after suspension be more than or equal those pre suspension.",
            )
        } else if wickets_lost > 9 {
            Err("Can't lose more than 9 wickets")
        } else {
            Ok(Self {
                overs_left_pre_suspension,
                wickets_lost,
                overs_left_post_suspension,
            })
        }
    }
    pub fn resource_lost(&self) -> Result<f32, &'static str> {
        let resource_pre_suspension =
            get_resource(self.overs_left_pre_suspension, self.wickets_lost)?;
        let resource_post_suspension =
            get_resource(self.overs_left_post_suspension, self.wickets_lost)?;
        Ok(resource_pre_suspension - resource_post_suspension)
    }
}

pub const MAX_OVERS: usize = 50;

// This is for full ICC members. may be different for  women's and under 15 teams
#[allow(dead_code)]
const G50: usize = 245;

const RESOURCE_TABLE: [[f32; 10]; 51] = [
    [100.0, 93.4, 85.1, 74.9, 62.7, 49.0, 34.9, 22.0, 11.9, 4.7],
    [99.1, 92.6, 84.5, 74.4, 62.5, 48.9, 34.9, 22.0, 11.9, 4.7],
    [98.1, 91.7, 83.8, 74.0, 62.2, 48.8, 34.9, 22.0, 11.9, 4.7],
    [97.1, 90.9, 83.2, 73.5, 61.9, 48.6, 34.9, 22.0, 11.9, 4.7],
    [96.1, 90.0, 82.5, 73.0, 61.6, 48.5, 34.8, 22.0, 11.9, 4.7],
    [95.0, 89.1, 81.8, 72.5, 61.3, 48.4, 34.8, 22.0, 11.9, 4.7],
    [93.9, 88.2, 81.0, 72.0, 61.0, 48.3, 34.8, 22.0, 11.9, 4.7],
    [92.8, 87.3, 80.3, 71.4, 60.7, 48.1, 34.7, 22.0, 11.9, 4.7],
    [91.7, 86.3, 79.5, 70.9, 60.3, 47.9, 34.7, 22.0, 11.9, 4.7],
    [90.5, 85.3, 78.7, 70.3, 59.9, 47.8, 34.6, 22.0, 11.9, 4.7],
    [89.3, 84.2, 77.8, 69.6, 59.5, 47.6, 34.6, 22.0, 11.9, 4.7],
    [88.0, 83.1, 76.9, 69.0, 59.1, 47.4, 34.5, 22.0, 11.9, 4.7],
    [86.7, 82.0, 76.0, 68.3, 58.7, 47.1, 34.5, 21.9, 11.9, 4.7],
    [85.4, 80.9, 75.0, 67.6, 58.2, 46.9, 34.4, 21.9, 11.9, 4.7],
    [84.1, 79.7, 74.1, 66.8, 57.7, 46.6, 34.3, 21.9, 11.9, 4.7],
    [82.7, 78.5, 73.0, 66.0, 57.2, 46.4, 34.2, 21.9, 11.9, 4.7],
    [81.3, 77.2, 72.0, 65.2, 56.6, 46.1, 34.1, 21.9, 11.9, 4.7],
    [79.8, 75.9, 70.9, 64.4, 56.0, 45.8, 34.0, 21.9, 11.9, 4.7],
    [78.3, 74.6, 69.7, 63.5, 55.4, 45.4, 33.9, 21.9, 11.9, 4.7],
    [76.7, 73.2, 68.6, 62.5, 54.8, 45.1, 33.7, 21.9, 11.9, 4.7],
    [75.1, 71.8, 67.3, 61.6, 54.1, 44.7, 33.6, 21.8, 11.9, 4.7],
    [73.5, 70.3, 66.1, 60.5, 53.4, 44.2, 33.4, 21.8, 11.9, 4.7],
    [71.8, 68.8, 64.8, 59.5, 52.6, 43.8, 33.2, 21.8, 11.9, 4.7],
    [70.1, 67.2, 63.4, 58.4, 51.8, 43.3, 33.0, 21.7, 11.9, 4.7],
    [68.3, 65.6, 62.0, 57.2, 50.9, 42.8, 32.8, 21.7, 11.9, 4.7],
    [66.5, 63.9, 60.5, 56.0, 50.0, 42.2, 32.6, 21.6, 11.9, 4.7],
    [64.6, 62.2, 59.0, 54.7, 49.0, 41.6, 32.3, 21.6, 11.9, 4.7],
    [62.7, 60.4, 57.4, 53.4, 48.0, 40.9, 32.0, 21.5, 11.9, 4.7],
    [60.7, 58.6, 55.8, 52.0, 47.0, 40.2, 31.6, 21.4, 11.9, 4.7],
    [58.7, 56.7, 54.1, 50.6, 45.8, 39.4, 31.2, 21.3, 11.9, 4.7],
    [56.6, 54.8, 52.4, 49.1, 44.6, 38.6, 30.8, 21.2, 11.9, 4.7],
    [54.4, 52.8, 50.5, 47.5, 43.4, 37.7, 30.3, 21.1, 11.9, 4.7],
    [52.2, 50.7, 48.6, 45.9, 42.0, 36.8, 29.8, 20.9, 11.9, 4.7],
    [49.9, 48.5, 46.7, 44.1, 40.6, 35.8, 29.2, 20.7, 11.9, 4.7],
    [47.6, 46.3, 44.7, 42.3, 39.1, 34.7, 28.5, 20.5, 11.8, 4.7],
    [45.2, 44.1, 42.6, 40.5, 37.6, 33.5, 27.8, 20.2, 11.8, 4.7],
    [42.7, 41.7, 40.4, 38.5, 35.9, 32.2, 27.0, 19.9, 11.8, 4.7],
    [40.2, 39.3, 38.1, 36.5, 34.2, 30.8, 26.1, 19.5, 11.7, 4.7],
    [37.6, 36.8, 35.8, 34.3, 32.3, 29.4, 25.1, 19.0, 11.6, 4.7],
    [34.9, 34.2, 33.4, 32.1, 30.4, 27.8, 24.0, 18.5, 11.5, 4.7],
    [32.1, 31.6, 30.8, 29.8, 28.3, 26.1, 22.8, 17.9, 11.4, 4.7],
    [99.3, 28.9, 28.2, 27.4, 26.1, 24.2, 21.4, 17.1, 11.2, 4.7],
    [86.4, 26.0, 25.5, 24.8, 23.8, 22.3, 19.9, 16.2, 10.9, 4.7],
    [73.4, 23.1, 22.7, 22.2, 21.4, 20.1, 18.2, 15.2, 10.5, 4.7],
    [60.3, 20.1, 19.8, 19.4, 18.8, 17.8, 16.4, 13.9, 10.1, 4.6],
    [57.2, 17.0, 16.8, 16.5, 16.1, 15.4, 14.3, 12.5, 9.4, 4.6],
    [43.9, 13.8, 13.7, 13.5, 13.2, 12.7, 12.0, 10.7, 8.4, 4.5],
    [30.6, 10.5, 10.4, 10.3, 10.2, 9.9, 9.5, 8.7, 7.2, 4.2],
    [2.2, 7.1, 7.1, 7.0, 7.0, 6.8, 6.6, 6.2, 5.5, 3.7],
    [1.6, 3.6, 3.6, 3.6, 3.6, 3.5, 3.5, 3.4, 3.2, 2.5],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
];
