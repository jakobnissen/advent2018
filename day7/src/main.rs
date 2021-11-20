fn main() {
    let inputstr = include_str!("../../data/day7.txt");
    //let inputstr = TEST_STR;
    let input = parse_input(inputstr);
    let result = get_order(&input);
    println!("{}", result.iter().collect::<String>());
}

static TEST_STR: &str = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";

#[derive(Clone, Copy, Debug)]
struct Step {
    name: u8, // ASCII byte in A..=Z
    steps: u32,
}

impl TryFrom<char> for Step {
    type Error = &'static str;

    fn try_from(name: char) -> Result<Step, &'static str>{
        if name > 'Z' || name < 'A' {
            Err("Input step name must be in A..Z")
        } else {
            Ok(Step {name: name as u8, steps: 0})
        }
    }
}

impl Step {
    fn add_prerequisite(&mut self, other: &Step) {
        self.steps |= 1u32 << (other.name - b'A')
    }

    fn is_ready(&self) -> bool {
        self.steps == 0u32
    }

    fn clear(&mut self, other: &Step) {
        self.steps &= !(1u32 << (other.name - b'A'))
    }
}


fn parse_input(lines: &str) -> Vec<Step> {
    let mut result: Vec<Option<Step>> = ('A'..='Z').map(|_| {None}).collect();
    for line in lines.lines().map(|s| s.trim()).filter(|s| !s.is_empty()) {
        let prerequisite_ch = line.chars().nth(5).unwrap();
        let prerequisite_ind = (prerequisite_ch as u8 - b'A') as usize;
        let dependent_ch = line.chars().nth(36).unwrap();
        let dependent_ind = (dependent_ch as u8 - b'A') as usize;
        if let None = result[prerequisite_ind] {
            result[prerequisite_ind] = Some(Step::try_from(prerequisite_ch).unwrap());
        }
        if let None = result[dependent_ind] {
            result[dependent_ind] = Some(Step::try_from(dependent_ch).unwrap());
        }
        let mut x = result[dependent_ind].unwrap();
        x.add_prerequisite(&result[prerequisite_ind].unwrap());
        result[dependent_ind] = Some(x);
    }
    let mut v: Vec<_> = result.into_iter().flatten().collect();
    v.sort_by(|x, y| y.name.cmp(&x.name));
    v
}

fn move_ready(ready: &mut Vec<Step>, unready: &mut Vec<Step>) {
    ready.extend(unready.iter().filter(|x| x.is_ready()));
    ready.sort_by(|x, y| y.name.cmp(&x.name));
    unready.retain(|x| !x.is_ready());
}

fn get_order(input: &Vec<Step>) -> Vec<char> {
    let mut unready = input.clone();
    let mut ready: Vec<Step> = Vec::new();
    let mut result = Vec::new();
    move_ready(&mut ready, &mut unready);
    while !ready.is_empty() {
        let step = ready.pop().unwrap();
        result.push(step.name as char);
        for i in unready.iter_mut() {
            i.clear(&step)
        };
        move_ready(&mut ready, &mut unready);
    }
    return result
}


/*
Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.
*/
