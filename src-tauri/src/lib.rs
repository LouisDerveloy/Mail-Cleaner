// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::ipc::{Channel};

#[tauri::command]
async fn get_list(ret_channel: Channel<[i32; 10]>) {
    let lst = [1,2,3,4,5,6,7,8,9,10];

    for i in 0..150 {
        let mut ret = lst;
        for j in 0..10 {
            ret[j as usize] = ret[j as usize] + i*10;
        }

        println!("Data sent");
        std::thread::sleep(std::time::Duration::from_secs(1));
        ret_channel.send(ret).unwrap();
    }

}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_list])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
