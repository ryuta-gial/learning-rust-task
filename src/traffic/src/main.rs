// Enumとパターンマッチング:
// トラフィックライトの状態（赤、黄、緑）を表すEnumを定義し、それぞれの状態に対してどのくらいの時間待つべきかを示す関数
// main関数内でTrafficLight列挙型の各値（Red、Yellow、Green）をそれぞれ変数に代入しています
// 次に、各信号のwaiting_time関数を呼び出し、それぞれの待ち時間を出力
fn main() {
    let red_light = TrafficLight::Red;
    let yellow_light = TrafficLight::Yellow;
    let green_light = TrafficLight::Green;

    println!(
        "Red light waiting time: {} seconds",
        waiting_time(red_light)
    );
    println!(
        "Yellow light waiting time: {} seconds",
        waiting_time(yellow_light)
    );
    println!(
        "Green light waiting time: {} seconds",
        waiting_time(green_light)
    );
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn waiting_time(light: TrafficLight) -> u32 {
    match light {
        TrafficLight::Red => 30,
        TrafficLight::Yellow => 5,
        TrafficLight::Green => 60,
    }
}
