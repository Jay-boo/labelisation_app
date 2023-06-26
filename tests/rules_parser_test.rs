use labelisation::csv_row_processor;

#[cfg(test)]
mod tests {
    use labelisation::csv_row_processor::{self, Row};

    #[test]
    fn split_string_on_patterns_test() {
        println!(r"   SPLIT TEST /!\ ");
        let res:Vec<String>=csv_row_processor::split_string_on_patterns(
            String::from(">0"),
            [String::from(">"),String::from("<"),String::from(">="),String::from("<="),String::from("==")].to_vec(),
            true
        );
        assert_eq!(res,[">","","0"]);

        // --------------------------------------------
        // Conditional rules parser test
        let res:Vec<String>=csv_row_processor::split_string_on_patterns(
            String::from("estate_type==Parking"),
            [String::from(">"),String::from("<"),String::from(">="),String::from("<="),String::from("==")].to_vec(),
            false
        );

        assert_eq!(res,["==","estate_type","Parking"]);

        let res:Vec<String>=csv_row_processor::split_string_on_patterns(
            String::from("area<40"),
            [String::from(">"),String::from("<"),String::from(">="),String::from("<="),String::from("==")].to_vec(),
            false
        );

        assert_eq!(res,["<","area","40"]);
    }




    #[test]
    fn detect_warning_test(){
        println!(r"  DETECT WARNING TEST /!\ ");

        let row1:Row=Row{
            announcement_id:String::from("slg_182207265"),
            type_source:String::from("sales"),
            estate_type:String::from("Appartement"),
            price: 314500.0,
            price_m2: 14473.0,
            area:21.73,
            room_count:2,
            meuble:false,
            postal_code:String::from("75020"),
            data_source:String::from("seloger"),
            type_owner:1,
            ..Default::default()
            


        };
        let mut processor=csv_row_processor::CsvRowProcessor::new(
            csv_row_processor::Row{
                area:40.0,
                ..Default::default()
            }

        );
        processor.detect_warnings();
        println!("Warning score : {}",processor.warn_score);


    }
}
