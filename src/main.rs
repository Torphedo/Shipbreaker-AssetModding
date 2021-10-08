use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use curl::easy::Easy;
use user_input::get_input;

fn main() {
	let b = std::path::Path::new("emip\\check.version4").exists();
	if b == false {
		download_all();
	}
	print_options();
	let selection: &str = &get_input("Enter the number of the mod you'd like to install:");
	match selection {
		"1" => run_exe("cmd","/C","emip\\UABE\\AssetBundleExtractor.exe applyemip emip\\Jupiter.emip ."),
		"2" => run_exe("cmd","/C","emip\\UABE\\AssetBundleExtractor.exe applyemip emip\\Moon.emip ."),
		"3" => run_exe("cmd","/C","emip\\UABE\\AssetBundleExtractor.exe applyemip emip\\NoBloom.emip ."),
		"4" => run_exe("cmd","/C","emip\\UABE\\AssetBundleExtractor.exe applyemip emip\\AlphaTitleScreen.emip ."),
		"5" => apply_cheats(),
		"6" => run_exe("cmd","/C","emip\\UABE\\AssetBundleExtractor.exe applyemip emip\\ModdingSticker.emip ."),
		"7" => (),
		_ => println!("Invalid option, try again."),
	}
	let exit: &str = &get_input("Exit installer? (y/n):");
	match exit {
		"n" => main(),
		_ => (),
	}
}
fn curl_download(url:&str,name:&'static str) {
	let path = Path::new(name);
	let display = path.display();
	let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };
	let mut easy = Easy::new();
	match easy.follow_location(true) {
	Err(why) => panic!("Failed to set redirect parameter! Cause: {}", why),
			Ok(_) => (),
		}
	easy.url(url).unwrap();
	easy.write_function(move |data| {
		match file.write_all(data) {
			Err(why) => panic!("couldn't write to {}: {}", display, why),
			Ok(_) => (),
		}
		Ok(data.len())
	}).unwrap();
	easy.perform().unwrap();
}
fn run_exe(exe:&str,arg:&'static str,arg2:&'static str) {
	Command::new(exe)
            .args([arg,arg2])
            .output()
            .expect("failed to execute process");
}
fn print_options() {
	assert!( std::process::Command::new("cls").status().or_else(|_| std::process::Command::new("clear").status()).unwrap().success() );
	println!("Welcome to Torph's Shipbreaker mod installer script. If you've");
	println!("used mods before, please verify your game files in Steam before");
	println!("using, to avoid potential crashes.");
	println!(" ");
	println!(" ");
	println!("      1. Jupiter over Earth");
	println!("      2. Moon over Earth");
	println!("      3. No Bloom");
	println!("      4. Alpha Title Screen");
	println!("      5. Cheats");
	println!("      6. Modding Sticker");
	println!("      7. Exit Mod Installer");
	println!(" ");
	println!(" ");
}
fn download_all() {
	println!("Downloading mod files...");
    fs::create_dir("emip").expect("Unable to create emip folder!");
	curl_download("https://raw.githubusercontent.com/Torphedo/Shipbreaker-AssetModding/main/Jupiter.emip","emip\\Jupiter.emip");
	curl_download("https://raw.githubusercontent.com/Torphedo/Shipbreaker-AssetModding/main/Moon.emip","emip\\Moon.emip");
	curl_download("https://raw.githubusercontent.com/Torphedo/Shipbreaker-AssetModding/main/NoBloom.emip","emip\\NoBloom.emip");
	curl_download("https://raw.githubusercontent.com/Torphedo/Shipbreaker-AssetModding/main/AlphaTitleScreen.emip","emip\\AlphaTitleScreen.emip");
	curl_download("https://raw.githubusercontent.com/Torphedo/Shipbreaker-AssetModding/main/ModdingSticker.emip","emip\\ModdingSticker.emip");
	curl_download("https://raw.githubusercontent.com/Torphedo/Shipbreaker-AssetModding/main/Cheats.xdelta","emip\\Cheats.xdelta");
	curl_download("https://raw.githubusercontent.com/Torphedo/Shipbreaker-AssetModding/main/Carbon.Core.xdelta","emip\\Carbon.Core.xdelta");
	curl_download("https://raw.githubusercontent.com/Torphedo/Shipbreaker-AssetModding/main/mod_config.ini","mod_config.ini");
	curl_download("https://raw.githubusercontent.com/Torphedo/Shipbreaker-AssetModding/main/check.version4","emip\\check.version4");
	println!("Downloading tools...");
	curl_download("https://raw.githubusercontent.com/NiceneNerd/BCML/master/bcml/helpers/7z.exe", "emip\\7z.exe");
	curl_download("https://raw.githubusercontent.com/marco-calautti/DeltaPatcher/master/xdelta.exe", "emip\\xdelta.exe");
	curl_download("https://github.com/DerPopo/UABE/releases/download/2.2stabled/AssetsBundleExtractor_2.2stabled_64bit.zip", "emip\\UABE.zip");
	run_exe("cmd","/C","emip\\7z x -oemip\\ emip\\UABE.zip");
	fs::remove_file("emip\\UABE.zip").expect("Unable to delete UABE.zip!");
	fs::rename("emip\\64bit", "emip\\UABE").expect("Unable to rename UABE folder!");
}
fn apply_cheats() {
	run_exe("cmd","/C","emip\\xdelta.exe -d -s Shipbreaker_Data\\Managed\\BBI.Unity.Game.dll emip\\Cheats.xdelta Shipbreaker_Data\\Managed\\BBI.Unity.Game.dll.mod");
	fs::remove_file("Shipbreaker_Data\\Managed\\BBI.Unity.Game.dll").expect("Unable to delete original DLL!");
	fs::rename("Shipbreaker_Data\\Managed\\BBI.Unity.Game.dll.mod", "Shipbreaker_Data\\Managed\\BBI.Unity.Game.dll").expect("Unable to rename modded DLL!");
	
	run_exe("cmd","/C","emip\\xdelta.exe -d -s Shipbreaker_Data\\Managed\\Carbon.Core.dll emip\\Carbon.Core.xdelta Shipbreaker_Data\\Managed\\Carbon.Core.dll.mod");
	fs::remove_file("Shipbreaker_Data\\Managed\\Carbon.Core.dll").expect("Unable to delete original DLL!");
	fs::rename("Shipbreaker_Data\\Managed\\Carbon.Core.dll.mod", "Shipbreaker_Data\\Managed\\Carbon.Core.dll").expect("Unable to rename modded DLL!");
	println!("Patch Applied.");
}
mod user_input {
    use std::io;
    pub fn get_input(prompt: &str) -> String{
        println!("{}",prompt);
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_goes_into_input_above) => {},
            Err(_no_updates_is_fine) => {},
        }
        input.trim().to_string()
    }
}