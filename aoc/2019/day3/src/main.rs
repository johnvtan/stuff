#[derive(Clone, Copy, PartialEq, Debug)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn distance(&self) -> isize {
        self.x.abs() + self.y.abs()
    }
}

#[derive(Debug)]
struct Segment {
    start: Point,
    end: Point,
}

fn overlap(start1: isize, end1: isize, start2: isize, end2: isize)  -> (isize, isize) {
    assert!(end1 >= start1);
    assert!(end2 >= start2);
    (std::cmp::max(start1, start2), std::cmp::min(end1, end2))
}

impl Segment {
    fn intersects(&self, other: &Segment) -> Option<Point> {
        self.assert_valid();
        other.assert_valid();

        let (self_start, self_end) = self.standardize();
        let (other_start, other_end) = other.standardize();

        if self.vertical() && other.vertical() {
            if self_start.x == other_start.x  {
                if self_start.y > other_end.y || other_start.y > self_end.y {
                    None
                } else {
                    let (overlap_start, overlap_end) = overlap(self_start.y, self_end.y, other_start.y, other_end.y);
                    //println!("ost {:?}, oend {:?}", overlap_start, overlap_end);
                    if overlap_start <= 0 && overlap_end >= 0 {
                        Some(Point { x: self_start.x, y: 0} )
                    } else {
                        let y = if overlap_start.abs() < overlap_end.abs() { overlap_start } else { overlap_end };
                        Some(Point{ x: self_start.x, y}) 
                    }
                }
            } else {
                None
            }
        } else if self.horizontal() && other.horizontal() {
            if self_start.y == other_start.y  {
                if self_start.x > other_end.x || other_start.x > self_end.x {
                    None
                } else {
                    let (overlap_start, overlap_end) = overlap(self_start.x, self_end.x, other_start.x, other_end.x);
                    if overlap_start <= 0 && overlap_end >= 0 {
                        Some(Point { x: 0, y: self_start.y} )
                    } else {
                        let x = if overlap_start.abs() < overlap_end.abs() { overlap_start } else { overlap_end };
                        Some(Point{ x, y: self_start.y}) 
                    }
                }
            } else {
                None
            }
        } else {
            // orthogonal case
            if self.start.x == self.end.x {
                if (other_start.x <= self_start.x)
                    && (self_start.x <= other_end.x)
                    && (self_start.y <= other_start.y)
                    && (other_start.y <= self_end.y)
                {
                    Some(Point { x: self_start.x, y: other_start.y })
                } else {
                    None
                }
            } else {
                if (other_start.y <= self_start.y)
                    && (self_start.y <= other_end.y)
                    && (self_start.x <= other_start.x)
                    && (other_start.x <= self_end.x)
                {
                    Some(Point { x: other_start.x, y: self_start.y })
                } else {
                    None
                }
            }
        }
        // None
    }

    fn assert_valid(&self) {
        assert!(self.start.x == self.end.x || self.start.y == self.end.y);
    }

    fn vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    fn horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    // Ensures that ret[0] is always left or below ret[1]
    fn standardize(&self) -> (Point, Point) {
        if self.start.x < self.end.x {
            (self.start, self.end)
        } else if self.start.x > self.end.x {
            (self.end, self.start)
        } else if self.start.y < self.end.y {
            (self.start, self.end)
        } else {
            assert!(self.start.y > self.end.y);
            (self.end, self.start)
        }
    }

    fn steps(&self) -> isize {
        let (start, end) = self.standardize();
        if self.vertical() {
            end.y - start.y
        } else {
            end.x - start.x
        }
    }
}

struct Wire {
    segments: Vec<Segment>,
}

impl Wire {
    fn parse(input: String) -> Self {
        let mut last_point = Point { x: 0, y: 0};
        let mut segments = vec![];

        for wire_segment in input.split(",") {
            let direction = wire_segment.chars().nth(0).unwrap();
            let magnitude = wire_segment[1..].parse::<isize>().unwrap();
            let Point { x, y } = last_point;

            let new_point =  match direction {
                'U' => Point {
                    x,
                    y: y + magnitude,
                },
                'D' => Point {
                    x,
                    y: y - magnitude,
                },
                'L' => Point {
                    x: x - magnitude,
                    y,
                },
                'R' => Point {
                    x: x + magnitude,
                    y,
                },
                _ => unreachable!(),
            };

            segments.push(Segment{ start: last_point, end: new_point});
            //println!("last_seg {:?}", segments.last());
            last_point = new_point;
        }
        Wire { segments }
    }

}

fn find_closest_intersection(w1: &Wire, w2: &Wire) -> isize {
    let mut closest = isize::max_value();
    for w1_seg in &w1.segments {
        for w2_seg in &w2.segments {
            if let Some(intersection) = w1_seg.intersects(&w2_seg) {
                if intersection.distance() > 0 {
                    closest = std::cmp::min(intersection.distance(), closest);
                }
            }
        }
    }
    closest
}

fn find_closest_intersection_steps(w1: &Wire, w2: &Wire) -> isize {
    let mut min_steps = isize::max_value();

    let mut w1_steps = 0;
    for w1_seg in &w1.segments {
        let mut w2_steps = 0;
        for w2_seg in &w2.segments {
            if let Some(intersection) = w1_seg.intersects(&w2_seg) {
                if intersection.distance() > 0 {
                    let new_w1_seg = Segment { start: w1_seg.start, end: intersection };
                    let new_w2_seg = Segment { start: w2_seg.start, end: intersection };
                    let steps = w1_steps + w2_steps + new_w1_seg.steps() + new_w2_seg.steps();
                    min_steps = std::cmp::min(steps, min_steps);
                }
            }
            w2_steps += w2_seg.steps();
        }

        w1_steps += w1_seg.steps();
    }
    min_steps
}

fn main() {
    let w1 = Wire::parse("R1003,U741,L919,U341,L204,U723,L113,D340,L810,D238,R750,U409,L104,U65,R119,U58,R94,D738,L543,U702,R612,D998,L580,U887,R664,D988,R232,D575,R462,U130,L386,U386,L217,U155,L68,U798,R792,U149,L573,D448,R76,U896,L745,D640,L783,D19,R567,D271,R618,U677,L449,D651,L843,D117,L636,U329,R484,U853,L523,U815,L765,U834,L500,U321,R874,U90,R473,U31,R846,U549,L70,U848,R677,D557,L702,U90,R78,U234,R282,D289,L952,D514,R308,U255,R752,D338,L134,D335,L207,U167,R746,U328,L65,D579,R894,U716,R510,D932,L396,U766,L981,D115,L668,U197,R773,U898,L22,U294,L548,D634,L31,U626,R596,U442,L103,U448,R826,U511,R732,U680,L279,D693,R292,U641,R253,U977,R699,U861,R534,D482,L481,U929,L244,U863,L951,D744,R775,U198,L658,U700,L740,U725,R286,D105,L629,D117,L991,D778,L627,D389,R942,D17,L791,D515,R231,U418,L497,D421,L508,U91,R841,D823,L88,U265,L223,D393,L399,D390,L431,D553,R40,U724,L566,U121,L436,U797,L42,U13,R19,D858,R912,D571,L207,D5,L981,D996,R814,D918,L16,U872,L5,U281,R706,U596,R827,D19,R976,D664,L930,U56,R168,D892,R661,D751,R219,U343,R120,U21,L659,U976,R498,U282,R1,U721,R475,D798,L5,U396,R268,D454,R118,U260,L709,D369,R96,D232,L320,D763,R548,U670,R102,D253,L947,U845,R888,D645,L734,D734,L459,D638,L82,U933,L485,U235,R181,D51,L45,D979,L74,D186,L513,U974,R283,D493,R128,U909,L96,D861,L291,U640,R793,D712,R421,D315,L152,U220,L252,U642,R126,D417,R137,D73,R1,D711,R880,U718,R104,U444,L36,D974,L360,U12,L890,D337,R184,D745,R164,D931,R915,D999,R452,U221,L399,D761,L987,U562,R25,D642,R411,D605,R964".to_string());
    let w2 = Wire::parse("L1010,U302,L697,D105,R618,U591,R185,U931,R595,D881,L50,D744,L320,D342,L221,D201,L862,D959,R553,D135,L238,U719,L418,U798,R861,U80,L571,U774,L896,U772,L960,U368,R415,D560,R276,U33,L532,U957,R621,D137,R373,U53,L842,U118,L299,U203,L352,D531,R118,U816,R355,U678,L983,D175,R652,U230,R190,D402,R111,D842,R756,D961,L82,U206,L576,U910,R622,D494,R630,D893,L200,U943,L696,D573,L143,D640,L885,D184,L52,D96,L580,U204,L793,D806,R477,D651,L348,D318,L924,D700,R675,D689,L723,D418,L156,D215,L943,D397,L301,U350,R922,D721,R14,U399,L774,U326,L14,D465,L65,U697,R564,D4,L40,D250,R914,U901,R316,U366,R877,D222,L672,D329,L560,U882,R321,D169,R161,U891,L552,U86,L194,D274,L567,D669,L682,U60,L985,U401,R587,U569,L1,D325,L73,U814,L338,U618,L49,U67,L258,D596,R493,D249,L310,D603,R810,D735,L829,D378,R65,U85,L765,D854,L863,U989,L595,U564,L373,U76,R923,U760,L965,U458,L610,U461,R900,U151,L650,D437,L1,U464,L65,D349,R256,D376,L686,U183,L403,D354,R867,U993,R819,D333,L249,U466,L39,D878,R855,U166,L254,D532,L909,U48,L980,U652,R393,D291,L502,U230,L738,U681,L393,U935,L333,D139,L499,D813,R302,D415,L693,D404,L308,D603,R968,U753,L510,D356,L356,U620,R386,D205,R587,U212,R715,U360,L603,U792,R58,U619,R73,D958,L53,D666,L756,U71,L621,D576,L174,U779,L382,U977,R890,D830,R822,U312,R716,U767,R36,U340,R322,D175,L417,U710,L313,D526,L573,D90,L493,D257,L918,U425,R93,D552,L691,U792,R189,U43,L633,U934,L953,U817,L404,D904,L384,D15,L670,D889,L648,U751,L928,D744,L932,U761,R879,D229,R491,U902,R134,D219,L634,U423,L241".to_string());
    println!("{:?}", find_closest_intersection_steps(&w1, &w2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn segment_intersects() {
        {
            let s1 = Segment {
                start: Point { x: 0, y: 0 },
                end: Point { x: 5, y: 0 },
            };

            let s2 = Segment {
                start: Point { x: 3, y: -1 },
                end: Point { x: 3, y: 4 },
            };

            assert_eq!(s1.intersects(&s2), Some(Point { x: 3, y: 0 }));
            assert_eq!(s2.intersects(&s1), Some(Point { x: 3, y: 0 }));
        }
        {
            let s1 = Segment {
                start: Point { x: 5, y: 0 },
                end: Point { x: 0, y: 0 },
            };

            let s2 = Segment {
                start: Point { x: 3, y: 4 },
                end: Point { x: 3, y: -1 },
            };

            assert_eq!(s1.intersects(&s2), Some(Point { x: 3, y: 0 }));
            assert_eq!(s2.intersects(&s1), Some(Point { x: 3, y: 0 }));
        }
        {
            let s1 = Segment {
                start: Point { x: 5, y: 0 },
                end: Point { x: 0, y: 0 },
            };

            // doesn't cross 0
            let s2 = Segment {
                start: Point { x: 3, y: 4 },
                end: Point { x: 3, y: 1 },
            };

            assert_eq!(s1.intersects(&s2), None);
            assert_eq!(s2.intersects(&s1), None);
        }
        {
            // doesn't cross 3
            let s1 = Segment {
                start: Point { x: 2, y: 0 },
                end: Point { x: 0, y: 0 },
            };

            let s2 = Segment {
                start: Point { x: 3, y: 4 },
                end: Point { x: 3, y: -1 },
            };

            assert_eq!(s1.intersects(&s2), None);
            assert_eq!(s2.intersects(&s1), None);
        }
        {
            // parallel
            let s1 = Segment {
                start: Point { x: 5, y: 0 },
                end: Point { x: 0, y: 0 },
            };

            let s2 = Segment {
                start: Point { x: 5, y: 1 },
                end: Point { x: 0, y: 1 },
            };

            assert_eq!(s1.intersects(&s2), None);
            assert_eq!(s2.intersects(&s1), None);
        }
        {
            // parallel
            let s1 = Segment {
                start: Point { x: 5, y: 1 },
                end: Point { x: 5, y: 0 },
            };

            let s2 = Segment {
                start: Point { x: 0, y: 1 },
                end: Point { x: 0, y: 0 },
            };

            assert_eq!(s1.intersects(&s2), None);
            assert_eq!(s2.intersects(&s1), None);
        }
        {
            // touch at end point orthogonal
            let s1 = Segment {
                start: Point { x: 5, y: 1 },
                end: Point { x: 5, y: 0 },
            };

            let s2 = Segment {
                start: Point { x: 0, y: 1 },
                end: Point { x: 5, y: 1 },
            };

            assert_eq!(s1.intersects(&s2), Some(Point { x: 5, y: 1 }));
            assert_eq!(s2.intersects(&s1), Some(Point { x: 5, y: 1 }));
        }

        {
            // touch at end point in same axis
            let s1 = Segment {
                start: Point { x: 5, y: 1},
                end: Point { x: 5, y: 0},
            };

            let s2 = Segment {
                start: Point { x: 5, y: 1 },
                end: Point { x: 5, y: 2 },
            };

            assert_eq!(s1.intersects(&s2), Some(Point{x: 5, y:1}));
            assert_eq!(s2.intersects(&s1), Some(Point{x: 5, y:1}));
        }

        {
            // touch at end point in same axis
            let s1 = Segment {
                start: Point { x: 5, y: 5},
                end: Point { x: 5, y: 0},
            };

            let s2 = Segment {
                start: Point { x: 5, y: 1 },
                end: Point { x: 5, y: 2 },
            };

            assert_eq!(s1.intersects(&s2), Some(Point{x: 5, y: 1}));
            assert_eq!(s2.intersects(&s1), Some(Point{x: 5, y: 1}));
        }

        {
            // touch at end point in same axis
            let s1 = Segment {
                start: Point { x: 5, y: -5},
                end: Point { x: 5, y: -2},
            };

            let s2 = Segment {
                start: Point { x: 5, y: -1 },
                end: Point { x: 5, y: -7 },
            };

            assert_eq!(s1.intersects(&s2), Some(Point{x: 5, y: -2}));
            assert_eq!(s2.intersects(&s1), Some(Point{x: 5, y: -2}));
        }

        {
            // touch at end point in same axis
            let s1 = Segment {
                start: Point { x: 5, y: 5},
                end: Point { x: 5, y: -2},
            };

            let s2 = Segment {
                start: Point { x: 5, y: 2 },
                end: Point { x: 5, y: -7 },
            };

            assert_eq!(s1.intersects(&s2), Some(Point{x: 5, y: 0}));
            assert_eq!(s2.intersects(&s1), Some(Point{x: 5, y: 0}));
        }

        {
            // touch at end point in same axis
            let s1 = Segment {
                start: Point { x: 5, y: 5},
                end: Point { x: 5, y: 2},
            };

            let s2 = Segment {
                start: Point { x: 5, y: 1 },
                end: Point { x: 5, y: 7 },
            };

            assert_eq!(s1.intersects(&s2), Some(Point{x: 5, y: 2}));
            assert_eq!(s2.intersects(&s1), Some(Point{x: 5, y: 2}));
        }
        {
            // touch at end point in same axis
            let s1 = Segment {
                start: Point { x: 5, y: 0},
                end: Point { x: 5, y: 1},
            };

            let s2 = Segment {
                start: Point { x: 5, y: 0 },
                end: Point { x: 5, y: -3 },
            };

            assert_eq!(s1.intersects(&s2), Some(Point{x: 5, y: 0}));
            assert_eq!(s2.intersects(&s1), Some(Point{x: 5, y: 0}));
        }





    }
}
