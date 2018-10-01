use models::*;
use std::collections::HashMap;
use actix_web::dev::Params;
use actix_web::HttpRequest;
use actix_web::HttpMessage;
use database::DBParams;
use ini;
use std;
use errors::*;

pub fn parse_configs(conf: &ini::Ini) -> DBParams {
    let engine;
    let mut host = &String::new();;
    let mut port = &String::new();;
    let mut user = &String::new();;
    let mut password = &String::new();;
    let mut db_name = &String::new();;

    match conf.section(Some("DEFAULT".to_owned())) {
         Some(section) => {
                match section.get("driver") {
                     Some(value) => {
                        engine = value;
                     },
                     None => {
                        error!("{}","Could not find value 'driver' in section 'DEFAULT'");
                        std::process::exit(1);               
                     }
                } 
         },
         None => {
              error!("{}","Could not find section - DEFAULT");
              std::process::exit(1);   
         }   
    }

    if engine == "postgres" {
        match conf.section(Some("POSTGRES".to_owned())) {
             Some(section) => {
                //host
                match section.get("host") {
                        Some(value) => {
                                host=value;
                        } None => {
                                error!("{}","Could not find value 'host' in section 'POSTGRES'");
                                std::process::exit(1); 
                        }
                }

                //port
                match section.get("port") {
                        Some(value) => {
                                port=value;
                        } None => {
                                error!("{}","Could not find value 'port' in section 'POSTGRES'");
                                std::process::exit(1); 
                        }
                }

                //user
                match section.get("user") {
                        Some(value) => {
                                user=value;
                        } None => {
                                error!("{}","Could not find value 'user' in section 'POSTGRES'");
                                std::process::exit(1); 
                        }
                }

                //pass
                match section.get("password") {
                        Some(value) => {
                                password=value;
                        } None => {
                                error!("{}","Could not find value 'password' in section 'POSTGRES'");
                                std::process::exit(1); 
                        }
                }

                //db name
                match section.get("database-name") {
                        Some(value) => {
                                db_name =value;
                        } None => {
                                error!("{}","Could not find value 'database-name' in section 'POSTGRES'");
                                std::process::exit(1); 
                        }
                }                                                                

             },
             None => {
                
             }   
        }
    }else{
        match conf.section(Some("ORACLE".to_owned())) {
             Some(section) => {
                //host
                match section.get("host") {
                        Some(value) => {
                                host=value;
                        } None => {
                                error!("{}","Could not find value 'host' in section 'ORACLE'");
                                std::process::exit(1); 
                        }
                }

                //port
                match section.get("port") {
                        Some(value) => {
                                port=value;
                        } None => {
                                error!("{}","Could not find value 'port' in section 'ORACLE'");
                                std::process::exit(1); 
                        }
                }

                //user
                match section.get("user") {
                        Some(value) => {
                                user=value;
                        } None => {
                                error!("{}","Could not find value 'user' in section 'ORACLE'");
                                std::process::exit(1); 
                        }
                }

                //pass
                match section.get("password") {
                        Some(value) => {
                                password=value;
                        } None => {
                                error!("{}","Could not find value 'password' in section 'ORACLE'");
                                std::process::exit(1); 
                        }
                }

                //service-name
                match section.get("service-name") {
                        Some(value) => {
                                db_name =value;
                        } None => {
                                error!("{}","Could not find value 'service-name' in section 'ORACLE'");
                                std::process::exit(1); 
                        }
                }
             },
             None => {
                     
             }
        }    
    }

    let db_params = DBParams {
            engine: engine.clone(),
            host: host.clone(),
            port: port.clone(),
            user: user.clone(),
            pass: password.clone(),
            db_name: db_name.clone()
    };

    return db_params;        
}

pub fn to_vec_string(data_str: &Option<String>, seperator: &str) -> Vec<String> {
    match data_str {
        &Some(ref data) => {
            let split = data.split(seperator);
            let mut vec: Vec<String> = Vec::new();
            for s in split {
                if s.len() != 0 {
                    vec.push(String::from(s));
                }
            }
            return vec;
        },
        &None => {
            Vec::new()
        }
    }
}

pub fn get_vec_str_from_param(params: &Params, key: &str)-> Vec<String> {
    let mut values : Vec<String> = Vec::new();
    for param in params.iter() {
        if param.0 == key {
            values.push(String::from(param.1.as_ref()));
        }
    }

    return values;

}

pub fn get_vec_i32_from_param(params: &Params, key: &str)-> Result<Vec<i32>> {
    let mut values : Vec<i32> = Vec::new();
    for param in params.iter() {
        if param.0 == key {
            let value_str : &str = param.1.as_ref();
            match value_str.parse::<i32>(){
                Ok(value) => {
                    values.push(value);
                },
                Err(_error) => {
                    return Err(format!("{value} is not int",value=value_str).into());
                }
            }            
        }
    }
    return Ok(values);
}

pub fn to_vec_i64(data_str: &Option<String>, seperator: &str) -> Vec<i64> {
    match data_str {
        &Some(ref data) => {
            let split = data.split(seperator);
            let mut vec: Vec<i64> = Vec::new();
            for s in split {
                if s.len() != 0 {
                    let number;
                    let number_option = s.parse::<i64>();
                    match number_option {
                        Ok(val) => {number=val},
                        Err(error) => {
                            error!("{}",error);
                            number = 0;
                        }
                    }

                    vec.push(number);
                }
            }
            return vec;
        },
        &None => {
            Vec::new()
        }
    }
}

pub fn to_pmid_list(pmid_option: Option<String>) -> Vec<String> {
     let mut pmids: Vec<String> = Vec::new();
     match pmid_option {
        Some(value) => {
            //check if this pmid is joined or seperate
            if value.contains(","){
                //split pmid into subsequent strings
                let split_pmids = value.split(",");
                for pmid in split_pmids {
                    let trimmed_pmid = pmid.trim();
                    //check if it is not empty string
                    if !trimmed_pmid.is_empty() {
                        pmids.push(String::from(trimmed_pmid));
                    }
                }
            }else{
                if !value.is_empty() {
                    pmids.push(value);
                }
            }            
        },
        None => {
            
        }
    }
    return pmids;
}

pub fn remove_duplicates(items: &Vec<String>) -> Vec<String>{
    let mut unique_items: Vec<String> = Vec::new();
    for item in items{
        if !unique_items.contains(item){
            unique_items.push(item.clone());
        }
    }
    return unique_items;
}

pub fn str_vec_to_str(items: &Vec<String>) -> String{
    let mut items_str = String::new();
    for (index,item) in items.iter().enumerate() {
        if index == 0 {
            items_str = item.clone();
        }else{
            items_str = format!("{prev_str},{curr_str}",prev_str=items_str,curr_str=item);
        }
    }

    return items_str;

}

/**
pub fn str_vec_to_str_with_sep(items: &Vec<String>,seperator: String) -> String{
    let mut items_str = String::new();
    for (index,item) in items.iter().enumerate() {
        if index == 0 {
            items_str = item.clone();
        }else{
            items_str = format!("{prev_str}{seperator}{curr_str}",prev_str=items_str,seperator=seperator,curr_str=item);
        }
    }

    return items_str;

}**/

pub fn taxons_to_tuple_str(taxons: &Vec<i32>) -> String {
    let mut taxons_str = String::new();
    for (index,taxon) in taxons.iter().enumerate() {
        let taxon_str = format!("{taxon}",taxon=taxon);
        if index == 0{
            taxons_str = taxon_str;
        }else{
            taxons_str = format!("{existing_string},{current_str}",existing_string=taxons_str,current_str=taxon_str);
        }
    }
    return taxons_str;
}

pub fn to_bool(bool_str: Option<String>) -> bool{
    match bool_str {
        Some(value) => {
            return value == "T"
        },
        None => {
            return false;
        }
    }
}

pub fn to_postgres_array_str(items: &Vec<String>) -> String{
    let mut items_str = String::new();
    for (index,item) in items.iter().enumerate() {
        if index == 0 {
            items_str = format!(r#"'{curr_str}'"#,curr_str=item);
        }else{
            let curr_str = format!(r#"'{curr_str}'"#,curr_str=item);
            items_str = format!("{prev_str},{curr_str}",prev_str=items_str,curr_str=curr_str);
        }
    }
       
    return format!("array[{items}]",items=items_str);

}

pub fn get_source(source_type: Option<String>) -> Option<Source> {
    match source_type {
        Some(value) => {

            let val_slice: &str = &value[..];

            match val_slice {
                "hprd" => Some(Source{name:Some(String::from("HPRD")),label: Some(String::from("hprd")),url:Some(String::from("http://www.hprd.org/"))}),
                "pelm" => Some(Source{name:Some(String::from("phospho.ELM")),label:Some(String::from("pelm")),url:Some(String::from("http://phospho.elm.eu.org/"))}),
                "psp" => Some(Source{name:Some(String::from("PSP")),label:Some(String::from("psp")),url:Some(String::from("http://www.phosphosite.org/"))}),
                "p3db" => Some(Source{name:Some(String::from("p3DB: Plant Protein Phosphorylation DataBase")),label: Some(String::from("p3db")),url:Some(String::from("http://www.p3db.org/"))}),
                "pgrd" => Some(Source{name:Some(String::from("PhosphoGrid")),label:Some(String::from("pgrd")),url:Some(String::from("http://www.phosphogrid.org/"))}),
                "phat" => Some(Source{name:Some(String::from("PhosPhAt")),label:Some(String::from("phat")),url:Some(String::from("http://phosphat.uni-hohenheim.de/"))}),
                "pro" => Some(Source{name:Some(String::from("PRO")),label:Some(String::from("pro")),url:Some(String::from("http://pir.georgetown.edu/pro/pro.shtml"))}),
                "uniprot" => Some(Source{name:Some(String::from("UniProt")),label:Some(String::from("uniprot")),url:Some(String::from("http://www.uniprot.org/"))}),
                "rlimsp" => Some(Source{name:Some(String::from("RLIMS-P")),label:Some(String::from("rlimsp")),url:Some(String::from("http://research.bioinformatics.udel.edu/rlimsp/"))}),
                "efip" => Some(Source{name:Some(String::from("eFIP")),label:Some(String::from("efip")),url:Some(String::from("http://research.bioinformatics.udel.edu/eFIPonline/index.php"))}),
                "pomb" => Some(Source{name:Some(String::from("PomBase")),label:Some(String::from("pomb")),url:Some(String::from("https://www.pombase.org/"))}),
                "npro" => Some(Source{name:Some(String::from("neXtProt")),label:Some(String::from("npro")),url:Some(String::from("www.nextprot.org"))}),
                "sign" => Some(Source{name:Some(String::from("Signor")),label:Some(String::from("sign")),url:Some(String::from("signor.uniroma2.it"))}),
                "sno" => Some(Source{name:Some(String::from("dbSNO")),label:Some(String::from("sno")),url:Some(String::from(""))}),
                _ => None,
            }
        },
        None => None

    }
}

pub fn get_ptm_event_label(ptm_name: &str) -> Option<String>{
    match ptm_name {
        "acetylation" => Some(String::from("ac")),
        "n-glycosylation" => Some(String::from("gn")),
        "o-glycosylation" => Some(String::from("go")),
        "c-glycosylation" => Some(String::from("gc")),
        "s-glycosylation" => Some(String::from("gs")),
        "methylation" => Some(String::from("me")),
        "myristoylation" => Some(String::from("my")),
        "phosphorylation" => Some(String::from("p")),
        "sumoylation" => Some(String::from("su")),
        "ubiquitination" => Some(String::from("ub")),
        "interaction" => Some(String::from("i")),
        "s-nitrosylation" => Some(String::from("sno")),
        _ => None
    }
}

pub fn default_ptm_labels() -> Vec<String>{
    let mut ptm_labels:Vec<String> = Vec::new();
    ptm_labels.push(String::from("ac"));
    ptm_labels.push(String::from("gn"));
    ptm_labels.push(String::from("go"));
    ptm_labels.push(String::from("gc"));
    ptm_labels.push(String::from("gs"));
    ptm_labels.push(String::from("me"));
    ptm_labels.push(String::from("my"));
    ptm_labels.push(String::from("p"));
    ptm_labels.push(String::from("su"));
    ptm_labels.push(String::from("ub"));
    ptm_labels.push(String::from("i"));
    ptm_labels.push(String::from("sno"));
    return ptm_labels;
}

pub fn query_substrates_to_tuple_str(query_substrates: &Vec<QuerySubstrate>) -> String {
    let mut query_substrates_str = String::new();
    for (index,substrate) in query_substrates.iter().enumerate() {
        let substrate_str = format!("('{substrate_ac}','{residue}','{position}')",substrate_ac=substrate.substrate_ac,
                                                                            residue=substrate.site_residue,
                                                                            position=substrate.site_position);    
        if index == 0{
            query_substrates_str = substrate_str;
        }else{
            query_substrates_str = format!("{existing_string},{current_str}",existing_string=query_substrates_str,current_str=substrate_str);
        }
    }
    return query_substrates_str;
}

pub fn calculate_score_batch_ptm_enzymes(num_substrates: Vec<i64>,sources: &Vec<String>,pmids: &Vec<String>) -> i64 {

    //build the pmid stats map
    let mut pmid_stats: HashMap<String,i64> = HashMap::new();

    for (index,pmid) in pmids.iter().enumerate() {
        let substrate_count = num_substrates.get(index);
        match substrate_count {
            Some(value) => {
                pmid_stats.insert(pmid.clone(),value.clone());
            },
            None => {
                pmid_stats.insert(pmid.clone(),0);
            }
        }        
    }

    let score = calculate_score(&pmid_stats,&pmids,&sources);

    return score;

}

pub fn calculate_score(pmid_stats: &HashMap<String,i64>,pmids: &Vec<String>, sources: &Vec<String> ) -> i64 {
    //sn
    let sn;
    if sources.len() >= 2 {
        sn = 1;
    }else{
        sn = 0;
    }

    //sq
    let sq;
    if sources.len() == 1 && sources.contains(&String::from("rlimsp")){
        sq = 0;
    }else if has_preferred_sources(sources) {
        sq = 2;
    }else{
        sq = 1;
    }

    //sp
    let sp: i64;
    let pmid_count = pmids.len();
    let has_non_large_scale = has_non_large_scale(pmids,pmid_stats);

    if pmid_count >= 2 && has_non_large_scale {
        sp = 1;
    }else if has_non_large_scale {
        sp = 0;
    }else{
        sp = -1;
    }

    return sn + sq + sp;
}

fn has_preferred_sources(sources: &Vec<String>) -> bool {
    let preferred_sources = vec!["p3db", "pelm", "pgrd", "phat", "pomb", "psp", "uniprot", "pro", "npro"]; 
    
    for source in sources {
        if preferred_sources.contains(&source.as_str()){
            return true;
        }
    }

    return false;

}

fn has_non_large_scale(pmids: &Vec<String>, pmid_stats: &HashMap<String,i64>) -> bool{
    let threshold = 10;
    for pmid in pmids {
        match pmid_stats.get(pmid) {
            Some(count) => {
                if count <= &threshold {
                    return true;
                }
            },
            None => {
                error!("Could not find in has_non_large_scale {}. This is serious and needs to be checked.",pmid);
            }
        }
        
    }

    return false;

}

pub fn get_accept_header_value(req: &HttpRequest<super::State>) -> String{
    let content_header;
    match req.headers().get("ACCEPT") {
        Some(value) => {
            match value.to_str() {
                Ok(header_value) => {
                    if header_value.contains("*/*") {
                        content_header = "application/json";
                    }else{
                        content_header = header_value;
                    }
                }
                Err(_) => {
                    content_header = "application/json";
                }
            }
        },
        None => {
            content_header = "application/json";
        }
    }

    return String::from(content_header);

}