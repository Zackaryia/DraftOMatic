use std::string;
//use cached::proc_macro::cached;
// MAXING = BLUE WIN
// MIN = RED WIN
use std::{fs::File, os::unix::raw::time_t};
use std::collections::HashMap;
use std::ops::Add;
use std::time::Instant;
use once_cell::sync::Lazy;
// use cached::Return;
use rayon::{prelude::*, vec};
use std::sync::RwLock;

struct evaluation {
	r_deep_eval: f32,
	positions_analyzed: u64,
	time_to_analyze: f32,
	optimal_path: Vec<u16>,
	imediate_next_moves: Vec<(f32, u16)>,
}

// Returns the evaluation of the position, the nodes analyzed, the 
pub fn eval_state(cru16s: Vec<u16>, depth: u16) -> (Vec<(f32, Vec<u16>)>, u64, f32) {
	

	let mut state = State {
		blue: vec![],
		red: vec![],
		r: 0_f32
	};


	for cru16 in cru16s {
		state.add_champ(cru16)
	}

	let start = Instant::now();

	let (imediate_best_picks, count) = par_analyze(state.clone(), depth, false);

	let time_to_eval = start.elapsed().as_secs_f32();

	(imediate_best_picks, count, time_to_eval)
}

fn par_analyze(state: State, max_depth: u16, use_alpha: bool) -> (Vec<(f32, Vec<u16>)>, u64) {
	let alpha_parallel = RwLock::new(std::f32::MIN);
	
	let count = RwLock::new(0_u64);

    if max_depth == 0 {
        let (a, b, c) = minimax(&CONTEXT.cru16_popular, state, 0, max_depth, std::f32::MIN, std::f32::MAX);
        return  (vec![(a, b)], c);
    }

	let best_picks = (state.champs_left(&CONTEXT.cru16_popular)).par_iter()
	.map(|champ| {
		let mut new_state = state.clone();
		new_state.add_champ(*champ);
		let (value, mut pick_path, count_i);
		if use_alpha {
			(value, pick_path, count_i) = minimax(&CONTEXT.cru16_popular, new_state, 1, max_depth, *alpha_parallel.read().unwrap(), std::f32::MAX);
		} else {
			(value, pick_path, count_i) = minimax(&CONTEXT.cru16_popular, new_state, 1, max_depth, std::f32::MIN, std::f32::MAX);
		}
		
		pick_path.push(*champ);

		let mut count = count.write().unwrap();
		*count += count_i;

		
		if use_alpha && *alpha_parallel.read().unwrap() < value {
			let mut alpha_parallel = alpha_parallel.write().unwrap();
			*alpha_parallel = value;
		}

		(value, pick_path)
	}).collect::<Vec<(f32, Vec<u16>)>>();

	let final_count = count.read().unwrap().add(0) as u64;

	return (best_picks, final_count);
}
// Will return the evaluation of the state
// Total positions analyzed
// The best next moves
// Each team dmg imbalance
// Champ WR
// Matchup WR
// Duos WR 



struct Context {
	// cids: Vec<u16>,
	cru16_popular: Vec<u16>,
	// cru16_dmg: HashMap<u16, [f32; 3]>,
	cru16_r: HashMap<u16, f32>,
	duo_matchups: HashMap<(u16, u16), f32>,
	vs_matchups: HashMap<(u16, u16), f32>,
	turn_is_maxing: [bool; 10]
}


static CONTEXT: Lazy<Context> = Lazy::new(|| {
	let start = Instant::now();
	dbg!("Loading Constants");
	let (cids, cru16_popular, cru16_dmg, cru16_r, duo_matchups, vs_matchups, turn_is_maxing) = load_constants();
	dbg!("Loaded", start.elapsed().as_secs_f32());

	Context { 
		// cids, cru16_dmg, 
		cru16_popular,
		cru16_r, duo_matchups, vs_matchups, turn_is_maxing
	}
});



fn main() {
    let x = State {
        blue: vec![8264],
        red: vec![16404],
        r: 0_f32,
    };

    dbg!(minimax(&CONTEXT.cru16_popular, x, 0, 2, std::f32::MIN, std::f32::MAX));
}

#[derive(Clone, Debug)]
struct State {
	blue: Vec<u16>, // A sorted list of champions for blue side
	red: Vec<u16>, // A sorted list of champions for red side
	//bans: Vector<u16> // A sorted list of champions for bans
	r: f32, // The evaluation of the state
}


impl State {
	fn is_maxing(&self) -> Option<bool> {
		if self.blue.len() + self.red.len() == 10 {
			return None;
		}

		Some(CONTEXT.turn_is_maxing[self.blue.len() + self.red.len()])
	}

	fn add_champ(&mut self, champ: u16) {
		if champ == 0 {
			return;
		}

		for blue_champ in &self.blue {
			if self.is_maxing().unwrap() {
				self.r += CONTEXT.duo_matchups.get(&(champ, *blue_champ)).unwrap_or(&0_f32);
			} else {
				self.r -= CONTEXT.vs_matchups.get(&(champ, *blue_champ)).unwrap_or(&0_f32);
			}
		}

		for red_champ in &self.red {
			if self.is_maxing().unwrap() {
				self.r += CONTEXT.vs_matchups.get(&(champ, *red_champ)).unwrap_or(&0_f32);
			} else {
				self.r -= CONTEXT.duo_matchups.get(&(champ, *red_champ)).unwrap_or(&0_f32);
			}
		}

		if self.is_maxing().unwrap() {
			self.r += CONTEXT.cru16_r.get(&champ).unwrap_or(&0_f32);
			self.blue.push(champ);
		} else {
			self.r -= CONTEXT.cru16_r.get(&champ).unwrap_or(&0_f32);
			self.red.push(champ);
		}
	}

	fn champs_left(&self, pickable_champs: &[u16]) -> Vec<u16> {
		//Gets all champs that blue side can pick left
		//removes the champs that have already been picked (last 8 bits)
		//removes all roles that have already been picked (first 8 bits)
		//Uses bitmasks and .iter().filter(...).collect() 
		let team_champs = if self.is_maxing().unwrap() { self.blue.clone() } else { self.red.clone() };

		pickable_champs.iter()
			.filter(
				|a| !(( // Remove champs with the same role
					(team_champs.clone()).into_iter().any(|x| (x & 0b1111u16 << 12 == *a & 0b1111u16 << 12))
				) || *( // Remove champs with the same champion ID
					&self.red.clone().into_iter().any(|x| (x & !(0b1111u16 << 12) == *a & !(0b1111u16 << 12)))
				) || *( // Remove champs with the same champion ID
					&self.blue.clone().into_iter().any(|x| (x & !(0b1111u16 << 12) == *a & !(0b1111u16 << 12)))
				))
			)
			.map(|x| *x)
			.collect::<Vec<_>>()
	}
}



fn minimax(pickable_champs: &[u16], state: State, depth: u16, max_depth: u16, mut alpha: f32, mut beta: f32) -> (f32, Vec<u16>, u64) {
	let mut count: u64 = 1;

	if depth >= max_depth || state.is_maxing() == None {
		return (state.r, vec![], count)
	}

	let mut best_pick_path: Vec<u16> = vec![];

	if state.is_maxing().unwrap() {
		let mut best_val = std::f32::MIN;
		for champ in state.champs_left(pickable_champs) {
			let mut new_state = state.clone();
			new_state.add_champ(champ);
			let (value, mut pick_path, count_i) = minimax(pickable_champs, new_state, depth+1, max_depth, alpha, beta);
			count += count_i;
			if value > best_val {
				best_val = value;
				pick_path.push(champ); 
				best_pick_path = pick_path;
			}

			if best_val > alpha {
				alpha = best_val;
			}

			if beta <= alpha {
				break
			}
		}  
		(best_val, best_pick_path, count)

	} else {
		let mut best_val = std::f32::MAX;
		for champ in state.champs_left(pickable_champs) {
			let mut new_state = state.clone();
			new_state.add_champ(champ);
			let (value, mut pick_path, count_i) = minimax(pickable_champs, new_state, depth+1, max_depth, alpha, beta);
			count += count_i;

			if value < best_val {
				best_val = value;
				pick_path.push(champ); 
				best_pick_path = pick_path;
			}

			if best_val < beta {
				beta = best_val;
			}

			if beta <= alpha {
				break
			}   
		}

		(best_val, best_pick_path, count)
	}
}









fn convert_lol_cid_role_to_cru16(cid: u16, role: u16) -> u16 {
	// This function converts the league of legends role and CID to a custom u16 to allow for more efficnet
	// data processing
	// The first 4 bits are the role ID and the next 12 are the champion's index in the champion_ids.
	role << 12 | cid
}




fn load_constants() -> (Vec<u16>, Vec<u16>, HashMap<u16, [f32; 3]>, HashMap<u16, f32>, HashMap<(u16, u16), f32>, HashMap<(u16, u16), f32>, [bool; 10]) {
	let file = File::open("/home/z/Desktop/draftomatic/champ_ids.json")
		.expect("file should open read only");
	let mut champ_ids: Vec<u16> = serde_json::from_reader(file)
		.expect("file should be proper JSON and hold an array of unsigned integers");

	champ_ids.sort();



	let file = File::open("/home/z/Desktop/draftomatic/played_champ_roles.json")
		.expect("file should open read only");
	let cru16_popular_json: Vec<String> = serde_json::from_reader(file)
		.expect("file should be proper JSON");

	let mut cru16_popular: Vec<u16> = vec![];
	for key in cru16_popular_json {
		let split = key.split("_").collect::<Vec<&str>>();
		let cru16 = convert_lol_cid_role_to_cru16(split[0].parse::<u16>().unwrap(), split[1].parse::<u16>().unwrap());

		cru16_popular.push(cru16);
	}
	


	let file = File::open("/home/z/Desktop/draftomatic/champion_dmg_data.json")
		.expect("file should open read only");
	let champion_dmg_data_json: HashMap<String, [f32; 3]> = serde_json::from_reader(file)
		.expect("file should be proper JSON");

	let mut champion_dmg_data: HashMap<u16, [f32; 3]> = HashMap::new();
	for (key, value) in champion_dmg_data_json {
		let split = key.split("_").collect::<Vec<&str>>();
		let cru16 = convert_lol_cid_role_to_cru16(split[1].parse::<u16>().unwrap(), split[2].parse::<u16>().unwrap());

		champion_dmg_data.insert(cru16, value);
	}
	

	let file = File::open("/home/z/Desktop/draftomatic/champion_r_data.json")
		.expect("file should open read only");
	let champion_r_data_json: HashMap<String, f32> = serde_json::from_reader(file)
		.expect("file should be proper JSON");

	let mut champion_r_data: HashMap<u16, f32> = HashMap::new();
	for (key, value) in champion_r_data_json {
		let split = key.split("_").collect::<Vec<&str>>();
		let cru16 = convert_lol_cid_role_to_cru16(split[1].parse::<u16>().unwrap(), split[2].parse::<u16>().unwrap());

		champion_r_data.insert(cru16, value);
	}
	


	let file = File::open("/home/z/Desktop/draftomatic/matchup_data.json")
		.expect("file should open read only");
	let matchup_data_json: HashMap<String, f32> = serde_json::from_reader(file)
		.expect("file should be proper JSON");
	
	
	let mut duo_matchups: HashMap<(u16, u16), f32> = HashMap::new();
	let mut vs_matchups: HashMap<(u16, u16), f32> = HashMap::new();

	for (key, value) in matchup_data_json {
		let split = key.split("_").collect::<Vec<&str>>();
		let cru16 = convert_lol_cid_role_to_cru16(split[1].parse::<u16>().unwrap(), split[2].parse::<u16>().unwrap());
		let cru16_2 = convert_lol_cid_role_to_cru16(split[3].parse::<u16>().unwrap(), split[4].parse::<u16>().unwrap());

		if split[0] == "team" {
			duo_matchups.insert((cru16, cru16_2), value);
		} else {
			vs_matchups.insert((cru16, cru16_2), value);
		}
	}



	cru16_popular.sort_by(|a, b| champion_r_data[a].partial_cmp(&champion_r_data[b]).unwrap());


	(
		champ_ids, 
		cru16_popular,
		champion_dmg_data, 
		champion_r_data, 
		duo_matchups, 
		vs_matchups, 
		[true, false, false, true, true, false, false, true, true, false]
	)
}


