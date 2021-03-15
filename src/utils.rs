use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub struct Split<I, P>
    where
    I: Iterator,
    P: Fn(&I::Item) -> bool
{
    iter: I,
    pred: P,
}

impl<I, P> Iterator for Split<I,P> where I: Iterator, P: Fn(&I::Item) -> bool {
    type Item = Vec<I::Item>;
    fn next(&mut self) -> Option<Vec<I::Item>> {
        let ref p = self.pred;
        let not_p = |x: &I::Item| !p(x);
        let mut i = self.iter.by_ref().skip_while(p).take_while(not_p);
        match i.next() {
            None    => None,
            Some(x) => {
                let mut v = Vec::new();
                v.push(x);
                v.extend(i);
                Some(v)
            }
        }
    }
}

pub trait SplitExt: Iterator {
    fn split<P>(self, p: P) -> Split<Self, P>
    where
        //Self::Item: Hash + Eq + Clone,
        Self: Sized,
        P: Fn(&Self::Item) -> bool,
    {
        Split {
            iter: self,
            pred: p,
        }
    }
}

impl<I: Iterator> SplitExt for I {}


/*pub fn split<I: Iterator, P: Fn(&I::Item) -> bool>(iter: I, pred: P) -> Split<I, P> {
    Split { iter: iter, pred: pred}
}*/