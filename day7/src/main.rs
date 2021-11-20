fn main() {
    let inputstr = include_str!("../../data/day7.txt");
    //let inputstr = TEST_STR;
    let input = parse_input(inputstr);
    let result = get_order(&input);
    println!("{}", result.iter().collect::<String>());
    println!("{}", get_timed_seconds(&input, 5));
}

/*
static TEST_STR: &str = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";
*/

#[derive(Clone, Copy, Debug)]
struct Step {
    name: u8, // ASCII byte in A..=Z
    seconds: u8, // remaining seconds
    steps: u32, // prerequisite steps to complete first
}

impl TryFrom<char> for Step {
    type Error = &'static str;

    fn try_from(name: char) -> Result<Step, &'static str>{
        if !('A'..='Z').contains(&name) {
            Err("Input step name must be in A..Z")
        } else {
            let n = name as u8;
            let s = n - b'A';
            Ok(Step {name: n, seconds: 1 + 60 + s, steps: 0})
        }
    }
}

impl Step {
    fn add_prerequisite(&mut self, other: &Step) {
        self.steps |= 1u32 << (other.name - b'A')
    }

    fn is_ready(&self) -> bool {
        self.steps == 0
    }

    fn is_done(&self) -> bool {
        self.seconds == 0
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
        if result[prerequisite_ind].is_none() {
            result[prerequisite_ind] = Some(Step::try_from(prerequisite_ch).unwrap());
        }
        if result[dependent_ind].is_none() {
            result[dependent_ind] = Some(Step::try_from(dependent_ch).unwrap());
        }
        let mut x = result[dependent_ind].unwrap();
        x.add_prerequisite(&result[prerequisite_ind].unwrap());
        result[dependent_ind] = Some(x);
    }
    result.into_iter().flatten().collect()
}

fn move_ready(ready: &mut Vec<Step>, unready: &mut Vec<Step>) {
    ready.extend(unready.iter().filter(|x| x.is_ready()));
    ready.sort_by(|x, y| y.name.cmp(&x.name));
    unready.retain(|x| !x.is_ready());
}

fn get_order(input: &[Step]) -> Vec<char> {
    let mut unready: Vec<_> = input.to_owned();
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
    result
}

fn get_timed_seconds(input: &[Step], workers: usize) -> usize {
    let mut unready = input.to_owned();
    let mut ready: Vec<Step> = Vec::new();
    let mut working: Vec<Step> = Vec::new();
    let mut elapsed: usize = 0;
    move_ready(&mut ready, &mut unready);
    loop {
        // For each stop done, mark other dependent steps as no longer waiting for this
        for step in working.iter_mut() {
            step.seconds -= 1;
            if step.is_done() {
                for i in unready.iter_mut() {
                    i.clear(step);
                }
            }
        };

        // Remove complete workers and tasks from their pools
        working.retain(|x| !x.is_done());
        let mut workers_available = workers - working.len();

        // Move ready tasks from unready to ready pool
        move_ready(&mut ready, &mut unready);

        // Check if any tasks and workers are available and mark them as working
        while !ready.is_empty() && workers_available > 0 {
            working.push(ready.pop().unwrap());
            workers_available -= 1;
        }

        // println!("{}: {}", elapsed, working.iter().map(|s| s.name as char).collect::<String>());
        if ready.is_empty() && working.is_empty() {
            return elapsed
        }        
        
        elapsed += 1;
    }
}
