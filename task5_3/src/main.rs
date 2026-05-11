#[derive(Debug)]
struct Player{
    nickname: String,
	level: u32,
	score: f64,
	is_online: bool,
}


fn main() {
    
let mut player1 = build_player(String::from("Ivan"));
println!("{:#?}", player1);

let player2 = Player{
	nickname: String::from("Nadya"),
	..player1
};

println!("{:#?}", player2);

player1.is_online = false;
println!("{:#?}", player1);

}

fn build_player(nickname:String)->Player{
    Player { nickname, level: 1, score: 0.0, is_online: true }
}
