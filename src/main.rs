use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use curl::easy::Easy;
use user_input::get_input;

fn main() {
	let b = std::path::Path::new("emip\\check.version5").exists();
	if !b {
		download_all();
	}
	print_options();
	let selection: &str = &get_input("Select an option:");
	match selection {
		"1" => run_exe("cmd","/C","emip\\UABE\\AssetBundleExtractor.exe applyemip emip\\Jupiter.emip ."),
		"2" => run_exe("cmd","/C","emip\\UABE\\AssetBundleExtractor.exe applyemip emip\\Moon.emip ."),
		"3" => run_exe("cmd","/C","emip\\UABE\\AssetBundleExtractor.exe applyemip emip\\NoBloom.emip ."),
		"4" => run_exe("cmd","/C","emip\\UABE\\AssetBundleExtractor.exe applyemip emip\\AlphaTitleScreen.emip ."),
		"5" => apply_cheats(),
		"6" => run_exe("cmd","/C","emip\\UABE\\AssetBundleExtractor.exe applyemip emip\\ModdingSticker.emip ."),
		"7" => map_menu(),
		"8" => std::process::exit(0),
		_ => println!("Invalid option, try again."),
	}
	let exit: &str = &get_input("Close installer? (y/n) ");
	match exit {
		"n" => main(),
		"y" => std::process::exit(0),
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
	println!("Welcome to Torph's Shipbreaker mod installer! If you've used");
	println!("mods before, please verify your game files in Steam before");
	println!("using, to avoid potential crashes.");
	println!(" ");
	println!(" ");
	println!("      1. Jupiter over Earth");
	println!("      2. Moon over Earth");
	println!("      3. No Bloom");
	println!("      4. Alpha Title Screen");
	println!("      5. Cheats");
	println!("      6. Modding Sticker");
	println!("      7. Maps");
	println!("      8. Exit Mod Installer");
	println!(" ");
	println!(" ");
}
fn map_menu() {
	assert!( std::process::Command::new("cls").status().or_else(|_| std::process::Command::new("clear").status()).unwrap().success() );
	println!("When you select a map from this list, it will be loaded instead of the cutting bay.");
	println!("To revert to the normal map, verify your game files, or select the same map again.");
	println!(" ");
	println!(" ");
	println!("      1. Master Jack (just the Master Jack, a ship, and infinite void)");
	println!("      2. Elemental Test Map (fun object spawners, replaces Master Jack)");
	println!("      3. Subsystem Test Map (replaces Master Jack)");
	println!("      4. QA map (TONS of super old scrapped concepts from early in development)");
	println!("      5. Deep Space");
	println!("      6. Back");
	println!(" ");
	println!(" ");
	fs::copy("Shipbreaker_Data\\level6", "Shipbreaker_Data\\level14").expect("Couldn't copy scene!"); //I needed a second empty scene for the Deep Space option
	let map_selection: &str = &get_input("Select a map: ");
	match map_selection {
		"1" => swap_scenes("8","6"),
		"2" => swap_scenes("9","7"),
		"3" => swap_scenes("10","9"),
		"4" => swap_scenes("5","8"),
		"5" => { swap_scenes("8","6"); swap_scenes("9","14"); }
		"6" => main(),
		_ => println!("Invalid option, please try again."),
	}
	let exit: &str = &get_input("Stay on map list? (y/n) ");
	match exit {
		"n" => main(),
		"y" => map_menu(),
		_ => (),
	}
}
fn swap_scenes(num1:&str,num2:&str) {
	println!("Swapping Unity scene files...");
	let first_arg = format!("Shipbreaker_Data\\level{}", num1);
	let second_arg = format!("Shipbreaker_Data\\level{}.temp", num1);
	fs::rename(&first_arg, &second_arg).expect("Couldn't rename scene!");
	let exist_arg = format!(r"Shipbreaker_Data\level{}.resS", num1);
	let exist = std::path::Path::new(&exist_arg).exists();
	if exist {
		let second_arg = format!(r"Shipbreaker_Data\level{}.resS.temp", num1);
		fs::rename(&exist_arg, &second_arg).expect("Couldn't rename scene!");
	}
	
	let first_arg = format!(r"Shipbreaker_Data\level{}", num2);
	let second_arg = format!(r"Shipbreaker_Data\level{}", num1);
	fs::rename(&first_arg, &second_arg).expect("Couldn't rename scene!");
	
	let exist_arg = format!(r"Shipbreaker_Data\level{}.resS", num2);
	let exist = std::path::Path::new(&exist_arg).exists();
	if exist {
		let second_arg = format!(r"Shipbreaker_Data\level{}.resS", num1);
		fs::rename(&exist_arg, &second_arg).expect("Couldn't rename scene!");
	}
	
	let first_arg = format!(r"Shipbreaker_Data\level{}.temp", num1);
	let second_arg = format!(r"Shipbreaker_Data\level{}", num2);
	fs::rename(&first_arg, &second_arg).expect("Couldn't rename scene!");
	
	let exist_arg = format!(r"Shipbreaker_Data\level{}.resS.temp", num1);
	let exist = std::path::Path::new(&exist_arg).exists();
	if exist {
		let second_arg = format!(r"Shipbreaker_Data\level{}.resS", num2);
		fs::rename(&exist_arg, &second_arg).expect("Couldn't rename scene!");
	}
	println!(" ");
	println!("Done.");
}
fn download_all() {
	println!("Downloading mod files...");
    fs::create_dir("emip").expect("Unable to create emip folder!");
	curl_download("https://raw.githubusercontent.com/Torphedo/Shipbreaker-AssetModding/main/bin/Jupiter.emip","emip\\Jupiter.emip");
	curl_download("https://raw.githubusercontent.com/Torphedo/Shipbreaker-AssetModding/main/bin/Moon.emip","emip\\Moon.emip");
	curl_download("https://raw.githubusercontent.com/Torphedo/Shipbreaker-AssetModding/main/bin/NoBloom.emip","emip\\NoBloom.emip");
	curl_download("https://raw.githubusercontent.com/Torphedo/Shipbreaker-AssetModding/main/bin/AlphaTitleScreen.emip","emip\\AlphaTitleScreen.emip");
	curl_download("https://raw.githubusercontent.com/Torphedo/Shipbreaker-AssetModding/main/bin/ModdingSticker.emip","emip\\ModdingSticker.emip");
	curl_download("https://raw.githubusercontent.com/Torphedo/Shipbreaker-AssetModding/main/bin/BBI.Unity.Game.xdelta","emip\\BBI.Unity.Game.xdelta");
	curl_download("https://raw.githubusercontent.com/Torphedo/Shipbreaker-AssetModding/main/bin/Carbon.Core.xdelta","emip\\Carbon.Core.xdelta");
	curl_download("https://raw.githubusercontent.com/Torphedo/Shipbreaker-AssetModding/main/bin/mod_config.ini","mod_config.ini");
	curl_download("https://raw.githubusercontent.com/Torphedo/Shipbreaker-AssetModding/main/bin/check.version5","emip\\check.version5");
	println!("Downloading tools...");
	curl_download("https://raw.githubusercontent.com/NiceneNerd/BCML/master/bcml/helpers/7z.exe", "emip\\7z.exe");
	curl_download("https://raw.githubusercontent.com/marco-calautti/DeltaPatcher/master/xdelta.exe", "emip\\xdelta.exe");
	curl_download("https://github.com/DerPopo/UABE/releases/download/2.2stabled/AssetsBundleExtractor_2.2stabled_64bit.zip", "emip\\UABE.zip");
	run_exe("cmd","/C","emip\\7z x -oemip\\ emip\\UABE.zip");
	fs::remove_file("emip\\UABE.zip").expect("Unable to delete UABE.zip!");
	fs::rename("emip\\64bit", "emip\\UABE").expect("Unable to rename UABE folder!");
	fs::remove_file("emip\\7z.exe").expect("Unable to delete 7zip!");
}
fn apply_cheats() {
	run_exe("cmd","/C",r"emip\xdelta.exe -n -d -s Shipbreaker_Data\Managed\BBI.Unity.Game.dll emip\BBI.Unity.Game.xdelta Shipbreaker_Data\Managed\BBI.Unity.Game.dll.mod");
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