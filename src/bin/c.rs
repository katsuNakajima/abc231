#[allow(unused_macros)]
macro_rules! parse_line {
    ( $t:ty ) => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            iter.next().unwrap().parse::<$t>().unwrap()
        }
    );

    ( $( $t:ty), +) => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            ( $(iter.next().unwrap().parse::<$t>().unwrap()),* )
        }
    );
}

#[allow(unused_macros)]
macro_rules! read_line {
    () => {{
        let mut line = String::new();
        ::std::io::stdin().read_line(&mut line).unwrap();
        line.pop();
        line
    }};
}

#[allow(unused_macros)]
macro_rules! parse_vec {
    ( $t:ty ) => {{
        let mut line = String::new();
        ::std::io::stdin().read_line(&mut line).unwrap();
        let iter = line.split_whitespace();
        iter.map(|v| v.parse::<$t>().unwrap()).collect::<Vec<_>>()
    }};
}

fn is_ok(n: i32, k: i32) -> bool {
    if n >= k {
        return true;
    } else {
        return false;
    }
}

fn bin_search(v: &Vec<i32>, num: i32) -> i32 {
    let mut ng = -1_i32;
    let mut ok = v.len() as i32;
    while (ok - ng).abs() > 1 {
        let mid = (ng + ok) / 2;
        let guess = v[mid as usize];
        if is_ok(guess, num) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    let ans = v.len() as i32 - ok;
    ans
}

fn main() {
    let (_n, q) = parse_line!(usize, usize);
    let mut a = parse_vec!(i32);
    a.sort();
    for _ in 0..q {
        let x = parse_line!(i32);
        let ans = bin_search(&a, x);
        println!("{}", ans);
    }
}
