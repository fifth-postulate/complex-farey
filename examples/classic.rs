use farey::farey::Fraction;

pub fn main() {
    let start: Vec<Fraction<u64>> = vec![(0, 1).into(), (1,1).into()];
    let a = start.iter();
    let mut b = start.iter();

    b.next();
    
    let result: Vec<Fraction<u64>> = a.zip(b).map(|(l,r)| l.mediant(*r)).collect();


    println!("{:?}", result)
}