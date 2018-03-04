
use std::env;
use std::io::{BufReader, BufRead, Write};
use std::fs::File;
use std::str::FromStr;
use std::collections::HashMap;
use std::cmp::PartialEq;

pub struct Grid {
    rows: u64,
    cols: u64,
    vehicles: Vec<Vehicle>,
    rides: Vec<Ride>,
    bonus: u64,
    steps: u64,
}

impl Grid {
    pub fn new(rows: u64, cols: u64, vehicles: usize, rides: usize, bonus: u64, steps: u64) -> Grid {
        let mut v = Vec::with_capacity(vehicles);

        for i in 0..vehicles {
            v.push(Vehicle::new(i as u64));
        }

        Grid {
            rows: rows,
            cols: cols,
            vehicles: v,
            rides: Vec::with_capacity(rides),
            bonus: bonus,
            steps: steps,
        }
    }

    pub fn add_ride(&mut self, r: usize, s: String) {
        let p: Vec<&str> = s.split(' ').collect();
        assert!(p.len() == 6, format!("Wrong input {}", s));

        self.rides.push(Ride::new(r as u64, p[0].parse().unwrap(), p[1].parse().unwrap(), p[2].parse().unwrap(), p[3].parse().unwrap(), p[4].parse().unwrap(), p[5].parse().unwrap()));
    }

//     pub fn should_I_wait(&self, vehicle: u64, wait: u64, ride: u64) -> bool {
//         if wait == 0 {
//             return true;
//         }
// 
//         //catalog the actual vehicles distance from ride's start, if they ends in time
//         let mut distances: HashMap<u64, Vec<u64>> = HashMap::new();
//         for v in 0..self.vehicles.len() {
//             if self.vehicles[v].pos.t <= wait {
//                 let t = self.vehicles[v].get_time_to_start_of(&self.rides[ride as usize]);
//                 if distances.contains_key(&t) {
//                     distances.get_mut(&t).unwrap().push(v as u64);
//                 }
//                 else {
//                     distances.insert(t, vec![v as u64]);
//                 }
//             }
//         }
// 
//         let mut d:Vec<&u64> = distances.keys().collect();
//         d.sort();
//         match d.first() {
//             Some(i) => {
//                 //between the nearest vehicles, am I the best one?
//                 if distances[*i].contains(&vehicle) {
//                     //we have to check between actual 
//                     let mut distances: HashMap<u64, Vec<u64>> = HashMap::new();
//                     for v in distances[*i] {
//                     }
//                 }
//                 else {
//                     false
//                 }
//             },
//             None => {
//                 //println!("Vehicle {} skipping ride {} because it would wait too much ({})", vehicle, ride, wait);
//                 false
//             },
//         }
//     }

    pub fn run(&mut self) {
        for step in 0..self.steps {
            for v in 0..self.vehicles.len() {
                if self.vehicles[v].is_free() {
                    //println!("Vehicle {} is free", v);
                    let mut points:HashMap<u64, Vec<usize>> = HashMap::new();
                    let mut times:HashMap<usize, u64> = HashMap::new();
                    for r in 0..self.rides.len() {
                        //for this Vehicle this ride isn't feasible
                        match self.vehicles[v].get_time_to_end(step, &self.rides[r]) {
                            Some(t) => {
                                if (step + t) > self.steps {
                                    //println!("Discarding ride {} because ends out of time ({} > {})", r, step + t + w, self.steps);
                                    continue;
                                }

                                let p = self.vehicles[v].get_points(step, self.bonus, &self.rides[r]);
                                //println!("Ride {} would ends at {} giving {} points", r, t, p);
                                if points.contains_key(&p) {
                                    points.get_mut(&p).unwrap().push(r);
                                }
                                else {
                                    points.insert(p, vec![r]);
                                }
                                times.insert(r, t);
                            },
                            None => {
                                continue;
                            },
                        }
                    }

                    let mut temp:Vec<&u64> = points.keys().collect();
                    temp.sort();
                    match temp.last() {
                        Some(i) => {
                            let rides = &points[*i];
                            //println!("Choosing between {} equivalent rides giving {} point", rides.len(), *i);
                            let mut rides_times: HashMap<u64, usize> = HashMap::new();
                            for r in rides {
                                rides_times.insert(times[&r], *r);
                            }

                            let mut temp:Vec<&u64> = rides_times.keys().collect();
                            temp.sort();
                            match temp.first() {
                                Some(i) => {
                                    //println!("Choosed ride {} because lasts {} steps", rides_times[*i], *i);
                                    self.vehicles[v].set_ride(step, self.rides.swap_remove(rides_times[*i]));
                                },
                                None => {
                                    //println!("WTF?");
                                },
                            }
                        },
                        None => {
                            //println!("Vehicle {} hasn't viable rides", v);
                        },
                    }
                }
            }
        }
    }
}

impl FromStr for Grid {
    type Err = String;

    fn from_str(d: &str) -> Result<Self, Self::Err> {
        let s = d.to_owned();
        let p: Vec<&str> = s.split(' ').collect();
        if p.len() != 6 {
            Err(format!("Wrong input {}", d))
        }
        else {
            Ok(Grid::new(p[0].parse().unwrap(), p[1].parse().unwrap(), p[2].parse().unwrap(), p[3].parse().unwrap(), p[4].parse().unwrap(), p[5].parse().unwrap()))
        }
    }
}

impl ToString for Grid {
    fn to_string(&self) -> String {
        let mut s = self.vehicles[0].to_string();

        for n in 1..self.vehicles.len() {
            s = format!("{}\n{}", s, self.vehicles[n].to_string());
        }

        s
    }
}

#[derive(PartialEq)]
pub struct Ride {
    index: u64,
    start: Intersection,
    end: Intersection,
}

impl Ride {
    pub fn new(index: u64, a: u64, b: u64, x: u64, y: u64, s: u64, f: u64) -> Ride {
        Ride {
            index: index,
            start: Intersection::new(a, b, s),
            end: Intersection::new(x, y, f),
        }
    }
}

impl ToString for Ride {
    fn to_string(&self) -> String {
        format!("start: {}\nend: {}", self.start.to_string(), self.end.to_string())
    }
}

#[derive(PartialEq)]
pub struct Intersection {
    x: u64,
    y: u64,
    t: u64,
}

impl Intersection {
    pub fn new(x: u64, y: u64, t: u64) -> Intersection {
        Intersection {
            x: x,
            y: y,
            t: t,
        }
    }

    fn get_distance(a: &Intersection, b: &Intersection) -> u64 {
        ((a.x as i64 - b.x as i64).abs() + (a.y as i64 - b.y as i64).abs()) as u64
    }
}

impl ToString for Intersection {
    fn to_string(&self) -> String {
        format!("x: {} y: {} t: {}", self.x, self.y, self.t)
    }
}

pub struct Vehicle {
    index: u64,
    runs: Vec<u64>,
    pos: Intersection,
    cur_ride: Option<Ride>
}

impl Vehicle {
    pub fn new(index: u64) -> Vehicle {
        Vehicle {
            index: index,
            runs: Vec::new(),
            pos: Intersection::new(0, 0, 0),
            cur_ride: None,
        }
    }

    pub fn get_remaining_time(&self) -> u64 {
        match self.cur_ride {
            Some(ref ride) => {
                Intersection::get_distance(&self.pos, &ride.end)
            },
            None => 0,
        }
    }

    pub fn get_start_distance(&self, r: &Ride) -> u64 {
        Intersection::get_distance(&self.pos, &r.start)
    }

    pub fn get_end_distance(&self, r: &Ride) -> u64 {
        Intersection::get_distance(&self.pos, &r.end)
    }

    pub fn get_time_to_start_of(&self, r: &Ride) -> u64 {
        match self.cur_ride {
            Some(ref ride) => self.pos.t + Intersection::get_distance(&ride.end, &r.start),
            None => Intersection::get_distance(&self.pos, &r.start),
        }
    }

    pub fn get_points(&self, step: u64, bonus: u64, r: &Ride) -> u64 {
        let mut points = Intersection::get_distance(&r.start, &r.end);

        if step <= r.start.t {
            points += bonus;
        }

        points
    }

    pub fn get_time_to_end(&self, step: u64, r: &Ride) -> Option<u64> {
        let mut time = self.get_start_distance(r);

        if (step + time) < r.start.t {
            time += r.start.t - (step + time);
        }

        time += Intersection::get_distance(&r.start, &r.end);

        if (step + time) > r.end.t {
            //println!("Discarding ride {} because cannot end in time ({} > {})", r.index, step + time, r.end.t);
            None
        }
        else {
            Some(time)
        }
    }

    pub fn is_free(&mut self) -> bool {
        if self.cur_ride.is_some() {
            self.pos.t -= 1;
            //println!("Vehicle {} moved, {} moves to end", self.index, self.pos.t);
            if self.pos.t == 0 {
                match self.cur_ride {
                    Some(ref r) => {
                        self.pos.x = r.end.x;
                        self.pos.y = r.end.y;
                    },
                    None => {
                        assert!(false, "How is it even possible?");
                    },
                }
                self.cur_ride = None;
                true
            }
            else {
                false
            }
        }
        else {
            true
        }
    }

    pub fn set_ride(&mut self, step: u64, ride: Ride) {
        //println!("Vehicle {} taking ride {}", self.index, ride.index);
        self.pos.t = self.get_time_to_end(step, &ride).unwrap();
        self.runs.push(ride.index);
        self.cur_ride = Some(ride);
    }
}

impl ToString for Vehicle {
    fn to_string(&self) -> String {
        let mut s = self.runs.len().to_string();

        for n in 0..self.runs.len() {
            s = format!("{} {}", s, self.runs[n].to_string());
        }

        s
    }
}

fn main() {
    let args:Vec<String> = env::args().collect();
    assert!(args.len() == 3, "Usage: <file.in> <file.out>");

    let fin = File::open(args.get(1).unwrap()).unwrap();
    let file = BufReader::new(&fin);
    let mut iter = file.lines();

    let mut g:Grid = iter.next().unwrap().unwrap().parse().unwrap();//orribile
    for i in 0..g.rides.capacity() {
        g.add_ride(i, iter.next().unwrap().unwrap());
    }

    g.run();

    let mut fout = File::create(args.get(2).unwrap()).unwrap();
    fout.write_all(&g.to_string().into_bytes()).unwrap();
}
