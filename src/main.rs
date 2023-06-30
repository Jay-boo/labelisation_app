pub mod csv_row_processor;
use colored::*;
pub mod regex_sets;
use ctrlc;
extern crate csv;
use std::iter::zip;
use std::io;
use std::process;
use std::str::FromStr;
use chrono::NaiveDate;
use colored::Colorize;
use chrono::DateTime;
use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;







struct CsvRow {
    row: csv::StringRecord,
    label: String
}


impl CsvRow{
    fn new(row: csv::StringRecord, label: String) -> Self {
        CsvRow { row, label }
    }
}


fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let args: Vec<String> = env::args().collect();
    let mode = &args[1];

    println!("{}",mode);

    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();

    ctrlc::set_handler(move ||{
        if running_clone.load( Ordering::SeqCst){
            running_clone.store(false,Ordering::SeqCst)
        }else{
            process::exit(0);
        }
    } ).expect("Error setting ctrl c handler");
    //-----------------------------------
    // Load table
    let csv_path="src/.data/processed_df_labelisation.csv";
    let mut reader = match csv::Reader::from_path(csv_path){
        Ok(reader) => reader,
        Err(err) => {
            eprintln!("Failed to open csv source file: {}",err);
            process::exit(1);
        }
    };

    //------------------------------
    //Restrict iterator watching already done labelized rows

    let mut output_path:String=Default::default();
    if mode=="label"{

        output_path = format!("src/.data/new_df_usable_profiteroles.csv");
    }else if mode=="sampling"{
        output_path = format!("src/.data/sampling.csv");
    }else{
        panic!("not correct Mode");
    }
    let output_path_clone=output_path.clone();

    let mut output_file_content = match csv::Reader::from_path(output_path){
        Ok( mut output_file_content) => {
            output_file_content
        },
        Err(err) => {
            eprintln!("Reader doesn't find output file :{}",err);
            let output_path_clone_clone=output_path_clone.clone();
            let file=match std::fs::File::create(output_path_clone) {
                Ok(file) => {
                    eprintln!("New file created");
                    file
                },
                Err(err) => {
                    eprintln!("Can't create file");
                    process::exit(1);
                }
            };
            csv::Reader::from_path(output_path_clone_clone).unwrap()

        }
    };
    let size_registered_data=output_file_content.records().count();
    println!("Actual columns registered : {}",size_registered_data);

    //---------------------------------------
    //Open output file 

    let mut output_path:String=Default::default();
    if mode=="label"{

        output_path = format!("src/.data/new_df_usable_profiteroles.csv");
    }else if mode=="sampling"{
        output_path = format!("src/.data/sampling.csv");
    }else{
        panic!("not correct Mode");
    }
    // let file = match std::fs::File::create(&output_path) {
    //     Ok(file) => file,
    //     Err(err) => {
    //         eprintln!("Failed to create output file: {}", err);
    //         process::exit(1);
    //     }
    // };
    println!("otuputttttt{}",output_path);


    let file = match std::fs::OpenOptions::new().write(true).append(true).create_new(true).open(&output_path) {
        Ok(file) => file,
        Err(err) => match err.kind() {
            std::io::ErrorKind::AlreadyExists => {
                match std::fs::OpenOptions::new().write(true).append(true).open(&output_path) {
                    Ok(file) => {
                        eprintln!(" Open existing file finally {}",output_path);
                        file
                    },
                    Err(err) => {
                        eprintln!("Failed to open existing file: {}", err);
                        process::exit(1);
                    }
                }
            }
            _ => {
                eprintln!("Failed to create or open file: {}", err);
                process::exit(1);
            }
        },
    };






    

    //------------------------
    // Writer
    let mut wrt=csv::Writer::from_writer(file);
    let mut rows: Vec<CsvRow>=Vec::new();
    if size_registered_data==0{
        let headers = match reader.headers(){
            Ok(headers) => headers,
            Err(err) => {
                eprintln!("Failed to find headers in file: {}", err);
                process::exit(1);
            }
        };
        let mut final_headers=headers.clone();
        final_headers.push_field("target");
        wrt.write_record(&final_headers);
    }















    let _iterator= reader.records().skip(size_registered_data);
    for res in _iterator{
        println!("--------------------------------");
        let mut processor=csv_row_processor::CsvRowProcessor::new(csv_row_processor::Row{..Default::default()});
        
        let record = match res {
            Ok(record) => record,
            Err(err) => {
                eprintln!("Failed to read CSV record: {}", err);
                process::exit(1);
            }
        };

        let collected_date = match NaiveDate::parse_from_str(record.get(13).unwrap(), "%Y-%m-%d") {
            Ok(date) => date,
            Err(err) => {
                eprintln!("Failed to parse collected_date: {}", err);
                process::exit(1);
            }
        };

        processor.next_row(
            csv_row_processor::Row{
                announcement_id: record.get(1).unwrap().to_string(),
                type_source: record.get(2).unwrap().to_string(),
                estate_type: record.get(3).unwrap().to_string(),
                price : record.get(4).unwrap().parse::<f32>().unwrap(),
                price_m2 : record.get(5).unwrap().parse::<f32>().unwrap(),
                area:record.get(6).unwrap().parse::<f32>().unwrap(),
                room_count:record.get(7).unwrap().parse::<f32>().unwrap() as i32,
                meuble:match record.get(8).unwrap(){
                    "True"=>true,
                    "False"=> false,
                    _=> false
                },
                postal_code:record.get(9).unwrap().to_string(),
                lat: record.get(10).unwrap().to_string(),
                lon: record.get(11).unwrap().to_string(),
                description: record.get(12).unwrap().to_string(),
                collected_date: collected_date,
                data_source: record.get(14).unwrap().to_string(),
                type_owner:record.get(15).unwrap().parse::<i32>().unwrap(),
            }
        );
        if mode=="label"{


            // 1---------------------------
            //  Detect anomaly
            println!("Row : ");
            for (field,field_name) in zip(record.iter().skip(1),vec!["announcement_id","type_source","estate_type","price","price_m2","area","room_count","meuble","postal_code","lat","lon","description","collected_date","data_source","type_owner"]) {
                let mut field_str=String::from(field);
                if field_name=="description"{continue;}
                if field_name=="price"{
                    field_str=field_str.color(Color::Green).to_string();
                }else if field_name=="estate_type"{
                    field_str=field_str.color(Color::Red).to_string();
                }else if field_name=="area" {
                     field_str =field_str.color(Color::Blue).to_string();
                }else if field_name=="room_count"{
                     field_str =field_str.color(Color::Yellow).to_string();
                }else if field_name=="meuble"{
                     field_str =field_str.color(Color::TrueColor { r: 160, g: 32, b: 240 }).to_string();

                }
                println!("{}:{}:",field_name, field_str);
            }
            println!("-----");
            processor.detect_warnings();
            println!(" Anomaly count {}",processor.warn_score);
            println!("-----");
            


            // 2------------------------
            // Use regex
            println!("{}",processor.detect_known_expression());



            //3---------------------------
            //print Information

            // println!(" Original col :\n {:?}", record);
            let mut new_col = String::new();
            println!("-----");
            println!("Enter the label value for row :" );
            io::stdin()
                .read_line(&mut new_col)
                .expect("Failed to read input");
            let new_col = new_col.trim().to_string();
            let _new_col_clone=new_col.clone();
            let csv_row=CsvRow::new(record,new_col);
            let last_row=csv_row.row.clone();

            rows.push(csv_row);
            write_record(&mut wrt, &last_row,_new_col_clone);
            if !running.load(Ordering::SeqCst){
                println!("break");
                break;
                
            }
        }
        else if mode=="sampling"{

            processor.detect_warnings();
            let warns:i32=processor.warn_score;
            let csv_row=CsvRow::new(record, warns.to_string());
            let last_row=csv_row.row.clone();
            println!("warn score : {}",warns);
            write_record(&mut wrt, &last_row,warns.to_string());

        }
        else{
            panic!("Unknow mode")
        }


    }
    
}

    fn write_record(wrt: &mut csv::Writer<std::fs::File>, row: &csv::StringRecord,label:String) -> Result<(), csv::Error> {
        let mut record = row.clone();
        record.push_field(&label);
        wrt.write_record(&record)
 }
