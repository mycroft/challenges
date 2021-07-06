use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
struct Team {
    matches: usize,
    win: usize,
    draw: usize,
    loss: usize,
    points: usize,
}

fn get_team(teams: &HashMap<String, Team>, team: String) -> Team {
    if teams.contains_key(&team) {
        *teams.get(&team).unwrap()
    } else {
        Team {
            matches: 0,
            win: 0,
            draw: 0,
            loss: 0,
            points: 0,
        }
    }
}

pub fn tally(match_results: &str) -> String {
    let mut out = String::from("Team                           | MP |  W |  D |  L |  P");
    let lines = match_results.split("\n");

    let mut teams : HashMap<String, Team> = HashMap::new();

    for line in lines {
        let fields = line.split(";").collect::<Vec<&str>>();

        if fields.len() != 3 {
            continue;
        }

        let mut team0 = get_team(&teams, fields[0].to_string());
        let mut team1 = get_team(&teams, fields[1].to_string());

        team0.matches += 1;
        team1.matches += 1;

        match fields[2] {
            "win" => {
                team0.win += 1;
                team0.points += 3;
                team1.loss += 1;
            },
            "loss" => {
                team1.win += 1;
                team1.points += 3;
                team0.loss += 1
            },
            "draw" => {
                team0.draw += 1;
                team1.draw += 1;
                team0.points += 1;
                team1.points += 1;
            },
            _ => unreachable!(),
        }

        teams.insert(fields[0].to_string(), team0);
        teams.insert(fields[1].to_string(), team1);
    }

    let mut v = teams
        .iter()
        .map(|(k, v)| (k, v))
        .collect::<Vec<(&String, &Team)>>();

    v.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    v.sort_by(|a, b| b.1.points.partial_cmp(&a.1.points).unwrap());

    for el in v.iter() {
        out.push_str(&format!("\n{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            el.0,
            el.1.matches,
            el.1.win,
            el.1.draw,
            el.1.loss,
            el.1.points));
    }

    out
}
