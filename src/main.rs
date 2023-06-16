extern crate csv;
use std::error::Error;
use std::io;
use std::process;





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
    // let mut csv_path=String::new();
    // println!("Enter path to CSV file");
    // io::stdin().read_line(&mut csv_path).expect("failed to read path");
    // let csv_path=csv_path.trim();
    //
    let csv_path="src/.data/df_usable_profiteroles.csv";
    let mut reader = match csv::Reader::from_path(csv_path){
        Ok(reader) => reader,
        Err(err) => {
            eprintln!("Failed to open csv file: {}",err);
            process::exit(1);
        }
    };

    let output_path = format!("src/.data/new_df_usable_profiteroles.csv");

    // Create the output file
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
