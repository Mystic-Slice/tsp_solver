use crate::utility::map::Map;

pub fn two_opt(route: &Vec<i32>, max_iterations: i32, map: &Map) -> Vec<i32> {
    let count = route.len();

    let mut route_mut = route.clone();

    'start_again:
    for _current_iterations in 0..max_iterations {
        for i in 0..count {
            for j in (i+2)..count {
                let ni = i+1;
                let nj = j+1;

                let icity = map.cities[(route_mut[i] - 1) as usize];
                let jcity = map.cities[(route_mut[j] - 1) as usize];
                let nicity = map.cities[(route_mut[ni % count] - 1) as usize];
                let njcity = map.cities[(route_mut[nj % count] - 1) as usize];

                let original_dist = icity.distance(nicity) + jcity.distance(njcity);
                let changed_dist = icity.distance(jcity) + nicity.distance(njcity);

                if changed_dist < original_dist {
                    let swap_count = ((j as f32 - ni as f32)/2.0 + 0.5) as usize;
                    for offset in 0..swap_count {
                        route_mut.swap((ni+offset)%count, (j-offset)%count);
                    }
                    continue 'start_again;
                }
            }
        }
        break;
    }
    return route_mut
}