use models::*;
use errors::*;
use misc;
use postgres;
use oracle;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use query_builder;

#[derive(Clone)]
pub struct DBParams {
        pub engine: String,
        pub host: String,
        pub port: String,
        pub user: String,
        pub pass: String,
        pub db_name: String
}

#[derive(Debug)]
pub enum Engine {
    Postgres,
    Oracle
}

pub trait MyRow<'a>{
    fn get_string(&self,column_name: &str) -> Option<String>;
    fn get_string_unwrapped(&self,column_name: &str) -> String;
    fn get_i64(&self,column_name: &str) -> Option<i64>;
    fn get_bool(&self,column_name: &str) -> Option<bool>;
}

impl<'a> MyRow<'a> for postgres::rows::Row<'a> {
    fn get_string(&self,column_name: &str) -> Option<String>{
        return self.get(column_name);
    }

    fn get_string_unwrapped(&self,column_name: &str) -> String {
        return self.get(column_name);
    }

    fn get_i64(&self,column_name: &str) -> Option<i64>{
        let value: Option<i64> = self.get(column_name);
        return value;
    }

    fn get_bool(&self,column_name: &str) -> Option<bool>{
        return self.get(column_name);
    }
}

impl<'a> MyRow<'a> for oracle::Row {
    fn get_string(&self,column_name: &str) -> Option<String>{
        let result = self.get(column_name);
        match result {
            Ok(val) => {
                return Some(val);
            },
            Err(_error) => {
                return None;
            }
        };
    }

    fn get_string_unwrapped(&self,column_name: &str) -> String {
        let result = self.get(column_name);
        match result {
            Ok(val) => {
                return val;
            },
            Err(error) => {
                panic!("{}",error);
            }
        };
    }

    fn get_i64(&self,column_name: &str) -> Option<i64>{
        let result = self.get(column_name);
        match result {
            Ok(val) => {
                return Some(val);
            },
            Err(_error) => {
                return None;
            }
        };
    }

    fn get_bool(&self,column_name: &str) -> Option<bool>{
        let result = self.get(column_name);
        match result {
            Ok(val) => {
                return Some(val);
            },
            Err(_error) => {
                return None;
            }
        };
    }
}


pub struct Connection {
    pub engine: Engine,
    pub pg_conn: Option<postgres::Connection>,
    pub oracle_conn: Option<oracle::Connection>
}

pub fn connect(db_params: &DBParams) -> Result<Connection> {
    if db_params.engine == "oracle" {
        let connect_string = format!("{host}:{port}/{db_name}",host=db_params.host,port=db_params.port,db_name=db_params.db_name);
        let connect_result = oracle::Connection::connect(&db_params.user, &db_params.pass, &connect_string, &[]);
        match connect_result {
            Ok(conn) => {
                let connection = Connection {
                    engine: Engine::Oracle,
                    pg_conn: None,
                    oracle_conn: Some(conn)
                };
                return Ok(connection);
            },
            Err(error) => {
                error!("{}",error);
                error!("Connect string - {}",connect_string);
                return Err(format!("{}",error).into());
            }
        }
    }else{
        let connect_string = format!("postgres://{user}:{pass}@{host}:{port}/{database}",user=db_params.user,
                                     pass=db_params.pass,
                                     host=db_params.host,
                                     port=db_params.port,
                                     database=db_params.db_name
        );
        let connect_result = postgres::Connection::connect(connect_string.clone(),postgres::TlsMode::None);
        match connect_result {
            Ok(conn) => {
                let connection = Connection {
                    engine: Engine::Postgres,
                    pg_conn: Some(conn),
                    oracle_conn: None
                };
                return Ok(connection);
            },
            Err(error) => {
                error!("{}",error);
                error!("Connect string - {}",connect_string);
                return Err(format!("{}",error).into());
            }
        }
    }
}

macro_rules! execute_query {
    ($builder_func:ident,$conn:ident,$query_str:ident,$params:expr) => (
            match $conn.engine {
                Engine::Postgres => {
                    match $conn.pg_conn {
                        Some(ref pg_conn) => {
                            let rows_result = pg_conn.query(&$query_str,$params);
                            match rows_result {
                                Ok(rows) => {
                                    //check if we have any results
                                    if !rows.is_empty() {
                                        //get the first row
                                        let row = rows.get(0);
                                        let info = $builder_func(&row)?;
                                        Ok(Some(info))
                                    }else{
                                        Ok(None)
                                    }
                                },
                                Err(ref error) => {
                                    Err(format!("{}",error).into())
                                }
                            }
                        },
                        None => {
                            //Invalid database engine
                            let message = String::from("Postgres engine is None");
                            error!("{}",message);
                            Err(message.into())
                        }
                    }
                },

                Engine::Oracle => {
                    match $conn.oracle_conn {
                        Some(ref oracle_conn) => {
                            let query_result = oracle_conn.query(&$query_str,$params);
                            match query_result {
                                Ok(rows_result) => {
                                    for row_result in rows_result {
                                        match row_result {
                                            Ok(row) => {
                                                let info = $builder_func(&row)?;
                                                return Ok(Some(info));
                                            },
                                            Err(error) => {
                                                error!("{}",error);
                                                return Err(format!("{}",error).into());
                                            }
                                        }
                                    }

                                    //if we have reached here, then that means row count was zero so we return error
                                    Ok(None)

                                },
                                Err(error) => {
                                    Err(format!("{}",error).into())
                                }
                            }
                        },
                        None => {
                            //Invalid database engine
                            let message = String::from("Oracle engine is None");
                            error!("{}",message);
                            Err(message.into())
                        }
                    }

                }
            }
    )
}

macro_rules! execute_query_bulk {
    ($build_item:ident, $conn:ident,$query_str:ident,$list:ident,$params:expr) => (
            match $conn.engine {
                Engine::Postgres => {

                    match $conn.pg_conn {
                        Some(ref pg_conn) => {
                            //execute the query                    
                            let rows_result = pg_conn.query(&$query_str,$params);
                            match rows_result {
                                Ok(rows) => {
                                    for row in rows.iter() {
                                        //bult the item from row
                                        let item = $build_item(&row)?;
                                        $list.push(item);
                                    }
                                },
                                Err(error) => {
                                    return Err(format!("{}",error).into());
                                }
                            };
                        },
                        None => {
                            let message = String::from("Postgres engine is None");
                            error!("{}",message);
                            return Err(message.into());
                        }
                    }
                },
                Engine::Oracle => {
                    match $conn.oracle_conn {
                        Some(ref oracle_conn) => {
                            //execute the query
                            let query_result = oracle_conn.query(&$query_str,$params);
                            match query_result {
                                Ok(rows_result) => {
                                    for row_result in rows_result {
                                        match row_result {
                                            Ok(row) => {
                                                // Get the item from the row
                                                let item = $build_item(&row)?;
                                                $list.push(item);
                                            },
                                            Err(error) => {
                                                error!("{}",error);
                                                return Err(format!("{}",error).into());
                                            }
                                        }
                                    };

                                },
                                Err(error) => {
                                    return Err(format!("{}",error).into());
                                }
                            }
                        },
                        None => {
                            //Invalid database engine
                            let message = String::from("Oracle engine is None");
                            error!("{}",message);
                            return Err(message.into());
                        }
                    }
                }
            }
    );
}

macro_rules! execute_query_callback {
    ($callback:ident, $conn:ident,$query_str:ident,$params:expr) => (
            match $conn.engine {
                Engine::Postgres => {

                    match $conn.pg_conn {
                        Some(ref pg_conn) => {
                            //execute the query                    
                            let rows_result = pg_conn.query(&$query_str,$params);
                            match rows_result {
                                Ok(rows) => {
                                    for row in rows.iter() {
                                        $callback(&row);
                                    }
                                },
                                Err(error) => {
                                    return Err(format!("{}",error).into());
                                }
                            };
                        },
                        None => {
                            let message = String::from("Postgres engine is None");
                            error!("{}",message);
                            return Err(message.into());
                        }
                    }
                },
                Engine::Oracle => {
                    match $conn.oracle_conn {
                        Some(ref oracle_conn) => {
                            //execute the query
                            let query_result = oracle_conn.query(&$query_str,$params);
                            match query_result {
                                Ok(rows_result) => {
                                    for row_result in rows_result {
                                        match row_result {
                                            Ok(row) => {
                                                $callback(&row);
                                            },
                                            Err(error) => {
                                                error!("{}",error);
                                                return Err(format!("{}",error).into());
                                            }
                                        }
                                    };

                                },
                                Err(error) => {
                                    return Err(format!("{}",error).into());
                                }
                            }
                        },
                        None => {
                            //Invalid database engine
                            let message = String::from("Oracle engine is None");
                            error!("{}",message);
                            return Err(message.into());
                        }
                    }
                }
            }
    );
}

pub fn get_info(id: &str, conn: &Connection) -> Result<Option<Info>> {
    //perform the query
    let query_str = query_builder::info(&conn.engine);
    let closure = |row: &MyRow| -> Result<Info> {
        return build_info(id, conn, row);
    };
    return execute_query!(closure,conn,query_str,&[&String::from(id)]);
}

fn build_info(id: &str,conn: &Connection,row: &MyRow) -> Result<Info>{
    //build the organism
    let organism = Organism {
        taxon_code: row.get_string("taxon_code"),
        species: row.get_string("taxon_species"),
        common_name: row.get_string("taxon_common"),
    };

    //get the pro info
    let pro = get_pro_info(id,&conn)?;

    //get synonyms
    let synonymns_str = row.get_string("gene_syn");
    let synonymns = misc::to_vec_string(&synonymns_str, "|");

    //build info
    let info = Info{
        uniprot_ac: row.get_string_unwrapped("iptm_entry_code"),
        uniprot_id: row.get_string_unwrapped("uniprot_id"),
        protein_name: row.get_string("protein_name"),
        gene_name: row.get_string("gene_name"),
        synonyms: synonymns,
        organism: organism,
        pro: pro,
    };

    return Ok(info);
}


pub fn get_pro_info(id: &str, conn: &Connection) -> Result<Option<Pro>> {
    //construct the pro id
    let pro_id = format!("PR:{id}",id=id);

    //build query str
    let query_str = query_builder::pro_info(&conn.engine);
        
    return execute_query!(build_pro_info,conn,query_str,&[&pro_id])

}

fn build_pro_info(row: &MyRow) -> Result<Pro> {
    let pro  = Pro {
        id: row.get_string_unwrapped("iptm_entry_code"),
        name: row.get_string("protein_name"),
        category: row.get_string("category"),
        definition: row.get_string("definition"),
        short_label: row.get_string("protein_synonyms")
    };
    return Ok(pro);
}


pub fn search(search_term: &str, term_type: &str, role: &str, ptm_types: &Vec<String>, organism_taxons: &Vec<i32>,conn: &Connection) -> Result<Rc<RefCell<Vec<SearchResult>>>>{

    //build the query
    let query_str = query_builder::search(term_type,role,organism_taxons,&conn.engine);
    info!("{}",query_str);
    //build ptm labels
    let mut ptm_labels_to_filter: Vec<String> = Vec::new();
    for ptm_type in ptm_types {
        let ptm_label_option = misc::get_ptm_event_label(&ptm_type.to_lowercase());
        match ptm_label_option {
            Some(ptm_label) => {
                ptm_labels_to_filter.push(ptm_label)
            },
            None => {
                return Err(format!("invalid PTM type {}",ptm_type).into());
            }
        }
    };

    let search_results: Rc<RefCell<Vec<SearchResult>>> = Rc::new(RefCell::new(Vec::new()));

    let closure = |row:&MyRow| {
        // Get the search result from the row
        let search_result_opt = get_search_result(row,&ptm_labels_to_filter);
        match search_result_opt {
            Some(value) => {
                search_results.borrow_mut().push(value);
            },
            None => {
            }
        }
    };

    let search_term_formatted;
    match &conn.engine {
        Engine::Postgres => {
            search_term_formatted = format!("%{search_term}%",search_term=search_term);
        },
        Engine::Oracle => {
            search_term_formatted = String::from(search_term);
        }
    }

    if term_type == "All" {
        execute_query_callback!(closure,conn,query_str,&[&search_term_formatted,&search_term_formatted,&search_term_formatted]);
        return Ok(search_results.clone());
    }else if term_type == "UniprotID" {
        execute_query_callback!(closure,conn,query_str,&[&search_term_formatted]);
        return Ok(search_results.clone());        
    }else if term_type == "Protein/Gene Name" {
        execute_query_callback!(closure,conn,query_str,&[&search_term_formatted,&search_term_formatted]);
        return Ok(search_results.clone());
    }else{
        return Ok(search_results.clone());
    }
}

fn get_search_result(row: &MyRow,ptm_labels_to_filter: &Vec<String>) -> Option<SearchResult> {
    let organism = Organism {
        taxon_code: row.get_string("taxon_code"),
        species: row.get_string("taxon_species"),
        common_name: row.get_string("taxon_common"),
    };

    let synonym_str: Option<String> = row.get_string("gene_syn");

    let search_result = SearchResult {
        iptm_id : row.get_string("iptm_entry_code"),
        uniprot_ac: row.get_string("uniprot_id"),
        protein_name: row.get_string("protein_name"),
        gene_name: row.get_string("gene_name"),
        synonyms: misc::to_vec_string(&synonym_str,"|"),
        organism: Some(organism),
        substrate_role: misc::to_bool(row.get_string("role_as_substrate")),
        substrate_num: row.get_i64("num_substrate"),
        enzyme_role: misc::to_bool(row.get_string("role_as_enzyme")),
        enzyme_num: row.get_i64("num_enzyme"),
        ptm_dependent_ppi_role: misc::to_bool(row.get_string("role_as_ppi")),
        ptm_dependent_ppi_num: row.get_i64("num_ppi"),
        sites: row.get_i64("num_site"),
        isoforms: row.get_i64("num_form")
    };

    let ptm_str: Option<String> = row.get_string("list_as_substrate");
    let ptm_labels = misc::remove_duplicates(&misc::to_vec_string(&ptm_str,","));

    // if the client has suppplied a list of ptm labels to filter against
    if ptm_labels_to_filter.len() > 0 {

        //check if the ptms we received from the DB contain the ptms that client has asked to filter against
        let has_filter_labels = misc::has_filter_labels(&ptm_labels,ptm_labels_to_filter);
        if has_filter_labels {
            return Some(search_result);
        }else{
            return None;
        }
    }else {
        return Some(search_result);
    }
}

pub fn get_substrate_events(id: &str, conn: &Connection) -> Result<HashMap<String,Vec<SubstrateEvent>>> {
     //get a list of forms for the given id
     let sub_forms_result = get_sub_forms(id,conn);  
     let sub_forms: Vec<String>;
     match sub_forms_result {
         Ok(value) => {
            sub_forms = value;
         },
         Err(error) => {
             return Err(format!("{}",error).into());
         }
     };

     let mut substrate_events: HashMap<String,Vec<SubstrateEvent>> = HashMap::new();
    
     for sub_form in sub_forms {
        let result = get_events_for_sub_form(&sub_form,conn);

        let mut events;
        let mut pmid_stats;

        match result {
            Ok(value) => {
                events = value.0;
                pmid_stats = value.1;
            },
            Err(error) => {
                    return Err(format!("{}",error).into());    
            }
        }

        for event in &mut events {
            let mut sources: Vec<String> = Vec::new();
            for source in &event.sources {
                match source.label {
                    Some(ref value) => {
                        sources.push(value.clone());
                    },
                    None => {
                        return Err("label for source is null".into());
                    }
                }
                
            }
            let score = misc::calculate_score(&pmid_stats,&event.pmids,&sources);
            event.score = Some(score);
        }

        substrate_events.insert(sub_form,events);

     }

     return Ok(substrate_events);
} 

pub fn get_sub_forms(id: &str,conn: &Connection) -> Result<Vec<String>>{
    let query_str = query_builder::sub_forms(&conn.engine);
  
    let mut sub_forms: Vec<String> = Vec::new();

    execute_query_bulk!(build_sub_form,conn,query_str,sub_forms,&[&String::from(id)]);

    return Ok(sub_forms);    

}

fn build_sub_form(row: &MyRow) -> Result<String> {
    let sub_form_code_option = row.get_string("sub_form_code");
    let sub_form_code;

    match sub_form_code_option {
        Some(value) => {
            sub_form_code = value;
        },
        None => {
            sub_form_code = String::from("");
        }
    }

    return Ok(sub_form_code);
}

fn get_events_for_sub_form(sub_form: &str,conn: &Connection) -> Result<(Vec<SubstrateEvent>,HashMap<String,i64>)>{
    let query_str = format!("SELECT RESIDUE,POSITION,EVENT_NAME,ENZ_CODE,ENZ_TYPE,ENZ_SYMBOL,SOURCE_LABEL,PMIDS,NUM_SUBSTRATES \
                    FROM MV_EVENT \
                    where SUB_FORM_CODE = '{id}' \
                    ORDER BY RESIDUE,POSITION,EVENT_NAME",id=sub_form);

    let mut events: Vec<SubstrateEvent> = Vec::new();
    let mut pmid_stats: HashMap<String,i64> = HashMap::new();

        match conn.engine {
        Engine::Postgres => {

            match conn.pg_conn {
                Some(ref pg_conn) => {
                    //execute the query                    
                    let rows_result = pg_conn.query(&query_str,&[]);
                    match rows_result {
                        Ok(rows) => {
                            for row in rows.iter() {
                                //update the events and pmid stats
                                update_events(&row,&mut events,&mut pmid_stats);
                            }
                        },
                        Err(error) => {
                            return Err(format!("{}",error).into());
                        }
                    };
                },
                None => {
                    let message = String::from("Postgres engine is None");
                    error!("{}",message);
                    return Err(message.into());
                }
            }
        },
        Engine::Oracle => {
            match conn.oracle_conn {
                Some(ref oracle_conn) => {
                    //execute the query
                    let query_result = oracle_conn.query(&query_str,&[]);
                    match query_result {
                        Ok(rows_result) => {
                            for row_result in rows_result {
                                match row_result {
                                    Ok(row) => {
                                        //update the events and pmid stats
                                        update_events(&row,&mut events,&mut pmid_stats);                                        
                                    },
                                    Err(error) => {
                                        error!("{}",error);
                                        return Err(format!("{}",error).into());
                                    }
                                }
                            };

                        },
                        Err(error) => {
                            return Err(format!("{}",error).into());
                        }
                    }
                },
                None => {
                    //Invalid database engine
                    let message = String::from("Oracle engine is None");
                    error!("{}",message);
                    return Err(message.into());
                }
            }
        }
    }

    return Ok((events,pmid_stats));

}

fn update_events(row: &MyRow,events: &mut Vec<SubstrateEvent>,pmid_stats: &mut HashMap<String,i64>){
    
    //if events is not empty
    let events_count = events.len();
    if events_count != 0 {
        //get the previous event
        let event_clone = events.clone();
        let previous_event = event_clone.get(events_count - 1).unwrap();

        //get the current event
        let current_event = build_event(row);

        //update the pmid stats
        update_pmid_stats(row,&current_event.pmids,pmid_stats);

        //if it is the same event
        if current_event.site == previous_event.site && current_event.ptm_type == previous_event.ptm_type {
            
            let previous_event_mut = &mut events.get_mut(events_count - 1).unwrap();

            //update enzymes
            for enzyme in current_event.enzymes {
                if !previous_event.enzymes.contains(&enzyme) {
                    previous_event_mut.enzymes.push(enzyme);
                }
            }

            //update the sources
            for source in current_event.sources {
                if !&previous_event.sources.contains(&source) {
                    previous_event_mut.sources.push(source);
                }     
            }

            //update the pmid
            for pmid in &current_event.pmids {
                if !previous_event.pmids.contains(&pmid) {
                    previous_event_mut.pmids.push(pmid.clone());
                }
            }

        }else{
            events.push(current_event);
        }

    }
    //if events is empty, which means this is our fist event
    else{
        let event = build_event(row);
        update_pmid_stats(row,&event.pmids,pmid_stats);
        events.push(event);
    }
}

fn build_event(row: &MyRow) -> SubstrateEvent {
    let residue = row.get_string("residue");
    let site;
    let ptm_type;  

    let mut enzymes: Vec<Enzyme> = Vec::new();
    let mut sources: Vec<Source> = Vec::new();

    match &residue {
        &Some(ref residue_value) => {
            let position = row.get_i64("position");
            match position {
                Some(pos_value) => {
                    site = Some(format!("{residue}{position}",residue=residue_value,position=pos_value));
                    ptm_type = row.get_string("event_name");
                },
                None => {
                    site = None;
                    ptm_type = None;
                }
            }            
        },
        & None => {
            site = None;
            ptm_type = None;
        }
    }

    let enzyme_id = row.get_string("enz_code");
    match enzyme_id {
        Some(value) => {
            let enzyme = Enzyme {
                id: Some(value),
                enz_type: row.get_string("enz_type"),
                name: row.get_string("enz_symbol"),
            };

            // add this enymze only if the name is not null
            match &enzyme.name {
                Some(ref val) => {
                    if val.len() > 0 {
                        enzymes.push(enzyme.clone());
                    }
                },
                None => {

                }
            }
            
        },
        None => {

        }
    }

    let source_label  = row.get_string("source_label");
    let source = misc::get_source(source_label);
    match source {
        Some(value) => {
            sources.push(value);
        },
        None => {

        }
    }

    let pmid_option = row.get_string("pmids");
    let pmids = misc::to_pmid_list(pmid_option);

    
    let event = SubstrateEvent {
        residue: residue,
        site: site,
        ptm_type: ptm_type,
        score:Some(0),
        sources: sources,
        enzymes: enzymes,
        pmids: pmids 
        
    };
    return event;

}

fn update_pmid_stats(row: &MyRow,pmids: &Vec<String>,pmid_stats: &mut HashMap<String,i64>) {
    let num_substrates_str = row.get_string("num_substrates");
    
    let num_substrates = misc::to_vec_i64(&num_substrates_str,"|");
    let mut index = 0;
    for pmid in pmids {
        match num_substrates.get(index) {
            Some(value) => {
                pmid_stats.insert(pmid.clone(),*value); 
            },
            None => {
                pmid_stats.insert(pmid.clone(),0);
            }
        }

       index = index + 1;

    }   

}


pub fn get_proteoforms(id: &str, conn: &Connection) -> Result<Vec<Proteoform>> {
    let query_str = query_builder::proteoforms(&conn.engine);    
    let mut proteoforms : Vec<Proteoform> = Vec::new();

    let formatted_id;
    match &conn.engine {
        Engine::Postgres => {
            formatted_id = format!("%{id}%",id=id);
        },
        Engine::Oracle => {
            formatted_id = String::from(id);
        }
    }

    execute_query_bulk!(build_proteoform,conn,query_str,proteoforms,&[&formatted_id]);

    return Ok(proteoforms);

}

fn build_proteoform(row: &MyRow) -> Result<Proteoform> {
        // build enzyme
        let enzyme = Protein {
            pro_id: row.get_string("enz_code"),
            label: row.get_string("enz_symbol"),
        };

        // build source
        let source_label: Option<String> = row.get_string("source_label");
        let source = misc::get_source(source_label);

        //build pmids
        let pmid_option = row.get_string("pmids");
        let pmids = misc::to_pmid_list(pmid_option);

        // build proteoform
        let sites_str: Option<String> = row.get_string("sites");
        let proteoform = Proteoform {
            pro_id: row.get_string("sub_code"),
            label: row.get_string("sub_symbol"),
            sites: misc::to_vec_string(&sites_str,","),
            ptm_enzyme: Some(enzyme),
            source: source,
            pmids: pmids
        };

        return Ok(proteoform);
}


pub fn get_proteoformppis(id: &str, conn: &Connection) -> Result<Vec<ProteoformPPI>> {

    // build the query
    let query_str = query_builder::proteoformppi(&conn.engine);

    let mut proteoformsppi : Vec<ProteoformPPI> = Vec::new();

    let formatted_id;
    match &conn.engine {
        Engine::Postgres => {
            formatted_id = format!("%{id}%",id=id);
        },
        Engine::Oracle => {
            formatted_id = String::from(id);
        }
    }

    execute_query_bulk!(build_proteoform_ppi,conn,query_str,proteoformsppi,&[&formatted_id]);    

    return Ok(proteoformsppi);

}

fn build_proteoform_ppi(row: &MyRow) -> Result<ProteoformPPI>{
        //build protein 1
        let protein_1 = Protein{
            pro_id: row.get_string("sub_code"),
            label: row.get_string("sub_symbol")
        };

        //build protein 2
        let protein_2 = Protein {
            pro_id: row.get_string("enz_code"),
            label: row.get_string("enz_symbol")
        };

        //relation
        let relation: Option<String> = row.get_string("event_name");

        //source
        let source_label: Option<String> = row.get_string("source_label");

        //pmids
        let pmids_option = row.get_string("pmids");
        let pmids = misc::to_pmid_list(pmids_option);

        //build proteoform ppi
        let proteoformppi = ProteoformPPI{
            protein_1: Some(protein_1),
            protein_2: Some(protein_2),
            relation: relation,
            source: misc::get_source(source_label),
            pmids: pmids,    
        };
        return Ok(proteoformppi);
}

pub fn get_ptmppis(id: &str, conn: &Connection) -> Result<Vec<PTMPPI>> {
    //build the query
    let query_str = query_builder::ptmppi(&conn.engine);
    
    let mut ptmppis : Vec<PTMPPI> = Vec::new();

    execute_query_bulk!(build_pptm_ppi,conn,query_str,ptmppis,&[&String::from(id)]);    

    return Ok(ptmppis);

}

fn build_pptm_ppi(row: &MyRow) -> Result<PTMPPI> {
        //build substrate
        let substrate = Entity{
            uniprot_id: row.get_string("ppi_sub_code"),
            name: row.get_string("ppi_sub_symbol"),
        };

        //build interactant
        let interactant = Entity {
            uniprot_id: row.get_string("ppi_pr_code"),
            name: row.get_string("ppi_pr_symbol"),
        };

        //site
        let residue: Option<String> = row.get_string("ptm_residue");
        let position: Option<i64> = row.get_i64("ptm_position");
        let site = match residue {
                    Some(residue_value) => {
                        match position {
                            Some(pos_value) => {
                                format!("{residue}{pos}",residue=residue_value,pos=pos_value)
                            },
                            None => {
                                String::from("")
                            }
                        }
                    },
                    None => {
                        String::from("")
                    }
                };

        //source
        let source_label: Option<String> = row.get_string("ppi_source_label");

        let ptmppi = PTMPPI {
            ptm_type: row.get_string("ptm_event_name"),
            substrate: Some(substrate),
            site: Some(site),
            interactant: Some(interactant),
            association_type: row.get_string("impact"),
            source: misc::get_source(source_label),
            pmid: row.get_string("ppi_pmids"),
        };

        return Ok(ptmppi);

}


pub fn get_ptm_enzymes(query_substrates: &Vec<QuerySubstrate>, conn: &Connection) -> Result<Vec<BatchPTMEnzyme>> {
    //convert query_substrates to tuples string
    let query_substrates_str = misc::query_substrates_to_tuple_str(query_substrates);

    //build the query string
    let query_str;
    match conn.engine {
        Engine::Postgres => {
            query_str = format!("SELECT event_name,sub_code,sub_symbol,residue,position,enz_code,enz_symbol, \
                string_agg(source_label,',' ORDER BY source_label) as source_label, \
                string_agg(num_substrates,'|' ORDER BY source_label) as num_substrates, \
                string_agg(pmids,',' ORDER BY source_label) as pmids \
                FROM MV_EVENT \
                where (sub_code,residue,position) in ({tuples}) and enz_code is not NULL \
                GROUP BY(enz_code,enz_symbol,sub_code,sub_symbol,residue,position,event_name)",tuples=query_substrates_str); 
        },
        Engine::Oracle => {
            query_str = format!("SELECT event_name,sub_code,sub_symbol,residue,position,enz_code,enz_symbol, \
                LISTAGG(source_label,',') WITHIN GROUP (ORDER BY source_label) as source_label, \
                LISTAGG(num_substrates,'|') WITHIN GROUP (ORDER BY source_label) as num_substrates, \
                LISTAGG(pmids,',') WITHIN GROUP (ORDER BY source_label) as pmids \
                FROM MV_EVENT \
                where (sub_code,residue,position) in ({tuples}) and enz_code is not NULL \
                GROUP BY(enz_code,enz_symbol,sub_code,sub_symbol,residue,position,event_name)",tuples=query_substrates_str);
        }
    }

    let mut ptm_enzymes: Vec<BatchPTMEnzyme> = Vec::new();

    execute_query_bulk!(build_ptm_enzyme,conn,query_str,ptm_enzymes,&[]);    

    Ok(ptm_enzymes)

}

fn build_ptm_enzyme(row: &MyRow) -> Result<BatchPTMEnzyme> {
        //build enzyme                
        let enzyme = Entity{
            name: row.get_string("enz_symbol"),
            uniprot_id: row.get_string("enz_code")
        };

        //build substrate
        let substrate = Entity {
            name: row.get_string("sub_symbol"),
            uniprot_id: row.get_string("sub_code")
        };

        //source
        let source_label_str: Option<String> = row.get_string("source_label");
        let seperator = ",";
        let sources_labels = misc::remove_duplicates(&misc::to_vec_string(&source_label_str,seperator));
        let mut sources: Vec<Source> = Vec::new();
        for source_label in &sources_labels {
            let source = misc::get_source(Some(source_label.clone()));
            match source {
                Some(val) => {sources.push(val)},
                None => {}
            }
        }

        //site
        let residue: Option<String> = row.get_string("residue");
        let position: Option<i64> = row.get_i64("position");
        let site = match residue {
                    Some(residue_value) => {
                        match position {
                            Some(pos_value) => {
                                format!("{residue}{pos}",residue=residue_value,pos=pos_value)
                            },
                            None => {
                                String::from("")
                            }
                        }
                    },
                    None => {
                        String::from("")
                    }
                };

        //pmids
        let pmid_str: Option<String> = row.get_string("pmids");
        let pmids = misc::remove_duplicates(&misc::to_vec_string(&pmid_str,","));

        //get number of substrates
        let num_substrates_str:Option<String> = row.get_string("num_substrates");
        let num_substrates = misc::to_vec_i64(&num_substrates_str,"|");        

        //calculate score
        let score = misc::calculate_score_batch_ptm_enzymes(num_substrates,&sources_labels,&pmids);

        //build ptm_enzyme
        let ptm_enzyme = BatchPTMEnzyme {
            ptm_type: row.get_string("event_name"),
            site: Some(site),
            site_position: row.get_i64("position"),
            enzyme: Some(enzyme),
            substrate: Some(substrate),
            source: sources,
            pmids: pmids,
            score: score
        };

        return Ok(ptm_enzyme);
}


pub fn get_ptm_ppi(query_substrates: &Vec<QuerySubstrate>, conn: &Connection) -> Result<Vec<BatchPTMPPI>> {
    //convert query_substrates to tuples string
    let query_substrates_str = misc::query_substrates_to_tuple_str(query_substrates);

    //build the query string
    let query_str = format!("SELECT * FROM MV_EFIP \
                where (ptm_sub_code,ptm_residue,ptm_position) in ({tuples})",tuples=query_substrates_str);

    let mut ptm_ppis: Vec<BatchPTMPPI> = Vec::new();

    execute_query_bulk!(build_ptm_ppi,conn,query_str,ptm_ppis,&[]);

    Ok(ptm_ppis)

}

fn build_ptm_ppi(row: &MyRow) -> Result<BatchPTMPPI> {
        //build interactant                
        let interactant = Entity{
            name: row.get_string("ppi_pr_symbol"),
            uniprot_id: row.get_string("ppi_pr_code")
        };

        //build substrate
        let substrate = Entity {
            name: row.get_string("ppi_sub_symbol"),
            uniprot_id: row.get_string("ppi_sub_code")
        };

        //source
        let source_label: Option<String> = row.get_string("ptm_source_label");

        //site
        let residue: Option<String> = row.get_string("ptm_residue");
        let position: Option<i64> = row.get_i64("ptm_position");
        let site = match residue {
                    Some(residue_value) => {
                        match position {
                            Some(pos_value) => {
                                format!("{residue}{pos}",residue=residue_value,pos=pos_value)
                            },
                            None => {
                                String::from("")
                            }
                        }
                    },
                    None => {
                        String::from("")
                    }
                };

        //pmids
        let pmid_str: Option<String> = row.get_string("ppi_pmids");

        //build ptm_ppi
        let ptm_ppi = BatchPTMPPI {
            ptm_type: row.get_string("ptm_event_name"),
            site: Some(site),
            site_position: position,
            association_type: row.get_string("impact"),
            interactant: Some(interactant),
            substrate: Some(substrate),
            source: misc::get_source(source_label),
            pmids: misc::to_vec_string(&pmid_str,",")
        };

        return Ok(ptm_ppi);
}
