pub fn hello_world() {
    println!("Hello, world!");

    let is_indian: bool = true;
    let can_dance: bool = true;
    let can_sing: bool = false;

    let desi_singer: bool = is_indian && can_sing;
    let desi_dancer: bool = is_indian && can_dance;
    let is_actor: bool = desi_dancer || desi_singer;
    let good_artist: bool = !(is_actor);

    let my_initial: char = 'X';

    println!(
        "\nI am an artist {0} who played in {1}-Men.",
        good_artist, my_initial
    )
}
