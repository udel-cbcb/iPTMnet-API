
use actix_web::*;
use database;
use serde_json;
use misc;
use msa;
use std::str;
use futures::future::Future;
use models::QuerySubstrate;
use flatten;
use csv;
use std::collections::HashMap;
use futures::Stream;
use std::fs::File;
use std::io::prelude::*;

pub fn get_status_controller(_req: HttpRequest<super::State>) -> HttpResponse {
    let mut status : HashMap<&str,&str> = HashMap::new();
    status.insert("status","alive");
    status.insert("version","1.1.8");
    let status_serialized = serde_json::to_string_pretty(&status).unwrap();
    return HttpResponse::Ok().force_close().body(status_serialized);
}

pub fn get_info_controller(req: HttpRequest<super::State>) -> HttpResponse {
    //get the value of ID
    let id: String  = req.match_info().query("id").unwrap();

    //get the connection from pool
    let conn;
    match database::connect(&req.state().db_params) {
        Ok(val) => {conn = val},
        Err(error) => {return HttpResponse::InternalServerError().force_close().body(format!("{}",error));},
    }

    //get the id string
    let info_result = database::get_info(&id,&conn);

    match info_result {
        Ok(info) => {

            let info_serialized_result = serde_json::to_string_pretty(&info);

            match info_serialized_result {

                Ok(info_serialized) => {
                    HttpResponse::Ok()
                    .force_close()
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(info_serialized)
                },

                Err(error) => {
                    HttpResponse::InternalServerError()
                    .force_close()
                    .header(http::header::CONTENT_TYPE, "text/plain")
                    .body(format!("{}",error))
                }

            }

        },

        Err(error) => {
            HttpResponse::InternalServerError()
            .force_close()
            .header(http::header::CONTENT_TYPE, "text/plain")
            .body(format!("{}",error))
        }
    }

}


pub fn search_controller(req: HttpRequest<super::State>) -> HttpResponse {

    //get the connection from pool
    let conn;
    match database::connect(&req.state().db_params) {
        Ok(val) => {conn = val},
        Err(error) => {return HttpResponse::InternalServerError()
                        .force_close()
                        .header(http::header::CONTENT_TYPE, "text/plain")
                        .body(format!("{}",error));},
    }

    //get content header
    let content_header = misc::get_accept_header_value(&req);
    
    //params
    let search_term;
    let term_type;
    let role;
    let mut paginate = false;
    let limit;
    let offset;

    // search term
    let search_term_option = req.query().get("search_term");
    match search_term_option {
        Some(val) => {
            if !val.is_empty(){
                search_term = val
            }else{
                return HttpResponse::BadRequest()
                .force_close()
                .header(http::header::CONTENT_TYPE, "text/plain")
                .body("search_term cannot be blank");
            }
        },
        None => {return HttpResponse::BadRequest()
        .force_close()
        .header(http::header::CONTENT_TYPE, "text/plain")
        .body("search_term cannot be empty");}
    }

    // term type
    let term_type_option = req.query().get("term_type");
    match term_type_option {
        Some(val) => {term_type=val},
        None => {return HttpResponse::BadRequest()
        .force_close()
        .header(http::header::CONTENT_TYPE, "text/plain")
        .body("term_type cannot be empty");}
    }

    // role
    let role_option = req.query().get("role");
    match role_option {
        Some(val) => {role=val},
        None => {return HttpResponse::BadRequest()
                .force_close()
                .header(http::header::CONTENT_TYPE, "text/plain")
                .body("role cannot be empty");}
    }

    // ptm type
    let ptm_types = misc::get_vec_str_from_param(req.query(),"ptm_type");
    
    //build ptm labels
    let mut ptm_labels_to_filter: Vec<String> = Vec::new();

    if !ptm_types.is_empty(){
        for ptm_type in ptm_types {
            let ptm_label_option = misc::get_ptm_event_label(&ptm_type.to_lowercase());
            match ptm_label_option {
                Some(ptm_label) => {
                    ptm_labels_to_filter.push(ptm_label)
                },
                None => {
                    let error_msg = format!("invalid PTM type {}",ptm_type);
                    error!("{}",error_msg);
                    return HttpResponse::BadRequest()
                    .force_close()
                    .header(http::header::CONTENT_TYPE, "text/plain")
                    .body(error_msg);
                }
            }
        };       
    }else{
        ptm_labels_to_filter = misc::default_ptm_labels();        
    }
    
    // Organism
    let organism_taxon_codes;
    match misc::get_vec_i32_from_param(req.query(),"organism") {
        Ok(value) => {
            organism_taxon_codes = value;
        },
        Err(error) => {
            error!("{}",error);
            return  HttpResponse::InternalServerError()
                    .force_close()
                    .header(http::header::CONTENT_TYPE, "text/plain")
                    .body(format!("{}",error))
        }
    }
    
    //paginate
    let paginate_option = req.query().get("paginate");
    match paginate_option {
        Some(value) => {
            if String::from(value).to_lowercase() == "true" {
                paginate = true;
            }else if String::from(value).to_lowercase() == "false" {
                paginate = false;
            }else{
                error!("Invalid paginate option : {}",String::from(value));
            }
        },
        None => {
            paginate = false;
        },
    }

    //start
    if paginate {
        let start_index;
        let start_index_option = req.query().get("start_index");
        match start_index_option {
            Some(val) => {
                match val.parse::<i32>() {
                    Ok(start_index_val) => {
                        start_index = start_index_val
                    },
                    Err(error) => {
                        error!("{}",error);
                        return  HttpResponse::InternalServerError()
                        .force_close()
                        .header(http::header::CONTENT_TYPE, "text/plain")
                        .body(format!("{}",error))
                    },
                }
            },
            None => {
                return HttpResponse::BadRequest()
                    .force_close()
                    .header(http::header::CONTENT_TYPE, "text/plain")
                    .body("start_index cannot be empty");
            }
        }

        //end
        let end_index;
        let end_index_option = req.query().get("end_index");
        match end_index_option {
            Some(val) => {
                match val.parse::<i32>() {
                    Ok(end_index_val) => {
                        end_index = end_index_val
                    },
                    Err(error) => {
                        error!("{}",error);
                        return  HttpResponse::InternalServerError()
                        .force_close()
                        .header(http::header::CONTENT_TYPE, "text/plain")
                        .body(format!("{}",error))
                    },
                }
            },
            None => {
                return HttpResponse::BadRequest()
                    .force_close()
                    .header(http::header::CONTENT_TYPE, "text/plain")
                    .body("end_index cannot be empty");
            }
        }

        //calculate the limit and offset
        limit = end_index - start_index;
        if limit <= 0 {
            {return HttpResponse::BadRequest()
                    .force_close()
                    .header(http::header::CONTENT_TYPE, "text/plain")
                    .body("end_index cannot be smaller than or equal to start index");}
        }
        offset = start_index;
    }else{
        limit = 0;
        offset = 0;
    }

    // perform the search
    let search_values_result = database::search(search_term,term_type,role,&ptm_labels_to_filter,&organism_taxon_codes,paginate,offset,limit,&conn);

    match search_values_result {
        Ok(values) => {
            let (count, search_values) = values; 
            if content_header == "application/json" || content_header.is_empty() {
                let search_values_serialized_result = serde_json::to_string_pretty(&(*search_values));

                match search_values_serialized_result {

                    Ok(search_values_serialized) => {
                        HttpResponse::Ok()
                        .force_close()
                        .header(http::header::CONTENT_TYPE, "application/json")
                        .header("count", count.to_string())
                        .body(search_values_serialized)
                    },

                    Err(error) => {
                        error!("{}",error);
                        HttpResponse::InternalServerError()
                        .force_close()
                        .header(http::header::CONTENT_TYPE, "text/plain")
                        .body(format!("{}",error))
                    }

                }
            } else if content_header == "text/plain" {
                    //convert the values to flat structure
                    let search_results_flat = flatten::search_results(&(*search_values.borrow()));

                    let mut wtr = csv::Writer::from_writer(vec![]);
                    
                    for search_result_flat in search_results_flat {
                        let result = wtr.serialize(&search_result_flat);
                        match result {
                            Ok(_) => {},
                            Err(error) => {
                                error!("{}",error);
                                return HttpResponse::InternalServerError()
                                .force_close()
                                .header(http::header::CONTENT_TYPE, "text/plain")
                                .body(format!("{}",error));
                            }
                        }
                    }

                    let inner;
                    let inner_result = wtr.into_inner();
                    match inner_result {
                        Ok(value) => {inner=value;},
                        Err(error) => {
                            error!("{}",error);
                            return HttpResponse::InternalServerError()
                            .force_close()
                            .header(http::header::CONTENT_TYPE, "text/plain")
                            .body(format!("{}",error));
                        }
                    }

                    let data_result = String::from_utf8(inner);
                    match data_result {
                        Ok(data) => {
                            return HttpResponse::Ok()
                            .force_close()
                            .header(http::header::CONTENT_TYPE, "text/csv")
                            .header("count", count.to_string())
                            .body(data);
                        },
                        Err(error) => {
                            error!("{}",error);
                            return HttpResponse::InternalServerError()
                            .force_close()
                            .header(http::header::CONTENT_TYPE, "text/plain")
                            .body(format!("{}",error));
                        }
                    }
            }else {
                return HttpResponse::BadRequest()
                .force_close()
                .header(http::header::CONTENT_TYPE, "text/plain")
                .body(format!("Invalid ACCEPT header - {}",content_header));
            }

        },

        Err(error) => {
            error!("{}",error);
            HttpResponse::InternalServerError()
            .force_close()
            .header(http::header::CONTENT_TYPE, "text/plain")
            .body(format!("{}",error))
        }

    }
}

pub fn browse_controller(req: HttpRequest<super::State>) -> HttpResponse {

    //get the connection from pool
    let conn;
    match database::connect(&req.state().db_params) {
        Ok(val) => {conn = val},
        Err(error) => {return HttpResponse::InternalServerError()
                        .force_close()
                        .header(http::header::CONTENT_TYPE, "text/plain")
                        .body(format!("{}",error));},
    }

    //get content header
    let content_header = misc::get_accept_header_value(&req);
    
    //params
    let search_term = "";
    let term_type;
    let role;
    let paginate = true;
    let limit;
    let offset;

    // term type
    let term_type_option = req.query().get("term_type");
    match term_type_option {
        Some(val) => {term_type=val},
        None => {return HttpResponse::BadRequest()
        .force_close()
        .header(http::header::CONTENT_TYPE, "text/plain")
        .body("term_type cannot be empty");}
    }

    // role
    let role_option = req.query().get("role");
    match role_option {
        Some(val) => {role=val},
        None => {return HttpResponse::BadRequest()
                .force_close()
                .header(http::header::CONTENT_TYPE, "text/plain")
                .body("role cannot be empty");}
    }

    // ptm type
    let ptm_types = misc::get_vec_str_from_param(req.query(),"ptm_type");
    
    //build ptm labels
    let mut ptm_labels_to_filter: Vec<String> = Vec::new();

    if !ptm_types.is_empty(){
        for ptm_type in ptm_types {
            let ptm_label_option = misc::get_ptm_event_label(&ptm_type.to_lowercase());
            match ptm_label_option {
                Some(ptm_label) => {
                    ptm_labels_to_filter.push(ptm_label)
                },
                None => {
                    let error_msg = format!("invalid PTM type {}",ptm_type);
                    error!("{}",error_msg);
                    return HttpResponse::BadRequest()
                    .force_close()
                    .header(http::header::CONTENT_TYPE, "text/plain")
                    .body(error_msg);
                }
            }
        };       
    }else{
        ptm_labels_to_filter = misc::default_ptm_labels();        
    }
       
    // Organism
    let organism_taxon_codes;
    match misc::get_vec_i32_from_param(req.query(),"organism") {
        Ok(value) => {
            organism_taxon_codes = value;
        },
        Err(error) => {
            error!("{}",error);
            return  HttpResponse::InternalServerError()
                    .force_close()
                    .header(http::header::CONTENT_TYPE, "text/plain")
                    .body(format!("{}",error))
        }
    }
    
    let start_index;
    let start_index_option = req.query().get("start_index");
    match start_index_option {
            Some(val) => {
                match val.parse::<i32>() {
                    Ok(start_index_val) => {
                        start_index = start_index_val
                    },
                    Err(error) => {
                        error!("{}",error);
                        return  HttpResponse::InternalServerError()
                        .force_close()
                        .header(http::header::CONTENT_TYPE, "text/plain")
                        .body(format!("{}",error))
                    },
                }
            },
            None => {
                return HttpResponse::BadRequest()
                    .force_close()
                    .header(http::header::CONTENT_TYPE, "text/plain")
                    .body("start_index cannot be empty");
            }
    }

    //end
    let end_index;
    let end_index_option = req.query().get("end_index");
    match end_index_option {
            Some(val) => {
                match val.parse::<i32>() {
                    Ok(end_index_val) => {
                        end_index = end_index_val
                    },
                    Err(error) => {
                        error!("{}",error);
                        return  HttpResponse::InternalServerError()
                        .force_close()
                        .header(http::header::CONTENT_TYPE, "text/plain")
                        .body(format!("{}",error))
                    },
                }
            },
            None => {
                return HttpResponse::BadRequest()
                    .force_close()
                    .header(http::header::CONTENT_TYPE, "text/plain")
                    .body("end_index cannot be empty");
        }
    }

    //calculate the limit and offset
    limit = end_index - start_index;
    if limit <= 0 {
        {return HttpResponse::BadRequest()
                    .force_close()
                    .header(http::header::CONTENT_TYPE, "text/plain")
                    .body("end_index cannot be smaller than or equal to start index");}
    }
    offset = start_index;

    // perform the search
    let search_values_result = database::search(search_term,term_type,role,&ptm_labels_to_filter,&organism_taxon_codes,paginate,offset,limit,&conn);

    match search_values_result {
        Ok(values) => {
            let (count, search_values) = values; 
            if content_header == "application/json" || content_header.is_empty() {
                let search_values_serialized_result = serde_json::to_string_pretty(&(*search_values));

                match search_values_serialized_result {

                    Ok(search_values_serialized) => {
                        HttpResponse::Ok()
                        .force_close()
                        .header(http::header::CONTENT_TYPE, "application/json")
                        .header("count", count.to_string())
                        .body(search_values_serialized)
                    },

                    Err(error) => {
                        error!("{}",error);
                        HttpResponse::InternalServerError()
                        .force_close()
                        .header(http::header::CONTENT_TYPE, "text/plain")
                        .body(format!("{}",error))
                    }

                }
            } else if content_header == "text/plain" {
                    //convert the values to flat structure
                    let search_results_flat = flatten::search_results(&(*search_values.borrow()));

                    let mut wtr = csv::Writer::from_writer(vec![]);
                    
                    for search_result_flat in search_results_flat {
                        let result = wtr.serialize(&search_result_flat);
                        match result {
                            Ok(_) => {},
                            Err(error) => {
                                error!("{}",error);
                                return HttpResponse::InternalServerError()
                                .force_close()
                                .header(http::header::CONTENT_TYPE, "text/plain")
                                .body(format!("{}",error));
                            }
                        }
                    }

                    let inner;
                    let inner_result = wtr.into_inner();
                    match inner_result {
                        Ok(value) => {inner=value;},
                        Err(error) => {
                            error!("{}",error);
                            return HttpResponse::InternalServerError()
                            .force_close()
                            .header(http::header::CONTENT_TYPE, "text/plain")
                            .body(format!("{}",error));
                        }
                    }

                    let data_result = String::from_utf8(inner);
                    match data_result {
                        Ok(data) => {
                            return HttpResponse::Ok()
                            .force_close()
                            .header(http::header::CONTENT_TYPE, "text/csv")
                            .header("count", count.to_string())
                            .body(data);
                        },
                        Err(error) => {
                            error!("{}",error);
                            return HttpResponse::InternalServerError()
                            .force_close()
                            .header(http::header::CONTENT_TYPE, "text/plain")
                            .body(format!("{}",error));
                        }
                    }
            }else {
                return HttpResponse::BadRequest()
                .force_close()
                .header(http::header::CONTENT_TYPE, "text/plain")
                .body(format!("Invalid ACCEPT header - {}",content_header));
            }

        },

        Err(error) => {
            HttpResponse::InternalServerError()
            .force_close()
            .header(http::header::CONTENT_TYPE, "text/plain")
            .body(format!("{}",error))
        }

    }
}


pub fn substrate_controller(req: HttpRequest<super::State>) -> HttpResponse {
    //get the value of ID
    let id: String  = req.match_info().query("id").unwrap();

    //get content header
    let content_header = misc::get_accept_header_value(&req);

    //get the connection from pool
    let conn;
    match database::connect(&req.state().db_params) {
        Ok(val) => {conn = val},
        Err(error) => {return HttpResponse::InternalServerError()
                        .force_close()
                        .header(http::header::CONTENT_TYPE, "text/plain")
                        .body(format!("{}",error));},
    }

    //get the id strings
    let substrate_events_results = database::get_substrate_events(&id,&conn);

    //check if operation was successful
    match substrate_events_results {
        Ok(substrate_events) => {

            if content_header == "application/json" || content_header.is_empty() {
                //try deserializing    
                let substrate_events_serialized_result = serde_json::to_string_pretty(&substrate_events);

                //check if operation was successful
                match substrate_events_serialized_result {

                    Ok(substrate_events_serialized) => {
                        HttpResponse::Ok()
                        .force_close()
                        .header(http::header::CONTENT_TYPE, "application/json")
                        .body(substrate_events_serialized)
                    },

                    Err(error) => {
                        error!("{}",error);
                        return HttpResponse::InternalServerError()
                        .force_close()
                        .header(http::header::CONTENT_TYPE, "text/plain")
                        .body(format!("{}",error));
                    }

                }
            }else if content_header == "text/plain"{
                //convert the values to flat structure
                let substrate_events_flat = flatten::substrate_events(&substrate_events);

                let mut wtr = csv::Writer::from_writer(vec![]);
                
                for substrate_event_flat in substrate_events_flat {
                    let result = wtr.serialize(&substrate_event_flat);
                    match result {
                        Ok(_) => {},
                        Err(error) => {
                            error!("{}",error);
                            return HttpResponse::InternalServerError()
                            .force_close()
                            .header(http::header::CONTENT_TYPE, "text/plain")
                            .body(format!("{}",error));
                        }
                    }
                }

                let inner;
                let inner_result = wtr.into_inner();
                match inner_result {
                    Ok(value) => {inner=value;},
                    Err(error) => {
                        error!("{}",error);
                        return HttpResponse::InternalServerError()
                        .force_close()
                        .header(http::header::CONTENT_TYPE, "text/plain")
                        .body(format!("{}",error));
                    }
                }

                let data_result = String::from_utf8(inner);
                match data_result {
                    Ok(data) => {
                        return HttpResponse::Ok().force_close().header(http::header::CONTENT_TYPE, "text/csv").body(data);
                    },
                    Err(error) => {
                        error!("{}",error);
                        return HttpResponse::InternalServerError()
                        .force_close()
                        .header(http::header::CONTENT_TYPE, "text/plain")
                        .body(format!("{}",error));
                    }
                }
            }else {
                return HttpResponse::BadRequest().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(format!("Invalid ACCEPT header - {}",content_header));
            }          

        },

        Err(error) => {
            HttpResponse::InternalServerError().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(format!("{}",error))
        }
    }

}


pub fn proteoforms_controller(req: HttpRequest<super::State>) -> HttpResponse {
    //get the value of ID
    let id: String  = req.match_info().query("id").unwrap();

    //get content header
    let content_header = misc::get_accept_header_value(&req);

    //get the connection from pool
    let conn;
    match database::connect(&req.state().db_params) {
        Ok(val) => {conn = val},
        Err(error) => {return HttpResponse::InternalServerError().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(format!("{}",error));},
    }

    //get the id strings
    let proteoforms_result = database::get_proteoforms(&id,&conn);

    //check if operation was successful
    match proteoforms_result {
        Ok(proteoforms) => {

            if content_header == "application/json" || content_header.is_empty() {
                //try deserializing    
                let proteoforms_serialized_result = serde_json::to_string_pretty(&proteoforms);

                //check if operation was successful
                match proteoforms_serialized_result {

                    Ok(proteoforms_serialized) => {
                        HttpResponse::Ok().force_close().header(http::header::CONTENT_TYPE, "application/json").body(proteoforms_serialized)
                    },

                    Err(error) => {
                        error!("{}",error);
                        return HttpResponse::InternalServerError().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(format!("{}",error));
                    }

                }
            }else if content_header == "text/plain"{
                //convert the values to flat structure
                let protoroforms_flat = flatten::proteoform(&proteoforms);

                let mut wtr = csv::Writer::from_writer(vec![]);
                
                for proteoform_flat in protoroforms_flat {
                    let result = wtr.serialize(&proteoform_flat);
                    match result {
                        Ok(_) => {},
                        Err(error) => {
                            error!("{}",error);
                            return HttpResponse::InternalServerError().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(format!("{}",error));
                        }
                    }
                }

                let inner;
                let inner_result = wtr.into_inner();
                match inner_result {
                    Ok(value) => {inner=value;},
                    Err(error) => {
                        error!("{}",error);
                        return HttpResponse::InternalServerError().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(format!("{}",error));
                    }
                }

                let data_result = String::from_utf8(inner);
                match data_result {
                    Ok(data) => {
                        return HttpResponse::Ok().force_close().body(data);
                    },
                    Err(error) => {
                        error!("{}",error);
                        return HttpResponse::InternalServerError().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(format!("{}",error));
                    }
                }
            }else {
                return HttpResponse::BadRequest().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(format!("Invalid ACCEPT header - {}",content_header));
            }          

        },

        Err(error) => {
            HttpResponse::InternalServerError().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(format!("{}",error))
        }
    }

}


pub fn proteoformsppi_controller(req: HttpRequest<super::State>) -> HttpResponse {
    //get the value of ID
    let id: String  = req.match_info().query("id").unwrap();

    //get content header
    let content_header = misc::get_accept_header_value(&req);

    //get the connection from pool
    let conn;
    match database::connect(&req.state().db_params) {
        Ok(val) => {conn = val},
        Err(error) => {return HttpResponse::InternalServerError().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(format!("{}",error));},
    }

    //get the id strings
    let proteoforms_ppi_result = database::get_proteoformppis(&id,&conn);

    //check if the operation was successful
    match proteoforms_ppi_result {
        Ok(proteoforms_ppi) => {
            
            if content_header == "application/json" || content_header.is_empty() {
                //try deserializing    
                let proteoforms_serialized_result = serde_json::to_string_pretty(&proteoforms_ppi);

                //check if operation was successful
                match proteoforms_serialized_result {

                    Ok(proteoforms_serialized) => {
                        HttpResponse::Ok().body(proteoforms_serialized)
                    },

                    Err(error) => {
                        HttpResponse::InternalServerError().force_close().header(http::header::CONTENT_TYPE, "application/json").body(format!("{}",error))
                    }

                }
            }else if content_header == "text/plain" {
                //convert the values to flat structure
                let protoroforms_ppi_flat = flatten::proteoform_ppis(&proteoforms_ppi);

                let mut wtr = csv::Writer::from_writer(vec![]);
                
                for proteoform_ppi_flat in protoroforms_ppi_flat {
                    let result = wtr.serialize(&proteoform_ppi_flat);
                    match result {
                        Ok(_) => {},
                        Err(error) => {
                            error!("{}",error);
                            return HttpResponse::InternalServerError().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(format!("{}",error));
                        }
                    }
                }

                let inner;
                let inner_result = wtr.into_inner();
                match inner_result {
                    Ok(value) => {inner=value;},
                    Err(error) => {
                        error!("{}",error);
                        return HttpResponse::InternalServerError().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(format!("{}",error));
                    }
                }

                let data_result = String::from_utf8(inner);
                match data_result {
                    Ok(data) => {
                        return HttpResponse::Ok().force_close().body(data);
                    },
                    Err(error) => {
                        error!("{}",error);
                        return HttpResponse::InternalServerError().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(format!("{}",error));
                    }
                }                
            }else {
                return HttpResponse::BadRequest().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(format!("Invalid ACCEPT header - {}",content_header));
            }           
        },

        Err(error) => {
            HttpResponse::InternalServerError().header(http::header::CONTENT_TYPE, "text/plain").force_close().body(format!("{}",error))
        }
    }
}


pub fn ptmppi_controller(req: HttpRequest<super::State>) -> HttpResponse {
    //get the value of ID
    let id: String  = req.match_info().query("id").unwrap();

    //get content header
    let content_header = misc::get_accept_header_value(&req);

    //get the connection from pool
    let conn;
    match database::connect(&req.state().db_params) {
        Ok(val) => {conn = val},
        Err(error) => {return HttpResponse::InternalServerError().header(http::header::CONTENT_TYPE, "text/plain").force_close().body(format!("{}",error));},
    }

    //get the id strings
    let ptmppi_result = database::get_ptmppis(&id,&conn);

    //check if the operation was successful
    match ptmppi_result {
        Ok(ptmppis) => {
            
            if content_header == "application/json"{
                //try deserializing    
                let ptmppis_serialized_result = serde_json::to_string_pretty(&ptmppis);

                //check if operation was successful
                match ptmppis_serialized_result {

                    Ok(proteoforms_serialized) => {
                        HttpResponse::Ok().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(proteoforms_serialized)
                    },

                    Err(error) => {
                        HttpResponse::InternalServerError().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(format!("{}",error))
                    }

                }
            } else if content_header =="text/plain" {
                    //convert the values to flat structure
                    let ptm_ppi_flat = flatten::ptm_ppi(&ptmppis);

                    let mut wtr = csv::Writer::from_writer(vec![]);
                    
                    for ptm_ppi_flat in ptm_ppi_flat {
                        let result = wtr.serialize(&ptm_ppi_flat);
                        match result {
                            Ok(_) => {},
                            Err(error) => {
                                error!("{}",error);
                                return HttpResponse::InternalServerError().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(format!("{}",error));
                            }
                        }
                    }

                    let inner;
                    let inner_result = wtr.into_inner();
                    match inner_result {
                        Ok(value) => {inner=value;},
                        Err(error) => {
                            error!("{}",error);
                            return HttpResponse::InternalServerError().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(format!("{}",error));
                        }
                    }

                    let data_result = String::from_utf8(inner);
                    match data_result {
                        Ok(data) => {
                            return HttpResponse::Ok().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(data);
                        },
                        Err(error) => {
                            error!("{}",error);
                            return HttpResponse::InternalServerError().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(format!("{}",error));
                        }
                    }                
            }else {
                return HttpResponse::BadRequest().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(format!("Invalid ACCEPT header - {}",content_header));
            }                 



        },

        Err(error) => {
            HttpResponse::InternalServerError().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(format!("{}",error))
        }
    }
}


pub fn batch_ptm_enzymes_controller(req: HttpRequest<super::State>) -> Box<Future<Item=HttpResponse, Error=Error>> {  
    //get content header
    let content_header = misc::get_accept_header_value(&req);

    //get the connection
    let conn_result = database::connect(&req.state().db_params);


    req.concat2()
        .from_err()
        .and_then(move |body_bytes| {
            
            let conn;
            match conn_result  {
                Ok(val) => {conn = val},
                Err(error) => {return Ok(HttpResponse::InternalServerError().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(format!("{}",error)));},
            }
            info!("Got database connection");

            //read the bytes into str
            let body_str;
            let body_read_result = str::from_utf8(&body_bytes);
            match body_read_result {
                Ok(val) => {body_str = val},
                Err(error) => {return Ok(HttpResponse::BadRequest().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(format!("{}",error)));}
            }

            info!("Got raw srtring");

            //parse the string
            let query_substrates: Vec<QuerySubstrate>;
            match serde_json::from_str(body_str) {
                Ok(val) => {query_substrates = val},
                Err(error) => {return Ok(HttpResponse::BadRequest().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(format!("{}",error)));}
            }

            info!("parsed srtring");

            //get the ptm enzymes
            let ptm_enzymes_result = database::get_ptm_enzymes(&query_substrates,&conn);

            info!("Got enzymes");

            match ptm_enzymes_result {
                Ok(ptm_enzymes) => {

                    if content_header == "application/json"{
                        info!("Serializing");
                        let ptm_enzymes_serialized_result = serde_json::to_string_pretty(&ptm_enzymes);
                        match ptm_enzymes_serialized_result {
                            Ok(ptm_enzymes_serialized) => {
                                info!("returned data");
                                return Ok(HttpResponse::Ok().force_close().header(http::header::CONTENT_TYPE, "application/json").body(ptm_enzymes_serialized));    
                            },
                            Err(error) => {
                                error!("{}",error);
                                return Ok(HttpResponse::InternalServerError().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(format!("{}",error)));
                            }
                        }
                    }else if content_header == "text/plain" {
                            //convert the values to flat structure
                            let batch_ptm_enzymes_flat = flatten::batch_ptm_enzymes(&ptm_enzymes);

                            let mut wtr = csv::Writer::from_writer(vec![]);
                            
                            for batch_ptm_enzyme_flat in batch_ptm_enzymes_flat {
                                let result = wtr.serialize(&batch_ptm_enzyme_flat);
                                match result {
                                    Ok(_) => {},
                                    Err(error) => {
                                        error!("{}",error);
                                        return Ok(HttpResponse::InternalServerError().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(format!("{}",error)));
                                    }
                                }
                            }

                            let inner;
                            let inner_result = wtr.into_inner();
                            match inner_result {
                                Ok(value) => {inner=value;},
                                Err(error) => {
                                    error!("{}",error);
                                    return Ok(HttpResponse::InternalServerError().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(format!("{}",error)));
                                }
                            }

                            let data_result = String::from_utf8(inner);
                            match data_result {
                                Ok(data) => {
                                    return Ok(HttpResponse::Ok().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(data));
                                },
                                Err(error) => {
                                    error!("{}",error);
                                    return Ok(HttpResponse::InternalServerError().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(format!("{}",error)));
                                }
                            }                 
                    }else {
                        return Ok(HttpResponse::BadRequest().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(format!("Invalid ACCEPT header - {}",content_header)));
                    }
                },
                Err(error) => {
                    error!("{}",error);
                    return Ok(HttpResponse::InternalServerError().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(format!("{}",error)));
                }
            }            


    })
    .responder()
}

pub fn batch_ptm_ppi_controller(req: HttpRequest<super::State>) -> Box<Future<Item=HttpResponse, Error=Error>> {
    //get content header
    let content_header = misc::get_accept_header_value(&req);

    //get the connection
    let conn_result = database::connect(&req.state().db_params);

    req.concat2()
        .from_err()
        .and_then(move |body_bytes| {
            //read the bytes into str
            let body_str;
            let body_read_result = str::from_utf8(&body_bytes);
            match body_read_result {
                Ok(val) => {body_str = val},
                Err(error) => {return Ok(HttpResponse::BadRequest().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(format!("{}",error)));}
            }

            //parse the string
            let query_substrates: Vec<QuerySubstrate>;
            match serde_json::from_str(body_str) {
                Ok(val) => {query_substrates = val},
                Err(error) => {return Ok(HttpResponse::BadRequest().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(format!("{}",error)));}
            }


            let conn;
            match conn_result  {
                Ok(val) => {conn = val},
                Err(error) => {return Ok(HttpResponse::InternalServerError().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(format!("{}",error)));},
            }

            //get the ptm enzymes
            let ptm_ppis_result = database::get_ptm_ppi(&query_substrates,&conn);

            match ptm_ppis_result {
                Ok(ptm_ppis) => {

                    if content_header == "application/json" {
                        let ptm_ppis_serialized_result = serde_json::to_string_pretty(&ptm_ppis);
                        match ptm_ppis_serialized_result {
                            Ok(ptm_ppis_serialized) => {
                                return Ok(HttpResponse::Ok().force_close().header(http::header::CONTENT_TYPE, "application/json").body(ptm_ppis_serialized));    
                            },
                            Err(error) => {
                                return Ok(HttpResponse::InternalServerError().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(format!("{}",error)));
                            }
                        }
                    }else if content_header == "text/plain"{
                            //convert the values to flat structure
                            let batch_ptm_ppis_flat = flatten::batch_ptm_ppi(&ptm_ppis);

                            let mut wtr = csv::Writer::from_writer(vec![]);
                            
                            for batch_ptm_ppis_flat in batch_ptm_ppis_flat {
                                let result = wtr.serialize(&batch_ptm_ppis_flat);
                                match result {
                                    Ok(_) => {},
                                    Err(error) => {
                                        error!("{}",error);
                                        return Ok(HttpResponse::InternalServerError().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(format!("{}",error)));
                                    }
                                }
                            }

                            let inner;
                            let inner_result = wtr.into_inner();
                            match inner_result {
                                Ok(value) => {inner=value;},
                                Err(error) => {
                                    error!("{}",error);
                                    return Ok(HttpResponse::InternalServerError().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(format!("{}",error)));
                                }
                            }

                            let data_result = String::from_utf8(inner);
                            match data_result {
                                Ok(data) => {
                                    return Ok(HttpResponse::Ok().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(data));
                                },
                                Err(error) => {
                                    error!("{}",error);
                                    return Ok(HttpResponse::InternalServerError().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(format!("{}",error)));
                                }
                            }  
                    }else {
                        return Ok(HttpResponse::BadRequest().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(format!("Invalid ACCEPT header - {}",content_header)));
                    }
                },
                Err(error) => {
                    return Ok(HttpResponse::InternalServerError().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(format!("{}",error)));
                }
            }

        }).responder()
}

pub fn get_statistics_controller(_req: HttpRequest<super::State>) -> HttpResponse {
    //Open the statistics file
    let mut statistics_file;
    match File::open("static/statistics.json") {
        Ok(value) => {
            statistics_file = value;
        },
        Err(error) => {
            error!("{}",error);
            return HttpResponse::InternalServerError().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(format!("{}",error));
        }
    }

    let mut contents = String::new();
    match statistics_file.read_to_string(&mut contents) {
        Ok(_) => {
            
        },
        Err(error) => {
            error!("{}",error);
            return HttpResponse::InternalServerError().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(format!("{}",error));
        }
    }

    return HttpResponse::Ok().force_close().body(contents);
}

pub fn get_msa_controller(req: HttpRequest<super::State>) -> HttpResponse {
    //get the value of ID
    let id: String  = req.match_info().query("id").unwrap();

    //get the connection from pool
    let conn;
    match database::connect(&req.state().db_params) {
        Ok(val) => {conn = val},
        Err(error) => {return HttpResponse::InternalServerError().force_close().body(format!("{}",error));},
    }

    //get the id string
    let sequences_result = database::get_sequences(&id,&conn);

    match sequences_result {
        Ok(sequences) => {
            let alignment_result = msa::align(&sequences);
            match alignment_result {
                Ok(alignment) => {
                    let decorate_result = msa::decorate(&id,&alignment,(&req.state().db_params).clone());
                    match decorate_result {
                        Ok(alignment_decorated) => {
                            let alignment_serialized_result = serde_json::to_string(&alignment_decorated);
                            match alignment_serialized_result {
                                Ok(alignment_serialized) => {
                                    return HttpResponse::Ok()
                                    .force_close()
                                    .header(http::header::CONTENT_TYPE, "application/json")
                                    .body(alignment_serialized);
                                },
                                Err(error) => {
                                    let error_msg = format!("{}",error);
                                    return HttpResponse::Ok()
                                    .force_close()
                                    .header(http::header::CONTENT_TYPE, "application/json")
                                    .body(error_msg);        
                                }
                            }
                        },
                        Err(error) => {
                            let error_msg = format!("{}",error);
                            return HttpResponse::Ok()
                                    .force_close()
                                    .header(http::header::CONTENT_TYPE, "application/json")
                                    .body(error_msg);
                        }
                    }
                },
                Err(error) => {
                    let error_msg = format!("{}",error);
                    return HttpResponse::InternalServerError()
                    .force_close()
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(error_msg);
                }
            }
            
        },
        Err(error) => {
            let error_msg = format!("{}",error);
            return HttpResponse::InternalServerError()
                    .force_close()
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(error_msg);       
        },
    }

}

pub fn get_variants(req: HttpRequest<super::State>) -> HttpResponse {
    //get the value of ID
    let id: String  = req.match_info().query("id").unwrap();

    //get content header
    let content_header = misc::get_accept_header_value(&req);

    //get the connection from pool
    let conn;
    match database::connect(&req.state().db_params) {
        Ok(val) => {conn = val},
        Err(error) => {return HttpResponse::InternalServerError().header(http::header::CONTENT_TYPE, "text/plain").force_close().body(format!("{}",error));},
    }

    //get the id strings
    let variant_result = database::get_variants(&id,&conn);

    //check if the operation was successful
    match variant_result {
        Ok(variants) => {
            
            if content_header == "application/json"{
                //try deserializing    
                let variants_serialized_result = serde_json::to_string_pretty(&variants);

                //check if operation was successful
                match variants_serialized_result {

                    Ok(proteoforms_serialized) => {
                        HttpResponse::Ok().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(proteoforms_serialized)
                    },

                    Err(error) => {
                        HttpResponse::InternalServerError().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(format!("{}",error))
                    }

                }
            } else if content_header =="text/plain" {
                    //convert the values to flat structure

                    let mut wtr = csv::Writer::from_writer(vec![]);
                    
                    for variant_flat in variants {
                        let result = wtr.serialize(&variant_flat);
                        match result {
                            Ok(_) => {},
                            Err(error) => {
                                error!("{}",error);
                                return HttpResponse::InternalServerError().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(format!("{}",error));
                            }
                        }
                    }

                    let inner;
                    let inner_result = wtr.into_inner();
                    match inner_result {
                        Ok(value) => {inner=value;},
                        Err(error) => {
                            error!("{}",error);
                            return HttpResponse::InternalServerError().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(format!("{}",error));
                        }
                    }

                    let data_result = String::from_utf8(inner);
                    match data_result {
                        Ok(data) => {
                            return HttpResponse::Ok().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(data);
                        },
                        Err(error) => {
                            error!("{}",error);
                            return HttpResponse::InternalServerError().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(format!("{}",error));
                        }
                    }                
            }else {
                return HttpResponse::BadRequest().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(format!("Invalid ACCEPT header - {}",content_header));
            }                 



        },

        Err(error) => {
            HttpResponse::InternalServerError().force_close().header(http::header::CONTENT_TYPE, "text/plain").body(format!("{}",error))
        }
    }
}