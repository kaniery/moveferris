use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::io::{self, Write};
use crossterm::terminal::size;

fn main() {
    let ferris_frames: [&str; 3] = [
    r#"        _~^~^~_
    _) /  o o  \ (_
      '_   -   _'
      \ '-----' /"#,
    r#"        _~^~^~_
    () /  o o  \ ()
      '_   -   _'
      | '-----' |"#,
    r#"        _~^~^~_
    \) /  o o  \ (/
      '_   -   _'
      / '-----' \"#
    ];

    // 終了フラグ
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = Arc::clone(&running);

    // Ferrisアニメーション用スレッド（ログに残さない描画）
    // 端末幅を取得して右端に描画し、カーソルは保存/復帰するため入力に干渉しない
    let max_width = ferris_frames
        .iter()
        .flat_map(|f| f.lines())
        .map(|l| l.len())
        .max()
        .unwrap_or(0);
    let frame_height = ferris_frames.iter().map(|f| f.lines().count()).max().unwrap_or(1);

    let ferris_thread = thread::spawn(move || {
        let mut frame_count = 0usize;

        while running_clone.load(Ordering::SeqCst) {
            let frame = ferris_frames[frame_count % ferris_frames.len()];

            // 現在の端末サイズを取得
            let (w, h) = size().unwrap_or((80, 24));
            let base_col = if (w as usize) > max_width { (w as usize) - max_width + 1 } else { 1 };
            let base_row = if (h as usize) > frame_height { (h as usize) - frame_height + 1 } else { 1 };

            let mut out = io::stdout();
            // カーソル位置を保存
            write!(out, "\x1b[s").ok();

            for (idx, line) in frame.lines().enumerate() {
                // 右端に描画（改行を出さない）
                write!(out, "\x1b[{};{}H\x1b[31m{}\x1b[0m", base_row + idx, base_col, line).ok();
            }

            // カーソルを元の位置に戻す
            write!(out, "\x1b[u").ok();
            out.flush().ok();

            frame_count = frame_count.wrapping_add(1);
            thread::sleep(Duration::from_millis(100));
        }
    });

    // メインスレッド：ユーザー入力を受け付ける
    // ユーザー出力の前にFerris領域を消去するヘルパ
    let clear_ferris = || {
        if let Ok((w, h)) = size() {
            let base_col = if (w as usize) > max_width { (w as usize) - max_width + 1 } else { 1 };
            let base_row = if (h as usize) > frame_height { (h as usize) - frame_height + 1 } else { 1 };
            let mut out = io::stdout();
            // カーソル位置を保存
            write!(out, "\x1b[s").ok();
            for idx in 0..frame_height {
                // 余白で上書きして消す
                write!(out, "\x1b[{};{}H{}", base_row + idx, base_col, " ".repeat(max_width)).ok();
            }
            // カーソルを元に戻す
            write!(out, "\x1b[u").ok();
            out.flush().ok();
        }
    };

    clear_ferris();
    println!("Ferris がバックグラウンドに常駐しました！");
    println!("コマンドを入力してください（exit で終了）：");
    
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        if input.trim().is_empty() {
            continue;
        }

        match input.trim() {
            "exit" | "quit" => {
                running.store(false, Ordering::SeqCst);
                break;
            }
            _ => {
                // ユーザー出力前にFerris描画領域を一旦消去
                clear_ferris();
                println!("入力: {}", input.trim());
            }
        }
    }

    ferris_thread.join().unwrap();
    println!("終了しました！");
}