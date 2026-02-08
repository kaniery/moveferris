use std::thread;
use std::time::Duration;
use clearscreen::clear;

fn main() {
    // 2フレームを用意して、下部の1箇所を切り替えることで足を動かす
    let ferris_frames: [&str; 3] = [
    r#"                                                  
            _~^~^~_
        _) /  o o  \ (_
          '_   -   _'
          \ '-----' /
    "#,
    r#"                                                  
            _~^~^~_
        () /  o o  \ ()
          '_   -   _'
          | '-----' |        
    "#,
    r#"                                                  
            _~^~^~_
        \) /  o o  \ (/
          '_   v   _'
          / '-----' \         
    "#
    ];

    // フレーム数と移動幅を調整
    let frames = ferris_frames.len();
    for i in 0..18 {
        // 1. 画面をクリア
        clear().unwrap();

        // 2. 左にスペースを入れて位置をずらす（戻りも可能）
        let indent: String = " ".repeat(i % 20);

        // 今回はフレームを交互に切り替えて足を動かす
        let frame = ferris_frames[i % frames];
        for line in frame.lines() {
            println!("{}\x1b[31m{}\x1b[0m", indent, line);
        }

        // 3. 少し待機（約10FPS）
        thread::sleep(Duration::from_millis(100));
    }
}