use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use rand::seq::SliceRandom;
use std::io::{stdout, Result as IoResult, Write};
use std::thread;
use std::time::Duration;

fn create_array() -> Vec<i32> {
    let mut rng = rand::rng();
    let mut arr: Vec<i32> = (1..50).collect();
    arr.shuffle(&mut rng);
    arr
}

fn display_rows(
    stdout: &mut std::io::Stdout,
    arr: &Vec<i32>,
    prev_arr: Option<&Vec<i32>>,
) -> IoResult<()> {
    const SIGN: char = '█';
    if prev_arr.is_none() {
        execute!(stdout, MoveTo(0, 0))?;
        for (index, i) in arr.iter().enumerate() {
            execute!(stdout, MoveTo(0, index as u16))?;
            execute!(stdout, Clear(ClearType::CurrentLine))?;
            for _j in 0..*i {
                execute!(stdout, Print(SIGN))?;
            }
        }
    } else if let Some(prev) = prev_arr {
        for (index, (current_val, prev_val)) in arr.iter().zip(prev.iter()).enumerate() {
            if current_val != prev_val {
                execute!(stdout, MoveTo(0, index as u16))?;
                execute!(stdout, Clear(ClearType::CurrentLine))?;
                for _j in 0..*current_val {
                    execute!(stdout, Print(SIGN))?;
                }
            }
        }
    }
    stdout.flush()?;
    Ok(())
}

fn bubble_sort() -> IoResult<()> {
    let mut stdout = stdout();
    let mut arr = create_array();
    let arr_len = arr.len();

    execute!(stdout, Clear(ClearType::All))?;
    display_rows(&mut stdout, &arr, None)?;
    thread::sleep(Duration::from_millis(10));

    for i in 0..arr_len - 1 {
        for j in 0..arr_len - i - 1 {
            if arr[j] > arr[j + 1] {
                let prev_arr_state = arr.clone();
                arr.swap(j, j + 1);
                display_rows(&mut stdout, &arr, Some(&prev_arr_state))?;
                thread::sleep(Duration::from_millis(10))
            }
        }
    }
    thread::sleep(Duration::from_secs(2));
    Ok(())
}

fn selection_sort() -> IoResult<()> {
    let mut stdout = stdout();
    let mut arr = create_array();
    let arr_len = arr.len();

    execute!(stdout, Clear(ClearType::All))?;
    display_rows(&mut stdout, &arr, None)?;
    thread::sleep(Duration::from_millis(90));
    for i in 0..arr_len {
        let mut min_index = i;
        for current_index in i + 1..arr_len {
            if arr[current_index] < arr[min_index] {
                min_index = current_index;
            }
        }
        let prev_arr_state = arr.clone();
        arr.swap(i, min_index);
        display_rows(&mut stdout, &arr, Some(&prev_arr_state))?;
        thread::sleep(Duration::from_millis(90));
    }
    thread::sleep(Duration::from_secs(2));
    Ok(())
}

fn insertion_sort() -> IoResult<()> {
    let mut stdout = stdout();
    let mut arr = create_array();
    let arr_len = arr.len();
    execute!(stdout, Clear(ClearType::All))?;
    display_rows(&mut stdout, &arr, None)?;
    thread::sleep(Duration::from_millis(20));
    for i in 1..arr_len {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            let prev_arr_state = arr.clone();
            arr.swap(j, j - 1);
            j -= 1;
            display_rows(&mut stdout, &arr, Some(&prev_arr_state))?;
            thread::sleep(Duration::from_millis(20));
        }
    }
    thread::sleep(Duration::from_secs(2));
    Ok(())
}

fn main() -> IoResult<()> {
    let mut stdout = stdout();
    execute!(stdout, SetForegroundColor(Color::DarkGreen))?;
    execute!(stdout, Clear(ClearType::All))?;
    execute!(stdout, MoveTo(0, 0))?;
    execute!(
        stdout,
        Print(
            r#"
    ____        __    __    __        _____            __ 
   / __ )__  __/ /_  / /_  / /__     / ___/____  _____/ /_
  / __  / / / / __ \/ __ \/ / _ \    \__ \/ __ \/ ___/ __/
 / /_/ / /_/ / /_/ / /_/ / /  __/   ___/ / /_/ / /  / /_  
/_____/\__,_/_.___/_.___/_/\___/   /____/\____/_/   \__/  
                                                          "#
        )
    )?;
    thread::sleep(Duration::from_secs(1));
    bubble_sort()?;
    execute!(stdout, Clear(ClearType::All))?;
    execute!(stdout, MoveTo(0, 0))?;
    execute!(
        stdout,
        Print(
            r#"
   _____      __          __  _                _____            __ 
  / ___/___  / /__  _____/ /_(_)___  ____     / ___/____  _____/ /_
  \__ \/ _ \/ / _ \/ ___/ __/ / __ \/ __ \    \__ \/ __ \/ ___/ __/
 ___/ /  __/ /  __/ /__/ /_/ / /_/ / / / /   ___/ / /_/ / /  / /_  
/____/\___/_/\___/\___/\__/_/\____/_/ /_/   /____/\____/_/   \__/  
                                                                   
    "#
        )
    )?;
    thread::sleep(Duration::from_secs(1));
    selection_sort()?;
    execute!(stdout, Clear(ClearType::All))?;
    execute!(stdout, MoveTo(0, 0))?;
    execute!(
        stdout,
        Print(
            r#"
    ____                     __  _                _____            __ 
   /  _/___  ________  _____/ /_(_)___  ____     / ___/____  _____/ /_
   / // __ \/ ___/ _ \/ ___/ __/ / __ \/ __ \    \__ \/ __ \/ ___/ __/
 _/ // / / (__  )  __/ /  / /_/ / /_/ / / / /   ___/ / /_/ / /  / /_  
/___/_/ /_/____/\___/_/   \__/_/\____/_/ /_/   /____/\____/_/   \__/  
                                                                      
    "#
        )
    )?;
    thread::sleep(Duration::from_secs(1));
    insertion_sort()?;
    execute!(stdout, Clear(ClearType::All))?;
    execute!(stdout, MoveTo(0, 0))?;
    Ok(())
}
