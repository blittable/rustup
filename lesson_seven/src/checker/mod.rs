use modulo::Mod;

pub fn prime_checker(input: &Vec<u32>) -> Vec<(u32, Option<u32>)> {
    let mut answer: Vec<(u32, Option<u32>)> = Vec::new();

    for item in input.iter() {
        let x = *item;
        if x.modulo(2) == 0 {
            answer.push((x, Some(2)));
        } else {
            let mid = (x as f64).sqrt().floor() as u32;
            let mut div = 3;
            let mut notfound = true;
            loop {
                if x.modulo(div) == 0 {
                    notfound = false;
                    break;
                }
                div += 2;
                if div > mid {
                    break;
                }
            }
            if notfound {
                answer.push((x, None));
            } else {
                answer.push((x, Some(div)))
            }
        }
    }
    return answer;
}