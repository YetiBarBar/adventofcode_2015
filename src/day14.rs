/*
Rudolph =>(22, 8 ,165))
Cupid   => (8 ,17 ,114))
Prancer =>,  (18, 6 ,103))
Donner  =>,(25, 6 ,145))
Dasher  =>(11 ,12 ,125))
Comet   => (21, 6 ,121))
Blitzen =>,(1, 3, 50))
Vixen   =>(2, 4, 75))
Dancer  =>(7 ,20 ,119))
*/

#[derive(Debug)]
pub struct Reindeer {
    _name: &'static str,
    speed: usize,
    speed_time: usize,
    rest: usize,
}

impl Reindeer {
    #[must_use]
    pub fn dist_by_second(&self, time: usize) -> usize {
        let (div, remain) = (
            time.div_euclid(self.speed_time + self.rest),
            self.speed_time
                .min(time.rem_euclid(self.speed_time + self.rest)),
        );

        self.speed * (div * self.speed_time + remain)
    }
}

macro_rules! reindeer {
    ($($name:literal => ($speed:literal, $speed_time:literal, $rest:literal)),+ $(,)?) => {
        [$(

            Reindeer {
                _name : $name,
                speed   : $speed,
                speed_time :  $speed_time,
                rest: $rest,
            },
        )+]
    };
}

pub fn main() {
    let reindeers = reindeer! {
        "Rudolph" =>(22, 8 ,165),
        "Cupid"   => (8 ,17 ,114),
        "Prancer" =>  (18, 6 ,103),
        "Donner"  =>(25, 6 ,145),
        "Dasher"  =>(11 ,12 ,125),
        "Comet"   => (21, 6 ,121),
        "Blitzen" => (1, 3, 50),
        "Vixen"   =>(2, 4, 75),
        "Dancer"  =>(7 ,20 ,119),
    };

    let res = reindeers
        .iter()
        .map(|r| (r.dist_by_second(2503)))
        .max()
        .unwrap();
    println!("Part 1: {}", res);

    let max_by_seconds: Vec<_> = (0..=2503)
        .map(|time| {
            reindeers
                .iter()
                .map(|r| (r.dist_by_second(time)))
                .max()
                .unwrap()
        })
        .collect();

    let points = reindeers
        .iter()
        .map(|r| {
            let mut points = 0;
            // We don't attribute points at second 0 as
            // all reindeer is on starting line.
            for (idx, value) in max_by_seconds.iter().enumerate().take(2503).skip(1) {
                if r.dist_by_second(idx) == *value {
                    points += 1;
                }
            }
            points
        })
        .max();
    println!("Part 2: {:?}", points.unwrap());
}
