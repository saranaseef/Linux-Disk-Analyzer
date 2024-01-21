use std::env;
use std::path::{Path, PathBuf};
use gtk4 as gtk;
mod scanner;
mod delete_file;
mod directory_operations;
use gtk::{prelude::*, BoxLayout, Button};
use gtk::{Label, Window,ScrolledWindow, ApplicationWindow, Application, Orientation, ComboBoxText, Box, TextView, TextBuffer};
use charts_rs::{BarChart, PieChart, Pie, SeriesCategory, THEME_GRAFANA};
use std::process::Command;


const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn build_ui(app: &Application) {
    let scrolled_window = ScrolledWindow::new();
    scrolled_window.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    scrolled_window.set_vexpand(true);
    scrolled_window.set_hexpand(true);

    let mainBox = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    let directoryLabel = gtk::Label::builder()
        .label("Size:")
        .build();

    let sizeLabel = gtk::Label::builder()
        .label("Enter Directory:")
        .build();

    let textBox = gtk::Entry::builder()
        .build();

    let removeButton = gtk::Button::builder()
        .label("Back")
        .build();

    let deleteButton = gtk::Button::builder()
        .label("Delete")
        .build();

    let display = gtk::Button::builder()
        .label("Display Contents")
        .build();

    let addDirectory = gtk::Button::builder()
        .label("Fetch Size")
        .build();

    let mut image = gtk::Image::new();


    let textBoxCloneback = textBox.clone();


    let mainClone2 = mainBox.clone(); 
    let remove = removeButton.clone();
    let addDirectoryClone = addDirectory.clone();
    let directoryLabelClone = directoryLabel.clone();
    let directoryLabelClone1 = directoryLabel.clone();
    let directoryLabelClone2 = directoryLabel.clone();
    let textClone = textBox.clone();
    let textClone2 = textBox.clone();
    let textBoxClone3 = textBox.clone();
    let textBoxCloneDelete = textBox.clone();
    let deleteButtonClone= deleteButton.clone();
    let removeButtonClone= removeButton.clone();
    let displayCl = display.clone();
    let s = sizeLabel.clone();
    let imageClone = image.clone();



    removeButton.connect_clicked(move |_| {
        let text = textBoxCloneback.text().to_string();
        
        let t = textBoxClone3.clone();
        let l = directoryLabelClone2.clone();
        let textB = textBoxCloneback.clone();
        let addDir = addDirectoryClone.clone();
        let delete= deleteButtonClone.clone();
        let dis = displayCl.clone();
        let iClone = imageClone.clone();

        if let Some(index) = text.rfind('/') {
            let modified_text = &text[0..index];
            textBoxCloneback.set_text(modified_text);
        } else {
            textBoxCloneback.set_text("");
        }
        unsafe{
            while(mainClone2.last_child().is_some())
            {
                mainClone2.remove(&mainClone2.last_child().unwrap());
            }
        }
        unsafe { mainClone2.append(&iClone) };
        unsafe { mainClone2.append(&s) };
        unsafe{mainClone2.append(&textB)};
        unsafe{mainClone2.append(&l)};
        unsafe{mainClone2.append(&addDir)};
        unsafe{mainClone2.append(&delete)};
        unsafe{mainClone2.append(&dis)};
        // unsafe{mainClone2.append(&remove)};
        unsafe{mainClone2.append(&remove);}

    });

    let text_view = TextView::new();

    addDirectory.connect_clicked(move |_| {
        let path = textClone.text().to_string();
        let path2 = Path::new(&path);
        if path2.is_dir() {
            let size = scanner::display_total_size(&path2);
            let label_text = format!("Size: {}", size);
            directoryLabelClone.set_text(&label_text);
        } else {
            let size = scanner::displayFileSize(&path2);
            let label_text = format!("Size: {}", size);
            directoryLabelClone.set_text(&label_text);
        }
    });

    deleteButton.connect_clicked(move |_| {
        let path = textBoxCloneDelete.text().to_string();
        let path2 = Path::new(&path);
        if path2.is_dir() {
            println!("This is a folder");
            //delete_file::delete_folder(&path2);

        }
        else{
            println!("This is a file");

            delete_file::delete_file(&path2);

        }
    });    
    

      

    static mut allDirs:Vec<String> = vec![];
    let mut dirButtons:Vec<Button> = vec![];
    let mut dirs2 = unsafe{dirButtons.clone()};
    let displayClone = display.clone();
    let mainBoxClone = mainBox.clone();
    let mainBoxClone1 = mainBox.clone();
    let addClone = addDirectory.clone();
    let textClone4 = textBox.clone();
    let delClone = deleteButton.clone();
    let sizeClone = sizeLabel.clone();
    let iClone = image.clone();


    display.connect_clicked(move |_| {
        let path = textClone2.text().to_string();
        let path2 = Path::new(&path);
        let data: Vec<(PathBuf, String, u64)> = scanner::display_file_sizes(&path2);
    
        let mut folder_buttons: Vec<Button> = Vec::new();
        let mut file_buttons: Vec<Button> = Vec::new();
        let mut totalSize = scanner::display_total_size_unformatted(&path2);

        let d2 = data.clone();
        let iClone1 = iClone.clone();



        for entry in data {
            if entry.1 == "Folder" {
                let new_button: Button = gtk::Button::builder()
                .label(&format!("Folder: {}", entry.0.file_name().unwrap().to_string_lossy()))
                .build();

                let s = sizeClone.clone();
                let t = textClone4.clone();
                let mainClone = mainBoxClone.clone();
                let l = directoryLabelClone1.clone();
                let textB = textClone4.clone();
                let addDir = addClone.clone();
                let delete = delClone.clone();
                let remove = removeButtonClone.clone();
                let dis = displayClone.clone();
                let iClone2 = iClone1.clone();

    
                new_button.connect_clicked(move |_| {
                    t.set_text(&entry.0.display().to_string());
                    unsafe {
                        while (mainClone.last_child().is_some()) {
                            mainClone.remove(&mainClone.last_child().unwrap());
                        }
                    }
                    unsafe { mainClone.append(&iClone2) };
                    unsafe { mainClone.append(&s) };
                    unsafe { mainClone.append(&textB) };
                    unsafe { mainClone.append(&l) };
                    unsafe { mainClone.append(&addDir) };
                    unsafe { mainClone.append(&delete) };
                    unsafe { mainClone.append(&dis) };
                    unsafe { mainClone.append(&remove) };
                });
    
                folder_buttons.push(new_button);
            } else if entry.1 == "File" {
                let new_button: Button = gtk::Button::builder()
                 .label(&format!("File: {}", entry.0.file_name().unwrap().to_string_lossy()))
                 .build(); 
                let t = textClone4.clone();
                let mainClone = mainBoxClone.clone();
                let l = directoryLabelClone1.clone();
                let textB = textClone4.clone();
                let addDir = addClone.clone();
                let delete = delClone.clone();
                let remove = removeButtonClone.clone();
                let dis = displayClone.clone();
                let s =   sizeClone.clone();
                let iClone2 = iClone1.clone();

    
                new_button.connect_clicked(move |_| {
                    t.set_text(&entry.0.display().to_string());
                    unsafe {
                        while (mainClone.last_child().is_some()) {
                            mainClone.remove(&mainClone.last_child().unwrap());
                        }
                    }
                    unsafe { mainClone.append(&iClone2) };
                    unsafe { mainClone.append(&s) };
                    unsafe { mainClone.append(&textB) };
                    unsafe { mainClone.append(&l) };
                    unsafe { mainClone.append(&addDir) };
                    unsafe { mainClone.append(&delete) };
                    unsafe { mainClone.append(&dis) };
                    unsafe { mainClone.append(&remove) };
                });
    
                file_buttons.push(new_button);
            }
        }
    
        for folder_button in folder_buttons {
            mainBoxClone1.append(&folder_button);
        }
    
        for file_button in file_buttons {
            mainBoxClone1.append(&file_button);
        }

        let mut percentages : Vec<(PathBuf, f32)> = vec![];
        let mut pieChart = PieChart::new(vec![]);


        for entry in d2
        {
            let percentt : f32 = 100.0;
           let mut percent : f32 = (entry.2 as f32 / totalSize as f32)*100.0 as f32;
           println!("percent: {}", percent);
           let name = Some(entry.0.to_str());
           percentages.push((entry.0 ,percent));
        
        }

        for entry in percentages
        {
            let name = entry.0.file_name().unwrap().to_string_lossy();
            let t:&str = &name;
            println!("{}", entry.1);
            pieChart.series_list.push((t, vec![entry.1]).into());
        }
        println!{"HEREEE"};
        pieChart.title_text = "Sizes".to_string();
        pieChart.svg().unwrap();
        // Example: export SVG to file
        let svg_data = pieChart.svg().unwrap();
        std::fs::write("chart.svg", &svg_data).expect("Failed to write SVG to file");
        let imagePath = "chart.svg";
        iClone1.set_from_file(Some(imagePath));
        iClone1.set_pixel_size(550);
        iClone1.set_hexpand(true);
    });
         
  

 
    let dirLabel = directoryLabel.clone();
    let textB = textBox.clone(); 
    let addDir = addDirectory.clone();
    let delButton = deleteButton.clone();
    let size = sizeLabel.clone();

    
    mainBox.prepend(&image);
    mainBox.append(&size);
    mainBox.append(&textB); 
    mainBox.append(&addDirectory);
    mainBox.append(&dirLabel);
    mainBox.append(&delButton);
    mainBox.append(&display);    
    mainBox.append(&removeButton); 

    scrolled_window.set_child(Some(&mainBox));
    let b = gtk::Box::builder()
    .build();

    b.append(&scrolled_window);
    

    let window = ApplicationWindow::builder()
    .application(app)
    .title("Disk Analyzer")
    .default_width(850)
    .default_height(850)
    .resizable(true)
    .child(&b)
    .build();


    window.show();
    
}

fn main() {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();
}