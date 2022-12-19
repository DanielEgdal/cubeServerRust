use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::HashMap;

// type Cube = (u64, u64);

// #[inline(always)]
pub fn swap5(k: u64, pos1: u8, pos2: u8) -> u64{
    let set1 =  (k >> pos1) & 31;
    let set2 = (k >> pos2) & 31;
    let mut xor = set1 ^ set2;
    xor = (xor << pos1) | (xor << pos2);
    let ret_val = k ^ xor;
    return ret_val;
}

pub fn twist_corner(k:u64) -> u64{
    let u = k & 24;
    let o = (u+8+((u&16)>>1))&24;
    return o + (k&7)
}

pub fn twist_corner_c(k:u64) -> u64{

    let u = k & 24;
    let t = u + 24;
    let o = (t - ((t & 16)>>1)) & 24;
    return o + (k&7)
}

// (mut c, mut e): Cube
pub fn u(mut c:u64,mut e: u64)->(u64,u64){
    let block1 = (c >> 20) & 31;
    let block2 = (c >> 25) & 31;
    let block3 = (c >> 30) & 31;
    let block4 = (c >> 35) & 31;

    c = c & 34084861509631;
    c = c ^ (( block1 << 25) | ( block2 << 35) |( block3 << 20 ) |( block4 << 30));

    let block1 = (e >> 40) & 31;
    let block2 = (e >> 45) & 31;
    let block3 = (e >> 50) & 31;
    let block4 = (e >> 55) & 31;
    e = e & 1099511627775;
    e = e ^ (( block1 << 45) | ( block2 << 55) |( block3 << 40) |( block4 << 50));

    return (c,e);
}

pub fn up(mut c:u64, mut e: u64)->(u64,u64){
    let block1 = (c >> 20) & 31;
    let block2 = (c >> 25) & 31;
    let block3 = (c >> 30) & 31;
    let block4 = (c >> 35) & 31;

    c = c & 34084861509631;
    c = c ^  ((block1 << 30) | (block2 << 20) |(block3 << 35 ) |(block4 << 25));

    let block1 = (e >> 40) & 31;
    let block2 = (e >> 45) & 31;
    let block3 = (e >> 50) & 31;
    let block4 = (e >> 55) & 31;
    e = e & 1099511627775;
    e = e ^ ((block1 << 50) | (block2 << 40) |(block3 << 55) |(block4 << 45));

    return (c,e);
}

pub fn u2(c:u64, e: u64)->(u64,u64){
    // let nc =  swap5(swap5(swap5(c,20,25),30,35),20,35);
    // let ne =  swap5(swap5(swap5(e,40,45),55,50),40,55);
    let mut nc = swap5(c,20,35);
    nc = swap5(nc,30,25);

    let mut ne = swap5(e,40,55);
    ne = swap5(ne,45,50);

    return (nc,ne);
}

pub fn d(mut c:u64, mut e: u64)->(u64,u64){
    let block1 = (c) & 31;
    let block2 = (c >> 5) & 31;
    let block3 = (c >> 10) & 31;
    let block4 = (c >> 15) & 31;

    c = c & 35184371040256;
    c = c ^ ((block1 << 5) | (block2 << 15) |(block3) |(block4 << 10));

    let block1 = (e) & 31;
    let block2 = (e >> 5) & 31;
    let block3 = (e >> 10) & 31;
    let block4 = (e >> 15) & 31;
    e = e & 1152921504605798400;
    e = e ^ ((block1 << 5) | (block2 << 15) |(block3) |(block4 << 10));

    return (c,e);
}

pub fn dp(mut c:u64,mut e: u64)->(u64,u64){
    let block1 = (c) & 31;
    let block2 = (c >> 5) & 31;
    let block3 = (c >> 10) & 31;
    let block4 = (c >> 15) & 31;

    c = c & 35184371040256;
    c = c ^ ((block1 << 10) | (block2) |(block3 << 15) |(block4 << 5));

    let block1 = (e) & 31;
    let block2 = (e >> 5) & 31;
    let block3 = (e >> 10) & 31;
    let block4 = (e >> 15) & 31;
    e = e & 1152921504605798400;
    e = e ^ ((block1 << 10) | (block2) |(block3 << 15) |(block4 << 5));

    return (c,e);
}

pub fn d2(c:u64, e: u64)->(u64,u64){
    // let nc =  swap5(swap5(swap5(c,20,25),30,35),20,35);
    // let ne =  swap5(swap5(swap5(e,40,45),55,50),40,55);
    let mut nc = swap5(c,0,15);
    nc = swap5(nc,5,10);

    let mut ne = swap5(e,0,15);
    ne = swap5(ne,5,10);

    return (nc,ne);
}

pub fn f2(c:u64, e: u64)->(u64,u64){
    // let nc =  swap5(swap5(swap5(c,20,25),30,35),20,35);
    // let ne =  swap5(swap5(swap5(e,40,45),55,50),40,55);
    let mut nc = swap5(c,20,15);
    nc = swap5(nc,25,10);

    let mut ne = swap5(e,15,40);
    ne = swap5(ne,30,35);

    return (nc,ne);
}

pub fn l2(c:u64, e: u64)->(u64,u64){
    // let nc =  swap5(swap5(swap5(c,20,25),30,35),20,35);
    // let ne =  swap5(swap5(swap5(e,40,45),55,50),40,55);
    let mut nc = swap5(c,35,15);
    nc = swap5(nc,25,5);

    let mut ne = swap5(e,45,5);
    ne = swap5(ne,35,20);

    return (nc,ne);
}

pub fn b2(c:u64, e: u64)->(u64,u64){
    // let nc =  swap5(swap5(swap5(c,20,25),30,35),20,35);
    // let ne =  swap5(swap5(swap5(e,40,45),55,50),40,55);
    let mut nc = swap5(c,30,5);
    nc = swap5(nc,35,0);

    let mut ne = swap5(e,25,20);
    ne = swap5(ne,55,0);

    return (nc,ne);
}

pub fn r2(c:u64, e: u64)->(u64,u64){
    // let nc =  swap5(swap5(swap5(c,20,25),30,35),20,35);
    // let ne =  swap5(swap5(swap5(e,40,45),55,50),40,55);
    let mut nc = swap5(c,10,30);
    nc = swap5(nc,20,0);

    let mut ne = swap5(e,10,50);
    ne = swap5(ne,30,25);

    return (nc,ne);
}


pub fn f(mut c:u64,mut e: u64)->(u64,u64){
    let block1 = twist_corner((c >> 10) & 31);
    let block2 = twist_corner_c((c >> 15) & 31);
    let block3 = twist_corner_c((c >> 20) & 31);
    let block4 = twist_corner((c >> 25) & 31);
    c = c & 35183298348031;
    c = c ^ ((block1 << 15) | (block2 << 25) |(block3 << 10) |(block4 << 20));

    let block1 = ((e >> 15) & 31)^16;
    let block2 = ((e >> 30) & 31) ^16;
    let block3 = ((e >> 35) & 31) ^16;
    let block4 = ((e >> 40) & 31) ^16;
    e = e & 1152886321307484159;
    e = e ^ ((block1 << 35) | (block2 << 15) |(block3 << 40) |(block4 << 30));
    return (c,e);
}

pub fn fp(mut c:u64,mut e: u64)->(u64,u64){
    let block1 = twist_corner((c >> 10) & 31);
    let block2 = twist_corner_c((c >> 15) & 31);
    let block3 = twist_corner_c((c >> 20) & 31);
    let block4 = twist_corner((c >> 25) & 31);
    c = c & 35183298348031;
    c = c ^ ((block1 << 20) | (block2 << 10) |(block3 << 25) |(block4 << 15));

    let block1 = ((e >> 15) & 31)^16;
    let block2 = ((e >> 30) & 31) ^16;
    let block3 = ((e >> 35) & 31) ^16;
    let block4 = ((e >> 40) & 31) ^16;
    e = e & 1152886321307484159;
    e = e ^ ((block1 << 30) | (block2 << 40) |(block3 << 15) |(block4 << 35));
    return (c,e);

}

pub fn l(mut c:u64,mut e: u64)->(u64,u64){
    let block1 = twist_corner_c((c >> 5) & 31);
    let block2 = twist_corner((c >> 15) & 31);
    let block3 = twist_corner_c((c >> 25) & 31);
    let block4 = twist_corner((c >>35) & 31);
    c = c & 34118178995231;
    c = c ^ ((block1 << 35) | (block2 << 5) |(block3 << 15) |(block4 << 25));

    let block1 = (e >> 5) & 31;
    let block2 = (e >> 20) & 31;
    let block3 = (e >> 35) & 31;
    let block4 = (e >> 45) & 31;
    e = e & 1151829723887696927;
    e = e ^ ((block1 << 20) | (block2 << 45) |(block3 << 5) |(block4 << 35));


    return (c,e);
}

pub fn lp(mut c:u64,mut e: u64)->(u64,u64){
    let block1 = twist_corner_c((c >> 5) & 31);
    let block2 = twist_corner((c >> 15) & 31);
    let block3 = twist_corner_c((c >> 25) & 31);
    let block4 = twist_corner((c >>35) & 31);
    c = c & 34118178995231;
    c = c ^ ((block1 << 15) | (block2 << 25) |(block3 << 35) |(block4 << 5));

    let block1 = (e >> 5) & 31;
    let block2 = (e >> 20) & 31;
    let block3 = (e >> 35) & 31;
    let block4 = (e >> 45) & 31;
    e = e & 1151829723887696927;
    e = e ^ ((block1 << 35) | (block2 << 5) |(block3 << 45) |(block4 << 20));

    return (c,e);
}

pub fn r(mut c:u64,mut e: u64)->(u64,u64){

    let block1 = twist_corner_c((c >> 10) & 31);
    let block2 = twist_corner((c >> 20) & 31);
    let block3 = twist_corner_c((c >> 30) & 31);
    let block4 = twist_corner((c) & 31);
    c =  c & 35151053554656;
    c = c ^ ((block1 << 20) | (block2 << 30) |(block3) |(block4 << 10));

    let block1 = (e >> 10) & 31;
    let block2 = (e >> 25) & 31;
    let block3 = (e >> 30) & 31;
    let block4 = (e >> 50) & 31;
    e = e & 1118018573168509951;
    e = e ^ ((block1 << 30) | (block2 << 10) |(block3 << 50) |(block4 << 25));

    return (c,e);
}

pub fn rp(mut c:u64,mut e: u64)->(u64,u64){

    let block1 = twist_corner_c((c >> 10) & 31);
    let block2 = twist_corner((c >> 20) & 31);
    let block3 = twist_corner_c((c >> 30) & 31);
    let block4 = twist_corner((c) & 31);
    c =  c & 35151053554656;
    c = c ^ ((block1) | (block2 << 10) |(block3 << 20) |(block4 << 30));

    let block1 = (e >> 10) & 31;
    let block2 = (e >> 25) & 31;
    let block3 = (e >> 30) & 31;
    let block4 = (e >> 50) & 31;
    e = e & 1118018573168509951;
    e = e ^ ((block1 << 25) | (block2 << 50) |(block3 << 10) |(block4 << 30));

    return (c,e);
}

pub fn b(mut c:u64,mut e: u64)->(u64,u64){
    let block1 = twist_corner_c((c) & 31);
    let block2 = twist_corner((c >> 5) & 31);
    let block3 = twist_corner((c >> 30) & 31);
    let block4 = twist_corner_c((c >> 35) & 31);
    c = c & 34085934201856;
    c = c ^ ((block1 << 30) | (block2) |(block3 << 35) |(block4 << 5));

    let block1 = ((e) & 31)^16;
    let block2 = ((e >> 20) & 31) ^16;
    let block3 = ((e >> 25) & 31) ^16;
    let block4 = ((e >> 55) & 31) ^16;
    e = e & 36028795946270688;
    e = e ^ ((block1 << 25) | (block2) |(block3 << 55) |(block4 << 20));

    return (c,e);
}

pub fn bp(mut c:u64,mut e: u64)->(u64,u64){
    let block1 = twist_corner_c((c) & 31);
    let block2 = twist_corner((c >> 5) & 31);
    let block3 = twist_corner((c >> 30) & 31);
    let block4 = twist_corner_c((c >> 35) & 31);
    c = c & 34085934201856;
    c = c ^ ((block1 << 5) | (block2 << 35) |(block3 ) |(block4 << 30));

    let block1 = ((e) & 31)^16;
    let block2 = ((e >> 20) & 31) ^16;
    let block3 = ((e >> 25) & 31) ^16;
    let block4 = ((e >> 55) & 31) ^16;
    e = e & 36028795946270688;
    e = e ^ ((block1 << 20) | (block2 << 55) |(block3) |(block4 << 25));

    return (c,e);
}

pub fn perform_move(movee: u8, c: u64, e: u64) -> (u64, u64) {
    let (nc, ne) = match movee{
        1 => r(c,e),
        2 => l(c,e),
        3 => u(c,e),
        4 => d(c,e),
        5 => f(c,e),
        6 => b(c,e),
        11 => r2(c,e),
        12 => l2(c,e),
        13 => u2(c,e),
        14 => d2(c,e),
        15 => f2(c,e),
        16 => b2(c,e),
        21 => rp(c,e),
        22 => lp(c,e),
        23 => up(c,e),
        24 => dp(c,e),
        25 => fp(c,e),
        26 => bp(c,e),
        0 => (c,e),
        _ => unreachable!()
    };
    return (nc,ne)
}

pub fn do_scramble(scramble:String, mut c:u64, mut e:u64) -> (u64, u64,Vec<u8>){
// pub fn do_scramble(c:u64,e:u64)->(u64,u64){
    // let mut move_int =HashMap::new();
    
    let split_scramble: Vec<_> = scramble.split_ascii_whitespace().map(|f|
        match f {
            "R" => 1,
            "L" => 2,
            "U" => 3,
            "D" => 4,
            "F" => 5,
            "B" => 6,
            "R2" => 11,
            "L2" => 12,
            "U2" => 13,
            "D2" => 14,
            "F2" => 15,
            "B2" => 16,
            "R'" => 21,
            "L'" => 22,
            "U'" => 23,
            "D'" => 24,
            "F'" => 25,
            "B'" => 26,
            _ => unreachable!()
        }).collect();

    // println!("{:?}",split_scramble);
    for movei in &split_scramble{
        (c, e) = perform_move(*movei, c, e);
        // println!("{:?} {:?} {:?}",movei,c,e)
    }
    return (c,e,split_scramble);
}

pub fn check_eo(e: u64) -> (bool, u64){
    let eo_state = e & 595056260442243600;
    (eo_state == 0, eo_state)
}

pub fn solve_eo(c:u64,e:u64)->[u8;7]{
    let moves:Vec<u8> = vec![1, 2, 3, 4, 5, 6, 11, 12, 13, 14, 15, 16, 21, 22, 23, 24, 25, 26];
    let mut q:VecDeque<(u64, u64, [u8;7],u8)> = VecDeque::new();
    let mut visited: HashSet <u64> = HashSet::new();
    // Missing check for if EO is already solved before starting
    q.push_back((c,e,[0;7],0));
    while let Some((nc, ne, nlist,depth)) = q.pop_front() {
        
        for movee in &moves{
            let (nnc, nne )= perform_move(*movee, nc, ne);
            let eo_state = check_eo(nne);
            let mut nnlist = nlist.to_owned();
            nnlist[depth as usize] = *movee;
            if eo_state.0{
                // println!("triggered {}",eo_state.0);
                return nnlist
            }
            else{
                if !visited.contains(&eo_state.1){
                    q.push_back((nnc,nne,nnlist,depth+1));
                    visited.insert(eo_state.1);
                }
            }

        }
    }
    // return vec![];
    unreachable!()

}

pub fn gen_eo_to_dr_prune()->HashMap<(u64,u64),[u8;8]>{
    let e:u64 = 532021248000; 
    let c: u64 = 248276819175;
    let moves:Vec<u8> = vec![1, 2, 3, 4, 11, 12, 13, 14, 15, 16, 21, 22, 23, 24];
    let mut q:VecDeque<(u64, u64, [u8;8],u8)> = VecDeque::new();
    let mut overview = HashMap::new();
    q.push_back((c,e,[0;8],0));
    let mut continuee = true;
    while !q.is_empty() && continuee{
        let (nc, ne, nlist,depth) = q.pop_front().expect("while loop should never trigger an error");
        for movee in &moves{
            let (nnc, nne )= perform_move(*movee, nc, ne);
            let mut nnlist = nlist.to_owned();
            nnlist[depth as usize] = *movee;
            if depth+1 <= 7{
                if !overview.contains_key(&(nnc,nne)){
                    overview.insert((nnc,nne),nnlist.clone());
                    q.push_back((nnc,nne,nnlist.clone(),depth+1));
                }
            }
            else{
                continuee = false;
            }
        }
    }
    overview
}

pub fn mk_inv(sol: &[u8;8])->Vec<u8>{
    let mut reverse_sol = Vec::new();
    for movee in sol.iter().rev(){
        if *movee >0{
            if *movee < 10{
                reverse_sol.push(*movee + 20);
            }
            else if *movee > 20{
                reverse_sol.push(*movee - 20);
            }
            else{
                reverse_sol.push(*movee);
            }
        }
    }
    reverse_sol

}

pub fn solve_dr(scramble: Vec<u8>,mut eo_sol: Vec<u8>,prune:&HashMap<(u64,u64),[u8;8]>)->Vec<u8>{
    let mut e:u64 = 532021248000; 
    let mut c:u64 = 248276819175;
    for movee in &scramble{
        (c, e )= perform_move(*movee, c, e);
    }
    for movee in &eo_sol{
        (c, e )= perform_move(*movee, c, e);
    }
    let moves:Vec<u8> = vec![1, 2, 3, 4, 11, 12, 13, 14, 15, 16, 21, 22, 23, 24];
    let mut q:VecDeque<(u64, u64, Vec<u8>)> = VecDeque::new();

    let mut visited: HashSet <u64> = HashSet::new();
    // Missing check for if EO is already solved before starting
    q.push_back((c,e,Vec::new()));
    while let Some((nc, ne, nlist)) = q.pop_front() {
        
        for movee in &moves{
            let (nnc, nne )= perform_move(*movee, nc, ne);
            let eo_state = check_eo(nne);
            let mut nnlist = nlist.to_owned();
            nnlist.push(*movee);
            if prune.contains_key(&(nnc,nne)){
                // println!("triggered {}",eo_state.0);
                let solution = prune.get(&(nnc,nne)).expect("Already checked that this value is in the hashmap");
                nnlist.extend(&mk_inv(solution));
                eo_sol.extend(nnlist);
                return eo_sol;

            }
            else{
                q.push_back((nnc,nne,nnlist));
                visited.insert(eo_state.1);
                }
            }

    }
    unreachable!()
}
    



pub fn solve_eo_from_scrm(scram:String,prune:&HashMap<(u64, u64), [u8;8]>)->String{
    let startc: u64 = 247132686368;
    let starte: u64 = 407901468851537952;
    let (c,e,split_scramble) = do_scramble(scram,startc,starte);
    let eo_sol = solve_eo(c,e); 
    // let prune = gen_eo_to_dr_prune();
    let dr_sol = solve_dr(split_scramble, eo_sol.to_vec(), prune);
    let mut stuff_str = String::new();
    for imove in &dr_sol{
        stuff_str.push_str(match imove {
            1 => "R ", 
            2 => "L ", 
            3 => "U ", 
            4 => "D ", 
            5 => "F ", 
            6 => "B ", 
            11 => "R2 ", 
            12 => "L2 ", 
            13 => "U2 ", 
            14 => "D2 ", 
            15 => "F2 ", 
            16 => "B2 ", 
            21 => "R' ", 
            22 => "L' ", 
            23 => "U' ", 
            24 => "D' ", 
            25 => "F' ", 
            26 => "B' ", 
                _ => unreachable!()
            })
    }
    // println!("{:?}",stuff_str);
    
    return stuff_str;
    // println!("{:?}",eo_sol);
}
