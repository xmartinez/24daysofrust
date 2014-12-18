extern crate redis;

use redis::{Client, Commands, Connection, RedisResult};
use std::collections::HashSet;
use std::rand;

type UserId = u64;

fn add_friend(conn: &Connection, my_id: UserId, their_id: UserId) -> RedisResult<()> {
    let my_key = format!("friends:{}", my_id);
    let their_key = format!("friends:{}", their_id);
    let _: () = try!(conn.sadd(my_key, their_id));
    let _: () = try!(conn.sadd(their_key, my_id));
    Ok(())
}

fn friends_in_common(conn: &Connection, my_id: UserId, their_id: UserId) -> RedisResult<HashSet<UserId>> {
    let my_key = format!("friends:{}", my_id);
    let their_key = format!("friends:{}", their_id);
    Ok(try!(conn.sinter((my_key, their_key))))
}

fn add_score(conn: &Connection, username: &str, score: uint) -> RedisResult<()> {
    conn.zadd("leaderboard", username, score)
}

type Leaderboard = Vec<(String, uint)>;

fn show_leaderboard(conn: &Connection, n: int) {
    let result: RedisResult<Leaderboard> = conn.zrevrange_withscores("leaderboard", 0, n - 1);
    match result {
        Ok(board) => {
            println!("----==== Top {} players ====----", n);
            for (i, (username, score)) in board.into_iter().enumerate() {
                println!("{:<5} {:^20} {:>4}", i + 1, username, score);
            }
        },
        Err(_) => println!("Failed to fetch leaderboard."),
    }
}

fn main() {
    println!("24 days of Rust - redis (day 18)");
    let client = Client::open("redis://127.0.0.1/").unwrap();
    let conn = client.get_connection().unwrap();
    let _: () = conn.set("answer", 42i).unwrap();
    let answer: int = conn.get("answer").unwrap();
    println!("Answer: {}", answer);

    for i in range(1, 10u64) {
        add_friend(&conn, i, i + 2).ok().expect("Friendship failed :(");
    }
    println!("You have {} friends in common.",
             friends_in_common(&conn, 2, 3).map(|s| s.len()).unwrap_or(0));

    let players = vec!["raynor", "kerrigan", "mengsk", "zasz", "tassadar"];
    for player in players.iter() {
        let score = rand::random::<uint>() % 1000;
        add_score(&conn, *player, score).ok().expect("Nuclear launch detected");
    }
    show_leaderboard(&conn, 3);
}
