use database::Engine;
use misc;

pub fn info(engine: &Engine) -> String {
    match engine {
        Engine::Postgres => {
            return String::from("SELECT * FROM MV_ENTRY where iptm_entry_code = $1");
        },
        Engine::Oracle => {
            return String::from("SELECT * FROM MV_ENTRY where iptm_entry_code = :1");
        }
    }
}

pub fn search(term_type: &str, role: &str,organism_taxons: &Vec<i32>,engine: &Engine) -> String {
    // build the search term matching clause
    let mut search_term_clause = String::new();

    if term_type == "All" {
        match engine {
            &Engine::Postgres => {
                search_term_clause = String::from("uniprot_id ILIKE $1 OR protein_name ILIKE $2 OR gene_name ILIKE $3");
            },
            &Engine::Oracle => {
                search_term_clause = String::from("regexp_like(uniprot_id,:1,'i') OR regexp_like(protein_name,:2,'i') OR regexp_like(gene_name,:3,'i')")    
            }
        }
        
    }else if term_type == "UniprotID" {
        match engine {
            &Engine::Postgres => {
                search_term_clause = String::from("uniprot_id ILIKE $1")
            },
            &Engine::Oracle => {
                search_term_clause = String::from("regexp_like(uniprot_id,:1,'i')")
            }
        }
        
    }else if term_type == "Protein/Gene Name" {
        match engine {
            &Engine::Postgres => {
                search_term_clause = String::from("uniprot_id ILIKE $1 OR gene_name ILIKE $2")
            },
            &Engine::Oracle => {
                search_term_clause = String::from("regexp_like(uniprot_id,:1,'i')' OR regexp_like(gene_name,:2,'i')")
            }
        }
    }

    // build the enzyme matching clause
    let mut enzyme_clause = "";
    if role == "Enzyme or Substrate" {
        enzyme_clause = "role_as_enzyme = 'T' OR role_as_substrate = 'T'"
    }else if role == "Enzyme" {
        enzyme_clause = "role_as_enzyme = 'T'"
    }else if role  == "Substrate" {
        enzyme_clause = "role_as_substrate = 'T'"
    }else if role == "Enzyme and Substrate" {
        enzyme_clause = "role_as_enzyme = 'T' AND role_as_substrate = 'T'"
    }

    //taxon clause
    let mut taxon_clause = String::new();
    if !organism_taxons.is_empty() {
        let taxon_codes=misc::taxons_to_tuple_str(organism_taxons);
        match engine {
            Engine::Postgres => {
                taxon_clause = format!("AND (taxon_code = ANY ('{{{taxon_codes}}}'))",taxon_codes=taxon_codes);
            },
            Engine::Oracle => {
                taxon_clause = format!("AND (taxon_code = ANY ({taxon_codes}))",taxon_codes=taxon_codes);
            }
        }
    }    

    return format!("SELECT * FROM MV_ENTRY where ({search_term_clause}) AND ({enzyme_clause}) AND iptm_entry_type != 'pro_id' {taxon_clause}",
                    search_term_clause=search_term_clause,
                    enzyme_clause=enzyme_clause,
                    taxon_clause=taxon_clause);

}

pub fn pro_info(engine: &Engine) -> String {
    let query_str = String::from("SELECT * FROM MV_ENTRY where iptm_entry_code = $1");
    match engine {
        &Engine::Postgres => {
            return query_str;
        },
        &Engine::Oracle => {
            return query_str.replace("$",":");
        }
    }
}

pub fn sub_forms(engine: &Engine) -> String {
    let query_str = String::from("SELECT DISTINCT SUB_FORM_CODE from MV_EVENT where SUB_CODE = $1");
    match engine {
        &Engine::Postgres => {
            return query_str;
        },
        &Engine::Oracle => {
            return query_str.replace("$",":");
        }
    }
}

pub fn proteoforms(engine: &Engine) -> String  {
    match engine {
        &Engine::Postgres => {
            return String::from("SELECT * FROM MV_PROTEO where SUB_XREF ILIKE $1 AND EVENT_NAME != 'Interaction'");
        },
        &Engine::Oracle => {
            return String::from("SELECT * FROM MV_PROTEO where regexp_like(SUB_XREF,:1,'i') AND EVENT_NAME != 'Interaction'");
        }
    }
}

pub fn proteoformppi(engine: &Engine) -> String {
    match engine {
        &Engine::Postgres => {
            return String::from("SELECT * FROM MV_PROTEO where SUB_XREF ILIKE $1 AND EVENT_NAME = 'Interaction'");
        },
        &Engine::Oracle => {
            return String::from("SELECT * FROM MV_PROTEO where regexp_like(SUB_XREF,:1,'i') AND EVENT_NAME = 'Interaction'");
        }
    }
}

pub fn ptmppi(engine: &Engine) -> String {
    let query_str = String::from("SELECT * FROM MV_EFIP where PPI_SUB_CODE = $1 OR PPI_PR_CODE = $1"); 
    match engine {
        &Engine::Postgres => {
            return query_str;
        },
        &Engine::Oracle => {
            return query_str.replace("$",":");
        }
    }
}