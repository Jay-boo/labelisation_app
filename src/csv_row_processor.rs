use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::{from_str,Value, to_string};
use chrono::NaiveDate;






#[derive(Default)]
pub struct Row{
    pub announcement_id: String,
    pub type_source: String,
    pub estate_type: String,
    pub price : f32,
    pub price_m2 : f32,
    pub area:f32,
    pub room_count:i32,
    pub meuble:bool,
    pub postal_code:String,
    pub lat:String,
    pub lon:String,
    pub description:String,
    pub collected_date:NaiveDate,
    pub data_source:String,
    pub type_owner: i32

}

pub struct CsvRowProcessor {
    row: Row,
    rules:JsonRule,
    pub warn_score: i32,
    pre_response :Row
    
}


#[derive(serde::Serialize,serde::Deserialize)]
struct JsonRule{
    pub global_rules:Vec<HashMap<String,String>>,
    pub conditional_rules:Vec<HashMap<String,String>>
} 


 impl CsvRowProcessor{
    pub fn new(row:Row) -> CsvRowProcessor{
        let data = r#"
            {
                "global_rules": [
                    {"var": "area", "then": ">0"},
                    {"var": "area", "then": "<150"},
                    {"var": "room_count", "then": ">0"},
                    {"var": "room_count", "then":  "<5"},
                    {"var": "price", "then": ">0"},
                    {"var": "price", "then":"<600000"}
                ],
                "conditional_rules": [
                    {"condition": "estate_type==Parking", "then":"area<40"},
                    {"condition": "estate_type==Autre", "then": "area>40" },
                    {"condition": "estate_type==Autre", "then": "area<105"},
                    {"condition": "estate_type==Appartement", "then":"area<60"},
                    {"condition": "meuble==true", "then": "area<100"},
                    {"condition": "room_count>3", "then": "area>60"},
                    {"condition": "room_count>1", "then": "area>18"},
                    {"condition": "room_count==1", "then": "area<50"},
                    {"condition": "type_owner==1", "then": "area<90"},
                    {"condition": "type_source==rentals", "then": "area<70"},
                    {"condition": "type_owner==sales", "then": "area>15"},
                    {"condition": "estate_type==Parking", "then": "meuble==false"},
                    {"condition": "price>300000", "then": "area>20"},
                    {"condition": "estate_type==Parking", "then": "price<100000"},
                    {"condition": "estate_type==Autre", "then": "price>150000"},
                    {"condition": "type_owner==1", "then": "price<50000"},
                    {"condition": "type_source==rentals", "then": "price<2000"},
                    {"condition": "meuble==true", "then": "room_count<4"},
                    {"condition": "type_source==sales", "then": "estate_type!=Parking"},
                    {"condition": "type_source==rentals", "then": "type_owner==0"}
                ]
            }"#;

        let rules: JsonRule = match serde_json::from_str(data) {
            Ok(value) => value,
            Err(err) => {panic!("Cant deal with {}",err)},
        };
        // for (key, value) in &rules.global_rules[0] {
        //     println!("{}: {}", key, value);
        // }
        CsvRowProcessor {
            row: row,
            rules:rules,
            warn_score: 0,
            pre_response: Row{
                ..Default::default()
            }
        }
    }

    pub fn  detect_warnings( &mut self){
        
        println!("----Parsing Global rules----");
        for global_rule in self.rules.global_rules.iter(){
            let _col= self.get_field_value(&global_rule["var"]).unwrap();
            let input_string:&String=&global_rule["then"];
            let _condition=split_string_on_patterns(
                input_string.to_string(),
                [String::from(">"),String::from("<"),String::from(">="),String::from("<="),String::from("==")].to_vec(),
                true
            );
            println!("col value: {}",_col);
            let _sucess: bool =match global_rule["var"].as_str(){
                "area"=>match _condition.get(0).unwrap().as_str(){
                    ">"=> _col.parse::<f32>().unwrap() > _condition.get(2).unwrap().parse::<f32>().unwrap(),
                    "<"=> _col.parse::<f32>().unwrap() < _condition.get(2).unwrap().parse::<f32>().unwrap(),
                    _=> panic!("The code panic becaused of _col value :{}",_col.to_string())
                } ,
                "room_count"=> match _condition.get(0).unwrap().as_str(){
                    ">"=> _col.parse::<i32>().unwrap() > _condition.get(2).unwrap().parse::<i32>().unwrap(),
                    "<"=> _col.parse::<i32>().unwrap() < _condition.get(2).unwrap().parse::<i32>().unwrap(),
                    _=> panic!("The code panic becaused of _col value :{}",_col.to_string())
                },

                "price"=> match _condition.get(0).unwrap().as_str(){
                    ">"=> _col.parse::<f32>().unwrap() > _condition.get(2).unwrap().parse::<f32>().unwrap(),
                    "<"=> _col.parse::<f32>().unwrap() < _condition.get(2).unwrap().parse::<f32>().unwrap(),
                    _=> panic!("The code panic becaused of _col value  :{}",_col.to_string())
                }

                _=> panic!("unknown {}",global_rule["var"])

            };
            if _sucess{self.warn_score+=3;}

        println!("----Parsing conditional rules----");
        for conditional_rule in self.rules.conditional_rules.iter(){




            let _filter=split_string_on_patterns(
                    conditional_rule["condition"].to_string(),
                    [String::from(">"),String::from("<"),String::from(">="),String::from("<="),String::from("==")].to_vec(),
                    false

                );

            let _col_filter:Option<String>=self.get_field_value(_filter.get(1).unwrap());
            let _filtered:bool=match _filter.get(1).unwrap().as_str(){
                    "estate_type"=>match _filter.get(0).unwrap().as_str(){
                        "=="=> _col_filter.unwrap().to_string() == _filter.get(2).unwrap().to_string(),
                        "!="=>  _col_filter.unwrap().to_string() != _filter.get(2).unwrap().to_string(),
                        _=> panic!("unknown {}",_filter.get(0).unwrap().as_str())
                    },
                        
                    "meuble"=>match _filter.get(0).unwrap().as_str(){
                        "=="=> _col_filter.unwrap().parse::<bool>().unwrap() == _filter.get(2).unwrap().parse::<bool>().unwrap(),
                        "!="=>  _col_filter.unwrap().parse::<bool>().unwrap() != _filter.get(2).unwrap().parse::<bool>().unwrap(),
                        _=> panic!("unknown {}",_filter.get(0).unwrap().as_str())
                    }
                    "room_count"=>match _filter.get(0).unwrap().as_str(){
                        "=="=> _col_filter.unwrap().parse::<i32>() == _filter.get(2).unwrap().parse::<i32>(),
                        "!="=>  _col_filter.unwrap().parse::<i32>() != _filter.get(2).unwrap().parse::<i32>(),
                        ">"=>  _col_filter.unwrap().parse::<i32>().unwrap() > _filter.get(2).unwrap().parse::<i32>().unwrap(),
                        "<"=>  _col_filter.unwrap().parse::<i32>().unwrap() < _filter.get(2).unwrap().parse::<i32>().unwrap(),
                        _=> panic!("unknown {}",_filter.get(0).unwrap().as_str())
                    },
                    "type_owner"=>match _filter.get(0).unwrap().as_str(){
                        "=="=> _col_filter.unwrap().parse::<i32>() == _filter.get(2).unwrap().parse::<i32>(),
                        "!="=>  _col_filter.unwrap().parse::<i32>() != _filter.get(2).unwrap().parse::<i32>(),
                        ">"=>  _col_filter.unwrap().parse::<i32>().unwrap() > _filter.get(2).unwrap().parse::<i32>().unwrap(),
                        "<"=>  _col_filter.unwrap().parse::<i32>().unwrap() < _filter.get(2).unwrap().parse::<i32>().unwrap(),
                        _=> panic!("unknown {}",_filter.get(0).unwrap().as_str())
                    },

                    "type_source"=>match _filter.get(0).unwrap().as_str(){
                        "=="=> _col_filter.unwrap().to_string() == _filter.get(2).unwrap().to_string(),
                        "!="=>  _col_filter.unwrap().to_string() != _filter.get(2).unwrap().to_string(),
                        _=> panic!("unknown {}",_filter.get(0).unwrap().as_str())
                    },
                    "price"=>match _filter.get(0).unwrap().as_str(){
                        "=="=> _col_filter.unwrap().parse::<i32>() == _filter.get(2).unwrap().parse::<i32>(),
                        "!="=>  _col_filter.unwrap().parse::<i32>() != _filter.get(2).unwrap().parse::<i32>(),
                        ">"=>  _col_filter.unwrap().parse::<i32>().unwrap() > _filter.get(2).unwrap().parse::<i32>().unwrap(),
                        "<"=>  _col_filter.unwrap().parse::<i32>().unwrap() < _filter.get(2).unwrap().parse::<i32>().unwrap(),
                        _=> panic!("unknown {}",_filter.get(0).unwrap().as_str())
                    },
                    _=> panic!("Panic  observing :{} on filter ",_filter.get(1).unwrap().as_str())




                };
            if !_filtered{continue;}
            let input_string:&String=&conditional_rule["then"];
            let _condition=split_string_on_patterns(
                input_string.to_string(),
                [String::from(">"),String::from("<"),String::from(">="),String::from("<="),String::from("==")].to_vec(),
                false
            );
            let _col= self.get_field_value(_condition.get(2).unwrap());
            let _sucess: bool =match _condition.get(1).unwrap().as_str(){
                "area"=>match _condition.get(0).unwrap().as_str(){
                    ">"=> _col.unwrap().parse::<f32>().unwrap() > _condition.get(2).unwrap().parse::<f32>().unwrap(),
                    "<"=> _col.unwrap().parse::<f32>().unwrap() < _condition.get(2).unwrap().parse::<f32>().unwrap(),
                    _=> panic!("The code panic becaused of _col value :{}",_col.unwrap().to_string())
                } ,
                "room_count"=> match _condition.get(0).unwrap().as_str(){
                    ">"=> _col.unwrap().parse::<i32>().unwrap() > _condition.get(2).unwrap().parse::<i32>().unwrap(),
                    "<"=> _col.unwrap().parse::<i32>().unwrap() < _condition.get(2).unwrap().parse::<i32>().unwrap(),
                    _=> panic!("The code panic becaused of _col value :{}",_col.unwrap().to_string())
                },

                "price"=> match _condition.get(0).unwrap().as_str(){
                    ">"=> _col.unwrap().parse::<f32>().unwrap() > _condition.get(2).unwrap().parse::<f32>().unwrap(),
                    "<"=> _col.unwrap().parse::<f32>().unwrap() < _condition.get(2).unwrap().parse::<f32>().unwrap(),
                    _=> panic!("The code panic becaused of _col value  :{}",_col.unwrap().to_string())
                }

                _=> panic!("unknown {}",global_rule["var"])

            };

            if _sucess{self.warn_score+=3;}

                
                

            };


        } 
        


    }
    
    fn detect_known_expression(self) {
        
    }


    fn get_field_value(&self,field_name:&str) -> Option<String>{
        match field_name{
            "announcement_id"=>Some(self.row.announcement_id.to_string()),
            "type_source"=>Some(self.row.type_source.to_string()),
            "estate_type"=>Some(self.row.estate_type.to_string()),
            "price"=>Some(self.row.price.to_string()),
            "price_m2"=>Some(self.row.price_m2.to_string()),
            "area"=>Some(self.row.area.to_string()),
            "room_count"=>Some(self.row.room_count.to_string()),
            "meuble"=>Some(self.row.meuble.to_string()),
            "postal_code"=>Some(self.row.postal_code.to_string()),
            "lat"=>Some(self.row.lat.to_string()),
            "lon"=>Some(self.row.lon.to_string()),
            "description"=>Some(self.row.description.to_string()),
            "collected_date"=>Some(self.row.collected_date.to_string()),
            "data_source"=>Some(self.row.data_source.to_string()),
            "type_owner"=>Some(self.row.type_owner.to_string()),
            _ => None

        }

    }
    pub fn next_row(&mut self,next_row:Row){
        self.row=next_row;
        self.warn_score=0;
        self.pre_response=Row{
            ..Default::default()
        };
    }



}



pub fn split_string_on_patterns(input_string: String, patterns: Vec<String>,_global_rules:bool) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    for pattern in patterns.iter() {
        let parts: Vec<String> = input_string.split(pattern.as_str()).map(String::from).collect();
        // println!("{:?}",parts);
        if parts.len()== 2{
            res.push(pattern.to_string());
            for  val in parts{ res.push(val)}
            break;
        }
    }
    // println!("{:?}", res);
    res
}
// use regex::Regex;
//
// fn main() {
//     let input = "true,false,true,false";
//
//     // Define the pattern to split on (a comma)
//     let pattern = Regex::new(r",").unwrap();
//
//     // Split the input string based on the pattern
//     let substrings: Vec<&str> = pattern.split(input).collect();
//
//     // Convert each substring into a boolean value
//     let bool_values: Vec<bool> = substrings
//         .iter()
//         .map(|substring| match *substring {
//             "true" => true,
//             "false" => false,
//             _ => panic!("Invalid boolean value"),
//         })
//         .collect();
//
//     // Print the resulting boolean values
//     for bool_value in bool_values {
//         println!("Boolean value: {}", bool_value);
//     }
// }

