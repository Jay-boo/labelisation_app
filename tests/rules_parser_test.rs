use labelisation::csv_row_processor;
use regex::{RegexSet, RegexSetBuilder,Regex};
use labelisation::regex_sets;
use colored::*;


#[cfg(test)]
mod tests {
    use colored::Colorize;
    use labelisation::{csv_row_processor::{self, Row}, regex_sets};
    use regex::RegexSet;

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


    #[test]
    fn check_regex_sets(){
        let  mut description:String=String::from("
Appartement 4 pièces 70 m²
Vente Appartement 4 pièces

iad France - Laurence Bisson vous propose : Vincennes // Situé au dernier étage, dans une copropriété entretenue avec ascenseur et Gardien, très lumineux appartement traversant de 4 pièces avec deux balcons. De beaux volumes donnent à cet appartement beaucoup d’élégance.
D’une superficie de 70 m2 environ, il se compose d’une entrée, d’un salon/salle à manger, d’une cuisine équipée, de trois chambres, d’un dressing, d’une salle de bain et d’un WC indépendant.
Une cave complète ce bien.
Possibilité de parking en sus.

La présente annonce immobilière vise 1 lot principal situé dans une copropriété formant 210 lots au total ne faisant l'objet d'aucune procédure en cours et d'un montant de charges d’environ 294 € par mois (soit 3528 € annuel) déclaré par le vendeur.Honoraires d’agence à la charge du vendeur.Information d'affichage énergétique sur ce bien : classe ENERGIE E indice 329 et classe CLIMAT E indice 59.   Les informations sur les risques auxquels ce bien est exposé sont disponibles sur le site Géorisques : www.georisques.gouv.fr.  La présente annonce immobilière a été rédigée sous la responsabilité éditoriale de Mme Laurence Bisson EI (ID 28744), mandataire indépendant en immobilier (sans détention de fonds), agent commercial de la SAS I@D France immatriculé au RSAC de CRETEIL sous le numéro 838834836, titulaire de la carte de démarchage immobilier pour le compte de la société I@D France SAS.
Référence annonce : 1332561
Date de réalisation du diagnostic : 16/05/2022
Les honoraires sont à la charge du vendeur

A propos de la copropriété :
Pas de procédure en cours
Nombre de lots : 250
Charges prévisionnelles annuelles : 3528 €

Montant estimé des dépenses annuelles d'énergie pour un usage standard : entre 1310 € et 1810 € par an. Prix moyens des énergies indexés sur l'année 2021 (abonnements compris)
Consommation énergie primaire : 329 kWh/m²/an.
Consommation énergie finale : Non communiquée.");
        

        let all_sets:Vec<&RegexSet>=vec![&regex_sets::PRICE_SET,&regex_sets::ESTATE_TYPE_SET];

        for set in all_sets{
            for pattern in set.patterns(){
                let re = regex::Regex::new(pattern).unwrap();
                if let Some(mat) = re.find(&description) {
                    let matched_str=&description[mat.start()..mat.end()];
                    description=description.replace(
                        matched_str,
                        &matched_str.green().bold().to_string()
                    );
                    println!("ONE MATCH FOUNDED");
                // Match found, you can do something with `mat` here
                } else {
                // No match found for the current pattern
                    continue;
                }

            }

        }
    println!("{}",description);
    }
}
