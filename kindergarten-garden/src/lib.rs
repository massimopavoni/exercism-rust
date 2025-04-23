pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let cup_skip = (student.as_bytes()[0] - b'A') as usize * 2;
    let diagram_cut = diagram.len() / 2 + 1;

    (diagram[cup_skip..cup_skip + 2].to_owned() + &diagram[diagram_cut..][cup_skip..cup_skip + 2])
        .chars()
        .map(|c| match c {
            'G' => "grass",
            'C' => "clover",
            'R' => "radishes",
            'V' => "violets",
            _ => panic!("Invalid plant"),
        })
        .collect()
}
