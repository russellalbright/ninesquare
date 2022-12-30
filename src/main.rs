use itertools::Itertools;

static mut _PIECES: [[i32; 4]; 9] = [
   0 [1, 8, 6, 3],
   1 [4, 9, 8, 3],
   2 [2, 9, 7, 4],
   3 [2, 6, 9, 3],
   4 [1, 7, 8, 4],
   5 [2, 6, 9, 3],
   6 [2, 7, 9, 3],
   7 [1, 6, 8, 4],
   8 [2, 7, 9, 4],
];
// static mut _PIECES: [[i32; 4]; 9] = [
//     [3, 5, 8, 4],
//     [3, 5, 8, 2],
//     [9, 3, 5, 6],
//     [3, 4, 1, 2],
//     [6, 5, 7, 1],
//     [8, 5, 4, 1],
//     [1, 9, 7, 4],
//     [7, 5, 6, 9],
//     [3, 2, 6, 7],
// ];
fn main() {
    let _board = [0, 1, 2, 3, 4, 5, 6, 7, 8];
    //let _board = [0, 1, 2];
    for attempt in _board.iter().permutations(_board.len()).unique() {
        // let attempt = vec![8i32, 4, 0, 2, 6, 3, 1, 5, 7];
        let attempt0 = vec![0i32, 1, 2, 3, 4, 5, 6, 7, 8];
        let mut attempt3: Vec<&i32> = Vec::new();
        let mut i = 0;
        for my_int in attempt.iter() {
            attempt3.push(&attempt0[i]);
            i += 1;
            // attempt2.push(&my_int);
        }
        i = 0;
        for my_int in attempt.iter() {
            attempt3[**my_int as usize] = &attempt0[i];
            i += 1;
            // attempt2.push(&my_int);
        }
        //println!("{:?}", attempt3);


        if test_answer(&attempt3) == true {
            println!("{:?}", attempt);
            println!("FOUND IT");
            return;
        }
    }

    println!("no answer!");
}

fn test_answer(attempt: &Vec<&i32>) -> bool {
    //check 4 surrounding the middle piece
    for _i in 0..1 {
        if check_1_4(attempt) == false {
            continue;
        }
        // println!("{:?}  a ", attempt);

        if check_3_4(attempt) == false {
            continue;
        }
        // println!("{:?}  b", attempt);

        if check_4_5(attempt) == false {
            continue;
        }
        // println!("{:?}  c", attempt);

        if check_4_7(attempt) == false {
            continue;
        }
        // println!("{:?}  d", attempt);

        if check_0(attempt) == false {
            continue;
        }

        println!("{:?} check 0", attempt);

        if check_2(attempt) == false {
            continue;
        }

        println!("{:?} check 2", attempt);

        if check_6(attempt) == false {
            continue;
        }
        println!("{:?}  check 6", attempt);

        if check_8(attempt) == false {
            continue;
        }
        println!("{:?}  check 8", attempt);
        return true;

        //rotate(*attempt[4] as usize);
    }
    return false;
}

fn rotate(i: usize) {
    unsafe {
        let temp = _PIECES[i][0];
        _PIECES[i][0] = _PIECES[i][1];
        _PIECES[i][1] = _PIECES[i][2];
        _PIECES[i][2] = _PIECES[i][3];
        _PIECES[i][3] = temp;
    }
}

fn check_1_4(attempt: &Vec<&i32>) -> bool {
    for _i in 0..4 {
        unsafe {
            // println!("{:?} squares {:?}", *attempt[1], *attempt[4]);
            // println!(
            //     "{:?} and {:?}",
            //     _PIECES[*attempt[1] as usize][2], _PIECES[*attempt[4] as usize][0]
            // );

            if _PIECES[*attempt[1] as usize][2] + _PIECES[*attempt[4] as usize][0] == 10 {
                return true;
            }
        }
        rotate(*attempt[1] as usize);
    }
    return false;
}
fn check_3_4(attempt: &Vec<&i32>) -> bool {
    unsafe {
        for _i in 0..4 {
            if _PIECES[*attempt[3] as usize][1] + _PIECES[*attempt[4] as usize][3] == 10 {
                return true;
            }
            rotate(*attempt[3] as usize);
        }
    }
    return false;
}
fn check_4_5(attempt: &Vec<&i32>) -> bool {
    unsafe {
        for _i in 0..4 {
            if _PIECES[*attempt[4] as usize][1] + _PIECES[*attempt[5] as usize][3] == 10 {
                return true;
            }
            rotate(*attempt[5] as usize);
        }
    }
    return false;
}
fn check_4_7(attempt: &Vec<&i32>) -> bool {
    unsafe {
        for _i in 0..4 {
            if _PIECES[*attempt[4] as usize][2] + _PIECES[*attempt[7] as usize][0] == 10 {
                return true;
            }
            rotate(*attempt[7] as usize);
        }
    }
    return false;
}

fn check_0(attempt: &Vec<&i32>) -> bool {
    for _i in 0..4 {
        unsafe {
            println!("{:?} squares {:?}", *attempt[0], *attempt[1]);
            println!(
                "{:?} and {:?}",
                _PIECES[*attempt[0] as usize][1], _PIECES[*attempt[1] as usize][3]
            );

            println!("{:?} squares {:?}", *attempt[0], *attempt[3]);
            println!(
                "{:?} and {:?}",
                _PIECES[*attempt[0] as usize][2], _PIECES[*attempt[3] as usize][0]
            );

            if _PIECES[*attempt[0] as usize][1] + _PIECES[*attempt[1] as usize][3] == 10
                && _PIECES[*attempt[0] as usize][2] + _PIECES[*attempt[3] as usize][0] == 10
            {
                return true;
            }
            rotate(*attempt[0] as usize);
        }
    }
    return false;
}
fn check_2(attempt: &Vec<&i32>) -> bool {
    for _i in 0..4 {
        unsafe {
            if _PIECES[*attempt[1] as usize][1] + _PIECES[*attempt[2] as usize][3] == 10
                && _PIECES[*attempt[2] as usize][2] + _PIECES[*attempt[5] as usize][0] == 10
            {
                return true;
            }
            rotate(*attempt[2] as usize);
        }
    }
    return false;
}
fn check_6(attempt: &Vec<&i32>) -> bool {
    for _i in 0..4 {
        unsafe {
            if _PIECES[*attempt[3] as usize][2] + _PIECES[*attempt[6] as usize][0] == 10
                && _PIECES[*attempt[6] as usize][1] + _PIECES[*attempt[7] as usize][3] == 10
            {
                return true;
            }
            rotate(*attempt[6] as usize);
        }
    }
    return false;
}
fn check_8(attempt: &Vec<&i32>) -> bool {
    for _i in 0..4 {
        unsafe {
            if _PIECES[*attempt[5] as usize][2] + _PIECES[*attempt[8] as usize][0] == 10
                && _PIECES[*attempt[7] as usize][1] + _PIECES[*attempt[8] as usize][3] == 10
            {
                return true;
            }
            rotate(*attempt[8] as usize);
        }
    }
    return false;
}
