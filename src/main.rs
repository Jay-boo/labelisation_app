pub mod csv_row_processor;
extern crate csv;
use std::io;
use std::process;
use std::str::FromStr;
use chrono::NaiveDate;
use colored::Colorize;
use chrono::DateTime;
use std::env;







struct CsvRow {
    row: csv::StringRecord,
    new_col: String
}


impl CsvRow{
    fn new(row: csv::StringRecord, new_col: String) -> Self {
        CsvRow { row, new_col }
    }
}


fn main() {
    env::set_var("RUST_BACKTRACE", "1");
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
    let output_path = format!("src/.data/new_df_usable_profiteroles.csv");
    let file = match std::fs::File::create(&output_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Failed to create output file: {}", err);
            process::exit(1);
        }
    };
    let mut wrt=csv::Writer::from_writer(file);
    let mut rows: Vec<CsvRow>=Vec::new();
    let headers = match reader.headers(){
        Ok(headers) => headers,
        Err(err) => {
            eprintln!("Failed to create output file: {}", err);
            process::exit(1);
        }
    };
    let mut final_headers=headers.clone();
    final_headers.push_field("target");
    wrt.write_record(&final_headers);
    




    




    for res in reader.records().take(5){
        let record = match res {
            Ok(record) => record,
            Err(err) => {
                eprintln!("Failed to read CSV record: {}", err);
                process::exit(1);
            }
        };
        let third_element=&record.get(0).unwrap_or("");
        println!("Third element: {}", third_element.on_yellow());

        let third_element=&record.get(1).unwrap_or("");
        println!("Third element: {}", third_element.on_yellow());

        let third_element=&record.get(2).unwrap_or("");
        println!("Third element: {}", third_element.on_yellow());
        let third_element=&record.get(7).unwrap_or("");
        println!("Third element: {}", third_element.on_yellow());
        // 1---------------------------
        //  Detect anomaly

        let collected_date_str = record.get(13).unwrap(); // Assuming collected_date is at index 12
        let collected_date = match NaiveDate::parse_from_str(collected_date_str, "%Y-%m-%d") {
            Ok(date) => date,
            Err(err) => {
                eprintln!("Failed to parse collected_date: {}", err);
                process::exit(1);
            }
        };

        let processor:csv_row_processor::CsvRowProcessor= csv_row_processor::CsvRowProcessor::new(
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


        // 2------------------------
        // Use regex



        //3---------------------------
        //print Information

        let third_element=&record.get(2).unwrap_or("");
        println!("Third element: {}", third_element.on_yellow());
        println!("------------------------------------");
        println!(" Original col :\n {:?}", record);
        let mut new_col = String::new();
        println!("Enter the new column value for row {:?}", record);
        io::stdin()
            .read_line(&mut new_col)
            .expect("Failed to read input");
        let new_col = new_col.trim().to_string();
        let csv_row=CsvRow::new(record,new_col);
        rows.push(csv_row);
    }

    //------------------------------------
    // Write outputs

    
    for row in rows{
        let mut record=row.row.clone();
        record.push_field(&row.new_col);
        let res=match wrt.write_record(&record) {
            Ok(res)=> res,
            Err(err) => {
                eprintln!("Failed to create output file: {}", err);
                process::exit(1);
            }
        };

    }



    
    
}
