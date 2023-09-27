{"payload":{"allShortcutsEnabled":false,"fileTree":{"rsa-rs/src":{"items":[{"name":"main.rs","path":"rsa-rs/src/main.rs","contentType":"file"}],"totalCount":1},"rsa-rs":{"items":[{"name":"src","path":"rsa-rs/src","contentType":"directory"},{"name":"target","path":"rsa-rs/target","contentType":"directory"},{"name":"log","path":"rsa-rs/log","contentType":"file"}],"totalCount":3},"":{"items":[{"name":"psuedo-RSA","path":"psuedo-RSA","contentType":"directory"},{"name":"rsa-rs","path":"rsa-rs","contentType":"directory"},{"name":"tests","path":"tests","contentType":"directory"},{"name":"README.md","path":"README.md","contentType":"file"},{"name":"factors","path":"factors","contentType":"file"},{"name":"factors.c","path":"factors.c","contentType":"file"},{"name":"factors.sh","path":"factors.sh","contentType":"file"},{"name":"rsa","path":"rsa","contentType":"file"},{"name":"rsa.sh","path":"rsa.sh","contentType":"file"}],"totalCount":9}},"fileTreeProcessingTime":8.119888000000001,"foldersToFetch":[],"reducedMotionEnabled":null,"repo":{"id":581677940,"defaultBranch":"master","name":"RSA-Factoring-Challenge","ownerLogin":"Lordwill1","currentUserCanPush":false,"isFork":false,"isEmpty":false,"createdAt":"2022-12-23T23:23:07.000Z","ownerAvatar":"https://avatars.githubusercontent.com/u/105258746?v=4","public":true,"private":false,"isOrgOwned":false},"symbolsExpanded":false,"treeExpanded":true,"refInfo":{"name":"master","listCacheKey":"v0:1671840303.705577","canEdit":false,"refType":"branch","currentOid":"1381cf58a3e19a876588c9b3d8de6a3cf60c504f"},"path":"rsa-rs/src/main.rs","currentUser":null,"blob":{"rawLines":["// (C) 2022 Eki Victory","// That's right.","#![allow(irrefutable_let_patterns)]","use std::{env, fs};","use ramp::int;","use rayon::prelude::*;","","","fn main() {","    let primes: Vec<int> = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29,","                            31, 37, 41, 43, 47, 53, 59, 61, 67,","                            71, 73, 79, 83, 89, 97].into_iter()","                                                   .map(|x| int::from(*x))","                                                   .collect();","    let filename = env::args().nth(1).unwrap();","    let contents = fs::read_to_string(filename).unwrap();","    let n: int = contents.lines().nth(0).unwrap().parse().unwrap();","    if n < 10000 {","        let (p, q) = look_up(&n, &primes);","        println!(\"{}={}*{}\", n, p, q);","    }","    else {","        let (p, q) = pollard_brent(&n);","        println!(\"{}={}*{}\", n, p, q);","    }","}","","/* https://stackoverflow.com/a/2274520/9221785 */","fn look_up(n: &int, ps: &Vec<int>) -> (int, int) {","    for p in ps.iter() {","\t\tif let (q, r) = n.divmod(p) {","            if r == 0 { return (p.clone(), q) };","        }","    }","    (int::one(), n.clone())","}","","/* Refs:"," * +http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.117.1230&rep=rep1&type=pdf"," * +https://comeoncodeon.wordpress.com/2010/09/18/pollard-rho-brent-integer-factorization/"," */","fn pollard_brent(num: &int) -> (int, int) {","    let (mut x, mut y) = (int::from(2), int::from(2));","    let (mut d, mut q) = (int::one(), int::one());","    let (mut ys, mut r) = (int::zero(), 1);","    const M: i32 = 71;","    let f = |i: &int| (i.pow_mod(&int::from(2), &num) + M) % num;","    while d == 1 {","        x = f(&y);","        for _ in 0..r {","            y = f(&y);","        }","        let mut k = 0_i32;","        while k < r && d == 1 {","            for _ in 0..(M.min(r - k)) {","                y = f(&y);","                q = q * (&x - &y).abs() % num;","            }","            ys = y.clone();","            d = q.gcd(num);","            k += M;","        }","        r *= 2;","    }","    if d == *num {","        loop {","            ys = f(&ys);","            d = ((&x - &ys).abs()).gcd(num);","            if d > 1 {","                break;","            }","        }","    }","    (d.clone(), num/d)","}"],"stylingDirectives":[[{"start":0,"end":23,"cssClass":"pl-c"}],[{"start":0,"end":16,"cssClass":"pl-c"}],[{"start":0,"end":35,"cssClass":"pl-c1"},{"start":2,"end":3,"cssClass":"pl-kos"},{"start":8,"end":9,"cssClass":"pl-kos"},{"start":33,"end":34,"cssClass":"pl-kos"},{"start":34,"end":35,"cssClass":"pl-kos"}],[{"start":0,"end":3,"cssClass":"pl-k"},{"start":7,"end":9,"cssClass":"pl-kos"},{"start":9,"end":10,"cssClass":"pl-kos"},{"start":13,"end":14,"cssClass":"pl-kos"},{"start":17,"end":18,"cssClass":"pl-kos"},{"start":18,"end":19,"cssClass":"pl-kos"}],[{"start":0,"end":3,"cssClass":"pl-k"},{"start":8,"end":10,"cssClass":"pl-kos"},{"start":13,"end":14,"cssClass":"pl-kos"}],[{"start":0,"end":3,"cssClass":"pl-k"},{"start":9,"end":11,"cssClass":"pl-kos"},{"start":18,"end":20,"cssClass":"pl-kos"},{"start":20,"end":21,"cssClass":"pl-c1"},{"start":21,"end":22,"cssClass":"pl-kos"}],[],[],[{"start":0,"end":2,"cssClass":"pl-k"},{"start":3,"end":7,"cssClass":"pl-en"},{"start":7,"end":8,"cssClass":"pl-kos"},{"start":8,"end":9,"cssClass":"pl-kos"},{"start":10,"end":11,"cssClass":"pl-kos"}],[{"start":4,"end":7,"cssClass":"pl-k"},{"start":14,"end":15,"cssClass":"pl-kos"},{"start":16,"end":19,"cssClass":"pl-smi"},{"start":19,"end":20,"cssClass":"pl-kos"},{"start":20,"end":23,"cssClass":"pl-smi"},{"start":23,"end":24,"cssClass":"pl-kos"},{"start":27,"end":28,"cssClass":"pl-kos"},{"start":28,"end":29,"cssClass":"pl-c1"},{"start":29,"end":30,"cssClass":"pl-kos"},{"start":31,"end":32,"cssClass":"pl-c1"},{"start":32,"end":33,"cssClass":"pl-kos"},{"start":34,"end":35,"cssClass":"pl-c1"},{"start":35,"end":36,"cssClass":"pl-kos"},{"start":37,"end":38,"cssClass":"pl-c1"},{"start":38,"end":39,"cssClass":"pl-kos"},{"start":40,"end":42,"cssClass":"pl-c1"},{"start":42,"end":43,"cssClass":"pl-kos"},{"start":44,"end":46,"cssClass":"pl-c1"},{"start":46,"end":47,"cssClass":"pl-kos"},{"start":48,"end":50,"cssClass":"pl-c1"},{"start":50,"end":51,"cssClass":"pl-kos"},{"start":52,"end":54,"cssClass":"pl-c1"},{"start":54,"end":55,"cssClass":"pl-kos"},{"start":56,"end":58,"cssClass":"pl-c1"},{"start":58,"end":59,"cssClass":"pl-kos"},{"start":60,"end":62,"cssClass":"pl-c1"},{"start":62,"end":63,"cssClass":"pl-kos"}],[{"start":28,"end":30,"cssClass":"pl-c1"},{"start":30,"end":31,"cssClass":"pl-kos"},{"start":32,"end":34,"cssClass":"pl-c1"},{"start":34,"end":35,"cssClass":"pl-kos"},{"start":36,"end":38,"cssClass":"pl-c1"},{"start":38,"end":39,"cssClass":"pl-kos"},{"start":40,"end":42,"cssClass":"pl-c1"},{"start":42,"end":43,"cssClass":"pl-kos"},{"start":44,"end":46,"cssClass":"pl-c1"},{"start":46,"end":47,"cssClass":"pl-kos"},{"start":48,"end":50,"cssClass":"pl-c1"},{"start":50,"end":51,"cssClass":"pl-kos"},{"start":52,"end":54,"cssClass":"pl-c1"},{"start":54,"end":55,"cssClass":"pl-kos"},{"start":56,"end":58,"cssClass":"pl-c1"},{"start":58,"end":59,"cssClass":"pl-kos"},{"start":60,"end":62,"cssClass":"pl-c1"},{"start":62,"end":63,"cssClass":"pl-kos"}],[{"start":28,"end":30,"cssClass":"pl-c1"},{"start":30,"end":31,"cssClass":"pl-kos"},{"start":32,"end":34,"cssClass":"pl-c1"},{"start":34,"end":35,"cssClass":"pl-kos"},{"start":36,"end":38,"cssClass":"pl-c1"},{"start":38,"end":39,"cssClass":"pl-kos"},{"start":40,"end":42,"cssClass":"pl-c1"},{"start":42,"end":43,"cssClass":"pl-kos"},{"start":44,"end":46,"cssClass":"pl-c1"},{"start":46,"end":47,"cssClass":"pl-kos"},{"start":48,"end":50,"cssClass":"pl-c1"},{"start":50,"end":51,"cssClass":"pl-kos"},{"start":51,"end":52,"cssClass":"pl-kos"},{"start":52,"end":61,"cssClass":"pl-en"},{"start":61,"end":62,"cssClass":"pl-kos"},{"start":62,"end":63,"cssClass":"pl-kos"}],[{"start":51,"end":52,"cssClass":"pl-kos"},{"start":52,"end":55,"cssClass":"pl-en"},{"start":55,"end":56,"cssClass":"pl-kos"},{"start":63,"end":65,"cssClass":"pl-kos"},{"start":65,"end":69,"cssClass":"pl-en"},{"start":69,"end":70,"cssClass":"pl-kos"},{"start":70,"end":71,"cssClass":"pl-c1"},{"start":72,"end":73,"cssClass":"pl-kos"},{"start":73,"end":74,"cssClass":"pl-kos"}],[{"start":51,"end":52,"cssClass":"pl-kos"},{"start":52,"end":59,"cssClass":"pl-en"},{"start":59,"end":60,"cssClass":"pl-kos"},{"start":60,"end":61,"cssClass":"pl-kos"},{"start":61,"end":62,"cssClass":"pl-kos"}],[{"start":4,"end":7,"cssClass":"pl-k"},{"start":22,"end":24,"cssClass":"pl-kos"},{"start":24,"end":28,"cssClass":"pl-en"},{"start":28,"end":29,"cssClass":"pl-kos"},{"start":29,"end":30,"cssClass":"pl-kos"},{"start":30,"end":31,"cssClass":"pl-kos"},{"start":31,"end":34,"cssClass":"pl-en"},{"start":34,"end":35,"cssClass":"pl-kos"},{"start":35,"end":36,"cssClass":"pl-c1"},{"start":36,"end":37,"cssClass":"pl-kos"},{"start":37,"end":38,"cssClass":"pl-kos"},{"start":38,"end":44,"cssClass":"pl-en"},{"start":44,"end":45,"cssClass":"pl-kos"},{"start":45,"end":46,"cssClass":"pl-kos"},{"start":46,"end":47,"cssClass":"pl-kos"}],[{"start":4,"end":7,"cssClass":"pl-k"},{"start":21,"end":23,"cssClass":"pl-kos"},{"start":23,"end":37,"cssClass":"pl-en"},{"start":37,"end":38,"cssClass":"pl-kos"},{"start":46,"end":47,"cssClass":"pl-kos"},{"start":47,"end":48,"cssClass":"pl-kos"},{"start":48,"end":54,"cssClass":"pl-en"},{"start":54,"end":55,"cssClass":"pl-kos"},{"start":55,"end":56,"cssClass":"pl-kos"},{"start":56,"end":57,"cssClass":"pl-kos"}],[{"start":4,"end":7,"cssClass":"pl-k"},{"start":9,"end":10,"cssClass":"pl-kos"},{"start":11,"end":14,"cssClass":"pl-smi"},{"start":25,"end":26,"cssClass":"pl-kos"},{"start":26,"end":31,"cssClass":"pl-en"},{"start":31,"end":32,"cssClass":"pl-kos"},{"start":32,"end":33,"cssClass":"pl-kos"},{"start":33,"end":34,"cssClass":"pl-kos"},{"start":34,"end":37,"cssClass":"pl-en"},{"start":37,"end":38,"cssClass":"pl-kos"},{"start":38,"end":39,"cssClass":"pl-c1"},{"start":39,"end":40,"cssClass":"pl-kos"},{"start":40,"end":41,"cssClass":"pl-kos"},{"start":41,"end":47,"cssClass":"pl-en"},{"start":47,"end":48,"cssClass":"pl-kos"},{"start":48,"end":49,"cssClass":"pl-kos"},{"start":49,"end":50,"cssClass":"pl-kos"},{"start":50,"end":55,"cssClass":"pl-en"},{"start":55,"end":56,"cssClass":"pl-kos"},{"start":56,"end":57,"cssClass":"pl-kos"},{"start":57,"end":58,"cssClass":"pl-kos"},{"start":58,"end":64,"cssClass":"pl-en"},{"start":64,"end":65,"cssClass":"pl-kos"},{"start":65,"end":66,"cssClass":"pl-kos"},{"start":66,"end":67,"cssClass":"pl-kos"}],[{"start":4,"end":6,"cssClass":"pl-k"},{"start":11,"end":16,"cssClass":"pl-c1"},{"start":17,"end":18,"cssClass":"pl-kos"}],[{"start":8,"end":11,"cssClass":"pl-k"},{"start":12,"end":13,"cssClass":"pl-kos"},{"start":14,"end":15,"cssClass":"pl-kos"},{"start":17,"end":18,"cssClass":"pl-kos"},{"start":21,"end":28,"cssClass":"pl-en"},{"start":28,"end":29,"cssClass":"pl-kos"},{"start":29,"end":30,"cssClass":"pl-c1"},{"start":31,"end":32,"cssClass":"pl-kos"},{"start":33,"end":34,"cssClass":"pl-c1"},{"start":40,"end":41,"cssClass":"pl-kos"},{"start":41,"end":42,"cssClass":"pl-kos"}],[{"start":8,"end":15,"cssClass":"pl-en"},{"start":15,"end":16,"cssClass":"pl-en"},{"start":16,"end":17,"cssClass":"pl-kos"},{"start":17,"end":27,"cssClass":"pl-s"},{"start":36,"end":37,"cssClass":"pl-kos"},{"start":37,"end":38,"cssClass":"pl-kos"}],[{"start":4,"end":5,"cssClass":"pl-kos"}],[{"start":4,"end":8,"cssClass":"pl-k"},{"start":9,"end":10,"cssClass":"pl-kos"}],[{"start":8,"end":11,"cssClass":"pl-k"},{"start":12,"end":13,"cssClass":"pl-kos"},{"start":14,"end":15,"cssClass":"pl-kos"},{"start":17,"end":18,"cssClass":"pl-kos"},{"start":21,"end":34,"cssClass":"pl-en"},{"start":34,"end":35,"cssClass":"pl-kos"},{"start":35,"end":36,"cssClass":"pl-c1"},{"start":37,"end":38,"cssClass":"pl-kos"},{"start":38,"end":39,"cssClass":"pl-kos"}],[{"start":8,"end":15,"cssClass":"pl-en"},{"start":15,"end":16,"cssClass":"pl-en"},{"start":16,"end":17,"cssClass":"pl-kos"},{"start":17,"end":27,"cssClass":"pl-s"},{"start":36,"end":37,"cssClass":"pl-kos"},{"start":37,"end":38,"cssClass":"pl-kos"}],[{"start":4,"end":5,"cssClass":"pl-kos"}],[{"start":0,"end":1,"cssClass":"pl-kos"}],[],[{"start":0,"end":49,"cssClass":"pl-c"}],[{"start":0,"end":2,"cssClass":"pl-k"},{"start":3,"end":10,"cssClass":"pl-en"},{"start":10,"end":11,"cssClass":"pl-kos"},{"start":11,"end":12,"cssClass":"pl-s1"},{"start":12,"end":13,"cssClass":"pl-kos"},{"start":14,"end":15,"cssClass":"pl-c1"},{"start":15,"end":18,"cssClass":"pl-smi"},{"start":18,"end":19,"cssClass":"pl-kos"},{"start":20,"end":22,"cssClass":"pl-s1"},{"start":22,"end":23,"cssClass":"pl-kos"},{"start":24,"end":25,"cssClass":"pl-c1"},{"start":25,"end":28,"cssClass":"pl-smi"},{"start":28,"end":29,"cssClass":"pl-kos"},{"start":29,"end":32,"cssClass":"pl-smi"},{"start":32,"end":33,"cssClass":"pl-kos"},{"start":33,"end":34,"cssClass":"pl-kos"},{"start":38,"end":39,"cssClass":"pl-kos"},{"start":39,"end":42,"cssClass":"pl-smi"},{"start":42,"end":43,"cssClass":"pl-kos"},{"start":44,"end":47,"cssClass":"pl-smi"},{"start":47,"end":48,"cssClass":"pl-kos"},{"start":49,"end":50,"cssClass":"pl-kos"}],[{"start":4,"end":7,"cssClass":"pl-k"},{"start":10,"end":12,"cssClass":"pl-k"},{"start":15,"end":16,"cssClass":"pl-kos"},{"start":16,"end":20,"cssClass":"pl-en"},{"start":20,"end":21,"cssClass":"pl-kos"},{"start":21,"end":22,"cssClass":"pl-kos"},{"start":23,"end":24,"cssClass":"pl-kos"}],[{"start":2,"end":4,"cssClass":"pl-k"},{"start":5,"end":8,"cssClass":"pl-k"},{"start":9,"end":10,"cssClass":"pl-kos"},{"start":11,"end":12,"cssClass":"pl-kos"},{"start":14,"end":15,"cssClass":"pl-kos"},{"start":19,"end":20,"cssClass":"pl-kos"},{"start":20,"end":26,"cssClass":"pl-en"},{"start":26,"end":27,"cssClass":"pl-kos"},{"start":28,"end":29,"cssClass":"pl-kos"},{"start":30,"end":31,"cssClass":"pl-kos"}],[{"start":12,"end":14,"cssClass":"pl-k"},{"start":20,"end":21,"cssClass":"pl-c1"},{"start":22,"end":23,"cssClass":"pl-kos"},{"start":24,"end":30,"cssClass":"pl-k"},{"start":31,"end":32,"cssClass":"pl-kos"},{"start":33,"end":34,"cssClass":"pl-kos"},{"start":34,"end":39,"cssClass":"pl-en"},{"start":39,"end":40,"cssClass":"pl-kos"},{"start":40,"end":41,"cssClass":"pl-kos"},{"start":41,"end":42,"cssClass":"pl-kos"},{"start":44,"end":45,"cssClass":"pl-kos"},{"start":46,"end":47,"cssClass":"pl-kos"},{"start":47,"end":48,"cssClass":"pl-kos"}],[{"start":8,"end":9,"cssClass":"pl-kos"}],[{"start":4,"end":5,"cssClass":"pl-kos"}],[{"start":4,"end":5,"cssClass":"pl-kos"},{"start":8,"end":10,"cssClass":"pl-kos"},{"start":10,"end":13,"cssClass":"pl-en"},{"start":13,"end":14,"cssClass":"pl-kos"},{"start":14,"end":15,"cssClass":"pl-kos"},{"start":15,"end":16,"cssClass":"pl-kos"},{"start":18,"end":19,"cssClass":"pl-kos"},{"start":19,"end":24,"cssClass":"pl-en"},{"start":24,"end":25,"cssClass":"pl-kos"},{"start":25,"end":26,"cssClass":"pl-kos"},{"start":26,"end":27,"cssClass":"pl-kos"}],[{"start":0,"end":1,"cssClass":"pl-kos"}],[],[{"start":0,"end":8,"cssClass":"pl-c"}],[{"start":0,"end":87,"cssClass":"pl-c"}],[{"start":0,"end":90,"cssClass":"pl-c"}],[{"start":0,"end":3,"cssClass":"pl-c"}],[{"start":0,"end":2,"cssClass":"pl-k"},{"start":3,"end":16,"cssClass":"pl-en"},{"start":16,"end":17,"cssClass":"pl-kos"},{"start":17,"end":20,"cssClass":"pl-s1"},{"start":20,"end":21,"cssClass":"pl-kos"},{"start":22,"end":23,"cssClass":"pl-c1"},{"start":23,"end":26,"cssClass":"pl-smi"},{"start":26,"end":27,"cssClass":"pl-kos"},{"start":31,"end":32,"cssClass":"pl-kos"},{"start":32,"end":35,"cssClass":"pl-smi"},{"start":35,"end":36,"cssClass":"pl-kos"},{"start":37,"end":40,"cssClass":"pl-smi"},{"start":40,"end":41,"cssClass":"pl-kos"},{"start":42,"end":43,"cssClass":"pl-kos"}],[{"start":4,"end":7,"cssClass":"pl-k"},{"start":8,"end":9,"cssClass":"pl-kos"},{"start":9,"end":12,"cssClass":"pl-k"},{"start":14,"end":15,"cssClass":"pl-kos"},{"start":16,"end":19,"cssClass":"pl-k"},{"start":21,"end":22,"cssClass":"pl-kos"},{"start":25,"end":26,"cssClass":"pl-kos"},{"start":29,"end":31,"cssClass":"pl-kos"},{"start":31,"end":35,"cssClass":"pl-en"},{"start":35,"end":36,"cssClass":"pl-kos"},{"start":36,"end":37,"cssClass":"pl-c1"},{"start":37,"end":38,"cssClass":"pl-kos"},{"start":38,"end":39,"cssClass":"pl-kos"},{"start":43,"end":45,"cssClass":"pl-kos"},{"start":45,"end":49,"cssClass":"pl-en"},{"start":49,"end":50,"cssClass":"pl-kos"},{"start":50,"end":51,"cssClass":"pl-c1"},{"start":51,"end":52,"cssClass":"pl-kos"},{"start":52,"end":53,"cssClass":"pl-kos"},{"start":53,"end":54,"cssClass":"pl-kos"}],[{"start":4,"end":7,"cssClass":"pl-k"},{"start":8,"end":9,"cssClass":"pl-kos"},{"start":9,"end":12,"cssClass":"pl-k"},{"start":14,"end":15,"cssClass":"pl-kos"},{"start":16,"end":19,"cssClass":"pl-k"},{"start":21,"end":22,"cssClass":"pl-kos"},{"start":25,"end":26,"cssClass":"pl-kos"},{"start":29,"end":31,"cssClass":"pl-kos"},{"start":31,"end":34,"cssClass":"pl-en"},{"start":34,"end":35,"cssClass":"pl-kos"},{"start":35,"end":36,"cssClass":"pl-kos"},{"start":36,"end":37,"cssClass":"pl-kos"},{"start":41,"end":43,"cssClass":"pl-kos"},{"start":43,"end":46,"cssClass":"pl-en"},{"start":46,"end":47,"cssClass":"pl-kos"},{"start":47,"end":48,"cssClass":"pl-kos"},{"start":48,"end":49,"cssClass":"pl-kos"},{"start":49,"end":50,"cssClass":"pl-kos"}],[{"start":4,"end":7,"cssClass":"pl-k"},{"start":8,"end":9,"cssClass":"pl-kos"},{"start":9,"end":12,"cssClass":"pl-k"},{"start":15,"end":16,"cssClass":"pl-kos"},{"start":17,"end":20,"cssClass":"pl-k"},{"start":22,"end":23,"cssClass":"pl-kos"},{"start":26,"end":27,"cssClass":"pl-kos"},{"start":30,"end":32,"cssClass":"pl-kos"},{"start":32,"end":36,"cssClass":"pl-en"},{"start":36,"end":37,"cssClass":"pl-kos"},{"start":37,"end":38,"cssClass":"pl-kos"},{"start":38,"end":39,"cssClass":"pl-kos"},{"start":40,"end":41,"cssClass":"pl-c1"},{"start":41,"end":42,"cssClass":"pl-kos"},{"start":42,"end":43,"cssClass":"pl-kos"}],[{"start":4,"end":9,"cssClass":"pl-k"},{"start":10,"end":11,"cssClass":"pl-v"},{"start":11,"end":12,"cssClass":"pl-kos"},{"start":13,"end":16,"cssClass":"pl-smi"},{"start":19,"end":21,"cssClass":"pl-c1"},{"start":21,"end":22,"cssClass":"pl-kos"}],[{"start":4,"end":7,"cssClass":"pl-k"},{"start":13,"end":14,"cssClass":"pl-s1"},{"start":14,"end":15,"cssClass":"pl-kos"},{"start":16,"end":17,"cssClass":"pl-c1"},{"start":17,"end":20,"cssClass":"pl-smi"},{"start":22,"end":23,"cssClass":"pl-kos"},{"start":24,"end":25,"cssClass":"pl-kos"},{"start":25,"end":32,"cssClass":"pl-en"},{"start":32,"end":33,"cssClass":"pl-kos"},{"start":33,"end":34,"cssClass":"pl-c1"},{"start":37,"end":39,"cssClass":"pl-kos"},{"start":39,"end":43,"cssClass":"pl-en"},{"start":43,"end":44,"cssClass":"pl-kos"},{"start":44,"end":45,"cssClass":"pl-c1"},{"start":45,"end":46,"cssClass":"pl-kos"},{"start":46,"end":47,"cssClass":"pl-kos"},{"start":48,"end":49,"cssClass":"pl-c1"},{"start":52,"end":53,"cssClass":"pl-kos"},{"start":56,"end":57,"cssClass":"pl-v"},{"start":57,"end":58,"cssClass":"pl-kos"},{"start":64,"end":65,"cssClass":"pl-kos"}],[{"start":4,"end":9,"cssClass":"pl-k"},{"start":15,"end":16,"cssClass":"pl-c1"},{"start":17,"end":18,"cssClass":"pl-kos"}],[{"start":12,"end":13,"cssClass":"pl-en"},{"start":13,"end":14,"cssClass":"pl-kos"},{"start":14,"end":15,"cssClass":"pl-c1"},{"start":16,"end":17,"cssClass":"pl-kos"},{"start":17,"end":18,"cssClass":"pl-kos"}],[{"start":8,"end":11,"cssClass":"pl-k"},{"start":14,"end":16,"cssClass":"pl-k"},{"start":17,"end":18,"cssClass":"pl-c1"},{"start":22,"end":23,"cssClass":"pl-kos"}],[{"start":16,"end":17,"cssClass":"pl-en"},{"start":17,"end":18,"cssClass":"pl-kos"},{"start":18,"end":19,"cssClass":"pl-c1"},{"start":20,"end":21,"cssClass":"pl-kos"},{"start":21,"end":22,"cssClass":"pl-kos"}],[{"start":8,"end":9,"cssClass":"pl-kos"}],[{"start":8,"end":11,"cssClass":"pl-k"},{"start":12,"end":15,"cssClass":"pl-k"},{"start":20,"end":25,"cssClass":"pl-c1"},{"start":25,"end":26,"cssClass":"pl-kos"}],[{"start":8,"end":13,"cssClass":"pl-k"},{"start":28,"end":29,"cssClass":"pl-c1"},{"start":30,"end":31,"cssClass":"pl-kos"}],[{"start":12,"end":15,"cssClass":"pl-k"},{"start":18,"end":20,"cssClass":"pl-k"},{"start":21,"end":22,"cssClass":"pl-c1"},{"start":24,"end":25,"cssClass":"pl-kos"},{"start":25,"end":26,"cssClass":"pl-v"},{"start":26,"end":27,"cssClass":"pl-kos"},{"start":27,"end":30,"cssClass":"pl-en"},{"start":30,"end":31,"cssClass":"pl-kos"},{"start":36,"end":37,"cssClass":"pl-kos"},{"start":37,"end":38,"cssClass":"pl-kos"},{"start":39,"end":40,"cssClass":"pl-kos"}],[{"start":20,"end":21,"cssClass":"pl-en"},{"start":21,"end":22,"cssClass":"pl-kos"},{"start":22,"end":23,"cssClass":"pl-c1"},{"start":24,"end":25,"cssClass":"pl-kos"},{"start":25,"end":26,"cssClass":"pl-kos"}],[{"start":22,"end":23,"cssClass":"pl-c1"},{"start":24,"end":25,"cssClass":"pl-kos"},{"start":25,"end":26,"cssClass":"pl-c1"},{"start":30,"end":31,"cssClass":"pl-c1"},{"start":32,"end":33,"cssClass":"pl-kos"},{"start":33,"end":34,"cssClass":"pl-kos"},{"start":34,"end":37,"cssClass":"pl-en"},{"start":37,"end":38,"cssClass":"pl-kos"},{"start":38,"end":39,"cssClass":"pl-kos"},{"start":45,"end":46,"cssClass":"pl-kos"}],[{"start":12,"end":13,"cssClass":"pl-kos"}],[{"start":18,"end":19,"cssClass":"pl-kos"},{"start":19,"end":24,"cssClass":"pl-en"},{"start":24,"end":25,"cssClass":"pl-kos"},{"start":25,"end":26,"cssClass":"pl-kos"},{"start":26,"end":27,"cssClass":"pl-kos"}],[{"start":17,"end":18,"cssClass":"pl-kos"},{"start":18,"end":21,"cssClass":"pl-en"},{"start":21,"end":22,"cssClass":"pl-kos"},{"start":25,"end":26,"cssClass":"pl-kos"},{"start":26,"end":27,"cssClass":"pl-kos"}],[{"start":17,"end":18,"cssClass":"pl-v"},{"start":18,"end":19,"cssClass":"pl-kos"}],[{"start":8,"end":9,"cssClass":"pl-kos"}],[{"start":13,"end":14,"cssClass":"pl-c1"},{"start":14,"end":15,"cssClass":"pl-kos"}],[{"start":4,"end":5,"cssClass":"pl-kos"}],[{"start":4,"end":6,"cssClass":"pl-k"},{"start":12,"end":13,"cssClass":"pl-c1"},{"start":17,"end":18,"cssClass":"pl-kos"}],[{"start":8,"end":12,"cssClass":"pl-k"},{"start":13,"end":14,"cssClass":"pl-kos"}],[{"start":17,"end":18,"cssClass":"pl-en"},{"start":18,"end":19,"cssClass":"pl-kos"},{"start":19,"end":20,"cssClass":"pl-c1"},{"start":22,"end":23,"cssClass":"pl-kos"},{"start":23,"end":24,"cssClass":"pl-kos"}],[{"start":16,"end":17,"cssClass":"pl-kos"},{"start":17,"end":18,"cssClass":"pl-kos"},{"start":18,"end":19,"cssClass":"pl-c1"},{"start":23,"end":24,"cssClass":"pl-c1"},{"start":26,"end":27,"cssClass":"pl-kos"},{"start":27,"end":28,"cssClass":"pl-kos"},{"start":28,"end":31,"cssClass":"pl-en"},{"start":31,"end":32,"cssClass":"pl-kos"},{"start":32,"end":33,"cssClass":"pl-kos"},{"start":33,"end":34,"cssClass":"pl-kos"},{"start":34,"end":35,"cssClass":"pl-kos"},{"start":35,"end":38,"cssClass":"pl-en"},{"start":38,"end":39,"cssClass":"pl-kos"},{"start":42,"end":43,"cssClass":"pl-kos"},{"start":43,"end":44,"cssClass":"pl-kos"}],[{"start":12,"end":14,"cssClass":"pl-k"},{"start":19,"end":20,"cssClass":"pl-c1"},{"start":21,"end":22,"cssClass":"pl-kos"}],[{"start":16,"end":21,"cssClass":"pl-k"},{"start":21,"end":22,"cssClass":"pl-kos"}],[{"start":12,"end":13,"cssClass":"pl-kos"}],[{"start":8,"end":9,"cssClass":"pl-kos"}],[{"start":4,"end":5,"cssClass":"pl-kos"}],[{"start":4,"end":5,"cssClass":"pl-kos"},{"start":6,"end":7,"cssClass":"pl-kos"},{"start":7,"end":12,"cssClass":"pl-en"},{"start":12,"end":13,"cssClass":"pl-kos"},{"start":13,"end":14,"cssClass":"pl-kos"},{"start":14,"end":15,"cssClass":"pl-kos"},{"start":21,"end":22,"cssClass":"pl-kos"}],[{"start":0,"end":1,"cssClass":"pl-kos"}]],"csv":null,"csvError":null,"dependabotInfo":{"showConfigurationBanner":false,"configFilePath":null,"networkDependabotPath":"/Lordwill1/RSA-Factoring-Challenge/network/updates","dismissConfigurationNoticePath":"/settings/dismiss-notice/dependabot_configuration_notice","configurationNoticeDismissed":null,"repoAlertsPath":"/Lordwill1/RSA-Factoring-Challenge/security/dependabot","repoSecurityAndAnalysisPath":"/Lordwill1/RSA-Factoring-Challenge/settings/security_analysis","repoOwnerIsOrg":false,"currentUserCanAdminRepo":false},"displayName":"main.rs","displayUrl":"https://github.com/Lordwill1/RSA-Factoring-Challenge/blob/master/rsa-rs/src/main.rs?raw=true","headerInfo":{"blobSize":"2.13 KB","deleteInfo":{"deleteTooltip":"You must be signed in to make or propose changes"},"editInfo":{"editTooltip":"You must be signed in to make or propose changes"},"ghDesktopPath":"https://desktop.github.com","gitLfsPath":null,"onBranch":true,"shortPath":"2499772","siteNavLoginPath":"/login?return_to=https%3A%2F%2Fgithub.com%2FLordwill1%2FRSA-Factoring-Challenge%2Fblob%2Fmaster%2Frsa-rs%2Fsrc%2Fmain.rs","isCSV":false,"isRichtext":false,"toc":null,"lineInfo":{"truncatedLoc":"75","truncatedSloc":"71"},"mode":"file"},"image":false,"isCodeownersFile":null,"isPlain":false,"isValidLegacyIssueTemplate":false,"issueTemplateHelpUrl":"https://docs.github.com/articles/about-issue-and-pull-request-templates","issueTemplate":null,"discussionTemplate":null,"language":"Rust","languageID":327,"large":false,"loggedIn":false,"newDiscussionPath":"/Lordwill1/RSA-Factoring-Challenge/discussions/new","newIssuePath":"/Lordwill1/RSA-Factoring-Challenge/issues/new","planSupportInfo":{"repoIsFork":null,"repoOwnedByCurrentUser":null,"requestFullPath":"/Lordwill1/RSA-Factoring-Challenge/blob/master/rsa-rs/src/main.rs","showFreeOrgGatedFeatureMessage":null,"showPlanSupportBanner":null,"upgradeDataAttributes":null,"upgradePath":null},"publishBannersInfo":{"dismissActionNoticePath":"/settings/dismiss-notice/publish_action_from_dockerfile","dismissStackNoticePath":"/settings/dismiss-notice/publish_stack_from_file","releasePath":"/Lordwill1/RSA-Factoring-Challenge/releases/new?marketplace=true","showPublishActionBanner":false,"showPublishStackBanner":false},"renderImageOrRaw":false,"richText":null,"renderedFileInfo":null,"shortPath":null,"tabSize":8,"topBannersInfo":{"overridingGlobalFundingFile":false,"globalPreferredFundingPath":null,"repoOwner":"Lordwill1","repoName":"RSA-Factoring-Challenge","showInvalidCitationWarning":false,"citationHelpUrl":"https://docs.github.com/en/github/creating-cloning-and-archiving-repositories/creating-a-repository-on-github/about-citation-files","showDependabotConfigurationBanner":false,"actionsOnboardingTip":null},"truncated":false,"viewable":true,"workflowRedirectUrl":null,"symbols":{"timedOut":false,"notAnalyzed":false,"symbols":[{"name":"main","kind":"function","identStart":140,"identEnd":144,"extentStart":137,"extentEnd":857,"fullyQualifiedName":"main","identUtf16":{"start":{"lineNumber":8,"utf16Col":3},"end":{"lineNumber":8,"utf16Col":7}},"extentUtf16":{"start":{"lineNumber":8,"utf16Col":0},"end":{"lineNumber":25,"utf16Col":1}}},{"name":"look_up","kind":"function","identStart":912,"identEnd":919,"extentStart":909,"extentEnd":1111,"fullyQualifiedName":"look_up","identUtf16":{"start":{"lineNumber":28,"utf16Col":3},"end":{"lineNumber":28,"utf16Col":10}},"extentUtf16":{"start":{"lineNumber":28,"utf16Col":0},"end":{"lineNumber":35,"utf16Col":1}}},{"name":"pollard_brent","kind":"function","identStart":1308,"identEnd":1321,"extentStart":1305,"extentEnd":2183,"fullyQualifiedName":"pollard_brent","identUtf16":{"start":{"lineNumber":41,"utf16Col":3},"end":{"lineNumber":41,"utf16Col":16}},"extentUtf16":{"start":{"lineNumber":41,"utf16Col":0},"end":{"lineNumber":74,"utf16Col":1}}},{"name":"M","kind":"constant","identStart":1509,"identEnd":1510,"extentStart":1503,"extentEnd":1521,"fullyQualifiedName":"M","identUtf16":{"start":{"lineNumber":45,"utf16Col":10},"end":{"lineNumber":45,"utf16Col":11}},"extentUtf16":{"start":{"lineNumber":45,"utf16Col":4},"end":{"lineNumber":45,"utf16Col":22}}}]}},"copilotInfo":null,"csrf_tokens":{"/Lordwill1/RSA-Factoring-Challenge/branches":{"post":"V-FETYTf-WODhE3MJu67ov0-UkUQdavK4RbW2PfJTR5Viaau6r8uIbIgpyWLHBAwzoPp-CGyaLF86_eYaUjfAA"},"/repos/preferences":{"post":"RJuQ17GRKDiaeegKX2qFolYGoursarrV2qyDN3G-D2FOWLmuMkx5syqs0M3pDO-WmkGXmSWUdjkDN_dApKtCqQ"}}},"title":"RSA-Factoring-Challenge/rsa-rs/src/main.rs at master · Lordwill1/RSA-Factoring-Challenge"}