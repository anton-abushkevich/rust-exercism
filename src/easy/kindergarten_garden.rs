const KIDS: [&str; 12] = ["Alice", "Bob", "Charlie", "David", "Eve", "Fred",
    "Ginny", "Harriet", "Ileana", "Joseph", "Kincaid", "Larry",];

fn get_plant(code: char) -> &'static str {
    match code {
        'G' => "grass",
        'C' => "clover",
        'R' => "radishes",
        'V' => "violets",
        _ => "unknown"
    }
}

pub fn _plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let place = 2 * KIDS.iter().position(|kid| *kid == student).unwrap();
    let mut plants: Vec<&str> = Vec::new();
    diagram.split_whitespace().for_each(|x| {
        for code in x.get(place..=place+1).unwrap().chars() {
            plants.push(get_plant(code));
        }
    });

    plants

}

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let place = 2 * KIDS.iter().position(|kid| *kid == student).unwrap();
    diagram.lines()
        .flat_map(|line| line[place..=place+1].chars().map(get_plant))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn garden_with_single_student() {
        let diagram = "RC
GG";
        let student = "Alice";
        let expected = vec!["radishes", "clover", "grass", "grass"];
        assert_eq!(plants(diagram, student), expected);
    }

    #[test]
    fn for_charlie() {
        let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
        let student = "Charlie";
        let expected = vec!["violets", "violets", "clover", "grass"];
        assert_eq!(plants(diagram, student), expected);
    }
}
