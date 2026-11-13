// b6: moving a field out from behind a reference.
//
// A `&mut Job` lets you change the job, but the job must still be a valid
// `Job` when the borrow ends, so its `name` cannot simply be moved away.
// Predict the error, then compile.
//
// Fix it WITHOUT `.clone()`: leave an empty String behind. See `std::mem`.

struct Job {
    name: String,
    attempts: u32,
}

fn take_name(job: &mut Job) -> String {
    job.attempts += 1;
    job.name
}

fn main() {
    let mut job = Job { name: String::from("backup"), attempts: 0 };
    let name = take_name(&mut job);
    println!("took {name:?}, left {:?}, attempts {}", job.name, job.attempts);
}
