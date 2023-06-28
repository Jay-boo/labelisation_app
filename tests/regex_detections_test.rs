use labelisation::regex_sets;
use colored::*;
use regex::{RegexSet,Regex};

#[cfg(test)]
mod tests {
    use std::iter::zip;

    use colored::{Colorize, Color};
    use labelisation::regex_sets;
    use regex::RegexSet;


    #[test]
    fn regex_detection_test(){
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
        

        let all_sets:Vec<&RegexSet>=vec![&regex_sets::PRICE_SET,&regex_sets::ESTATE_TYPE_SET,&regex_sets::AREA_SET,&regex_sets::ROOM_COUNT_SET,&regex_sets::MEUBLE_SET];
        let colors:Vec<Color>=vec![Color::Green,Color::Red,Color::Yellow,Color::Blue,Color::TrueColor { r:160 , g: 32, b: 240 }];
        let mut modified_description=description.clone();

        for (set,color) in zip(all_sets.iter(),colors.iter()){
            for pattern in set.patterns(){
                println!("-------{}",pattern);
                for mat in regex::Regex::new(pattern).unwrap().find_iter(&description){
                    let matched_str=&description[mat.start()..mat.end()];
                    modified_description=modified_description.replace(
                        matched_str,
                        &matched_str.color(*color).bold().to_string());

                }

                // if let Some(mat) = re.find(&description) {
                //     let matched_str=&description[mat.start()..mat.end()];
                //     println!("matched_str: {}, from:{} to : {}",matched_str);
                //     description=description.replace(
                //         matched_str,
                //         &matched_str.color(*color).bold().to_string()
                //     );
                // // Match found, you can do something with `mat` here
                // } else {
                // // No match found for the current pattern
                //     continue;
                // }

            }

        }
    println!("{}",modified_description);
    }

}
