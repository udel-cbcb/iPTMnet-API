use models::*;
use misc;
use std::collections::HashMap;

pub fn search_results(search_results: &Vec<SearchResult>) -> Vec<SearchResultFlat> {
    let mut search_results_flat: Vec<SearchResultFlat> = Vec::new();

    for search_result in search_results {
        //flatten organism
        let taxon_code;
        let species;
        let common_name;

        match &search_result.organism {
            &Some(ref organism) => {
                taxon_code = organism.taxon_code.clone();
                species = organism.species.clone();
                common_name = organism.common_name.clone();
            },
            &None => {
                taxon_code = None;
                species = None;
                common_name = None;
            }
        }

        //flatten synonyms
        let synonyms_flat = misc::str_vec_to_str(&search_result.synonyms);

        //build flat search result 
        let search_result_flat = SearchResultFlat {
            iptm_id : search_result.iptm_id.clone(),
            protein_name: search_result.protein_name.clone(),
            gene_name: search_result.gene_name.clone(),
            synonyms: Some(synonyms_flat),
            organism_taxon_code: taxon_code,
            organism_species: species,
            organism_common_name: common_name,
            substrate_role: search_result.substrate_role.clone(),
            substrate_num: search_result.substrate_num.clone(),
            enzyme_role: search_result.enzyme_role.clone(),
            enzyme_num: search_result.enzyme_num.clone(),
            ptm_dependent_ppi_role : search_result.ptm_dependent_ppi_role.clone(),
            ptm_dependent_ppi_num : search_result.ptm_dependent_ppi_num.clone(),
            sites: search_result.sites.clone(),
            isoforms: search_result.isoforms.clone(),
        };

        // add to vector
        search_results_flat.push(search_result_flat);

    }

    return search_results_flat;

}

pub fn substrate_events(substrate_events: &HashMap<String,Vec<SubstrateEvent>>) -> Vec<SubstrateEventFlat> {
    
    let mut substrate_events_flat:Vec<SubstrateEventFlat> = Vec::new();

    for sub_from in substrate_events.iter() {
        let events = sub_from.1;
        for event in events {
            
            //flatten enzymes
            let mut enzymes_str = String::new();
            for (index,enzyme) in event.enzymes.iter().enumerate() {
                
                let mut enzyme_name = String::new();
                let mut enzyme_id = String::new();
                let mut enzyme_type = String::new();

                match enzyme.name {
                    Some(ref value) => {
                        enzyme_name = value.clone();
                    },
                    None => {

                    }
                }

                match enzyme.id {
                    Some(ref value) => {
                        enzyme_id = value.clone();
                    },
                    None => {
                        
                    }
                }

                match enzyme.enz_type {
                    Some(ref value) => {
                        enzyme_type = value.clone();
                    },
                    None => {
                        
                    }
                }

                let current_str = format!("[{name},{id},{enz_type}]",name=enzyme_name,id=enzyme_id,enz_type=enzyme_type);
                if index == 0 {
                    enzymes_str = current_str;
                }else{
                    enzymes_str = format!("{prev_str},{curr_str}",prev_str=enzymes_str,curr_str=current_str);
                }
            };

            let event_flat = SubstrateEventFlat {
                sub_form: Some(sub_from.0.clone()),
                residue: event.residue.clone(),
                site: event.site.clone(),
                ptm_type: event.ptm_type.clone(),
                score: event.score.clone(),
                sources: Some(sources(&event.sources)),
                enzymes: Some(enzymes_str),
                pmids: Some(misc::str_vec_to_str(&event.pmids))
            };

            substrate_events_flat.push(event_flat);

        }   
    }

    return substrate_events_flat;

}

pub fn proteoform(proteoforms: &Vec<Proteoform>) -> Vec<ProteoformFlat> {
    let mut proteoforms_flat: Vec<ProteoformFlat> = Vec::new();
    for proteoform in proteoforms {

        // flatten ptm enzyme
        let ptm_enzyme_id;
        let ptm_enzyme_label;
        match &proteoform.ptm_enzyme {
            &Some(ref ptm_enzyme) => {
                ptm_enzyme_id = ptm_enzyme.pro_id.clone();
                ptm_enzyme_label = ptm_enzyme.label.clone();
            },
            &None => {
                ptm_enzyme_id = None;
                ptm_enzyme_label = None;
            }
        }

        // flatten source
        let source_name;
        match &proteoform.source {
            &Some(ref source) => {
                source_name = source.name.clone(); 
            },
            &None => {
                source_name = None;
            }
        }

        //build the flat proteoform
        let proteoform_flat = ProteoformFlat {
            pro_id : proteoform.pro_id.clone(),
            label: proteoform.label.clone(),
            sites: Some(misc::str_vec_to_str(&proteoform.sites)),
            ptm_enzyme_id: ptm_enzyme_id,
            ptm_enzyme_label: ptm_enzyme_label,
            source: source_name,
            pmids: Some(misc::str_vec_to_str(&proteoform.sites))
        };

        //add to vector
        proteoforms_flat.push(proteoform_flat);
    }
    return proteoforms_flat;
}


pub fn proteoform_ppis(proteoforms_ppi: &Vec<ProteoformPPI>) -> Vec<ProteoformPPIFlat> {
    let mut proteoforms_ppi_flat: Vec<ProteoformPPIFlat> = Vec::new();
    for proteoform_ppi in proteoforms_ppi {

        //build protein_1
        let protein_1_pro_id;
        let protein_1_label;
        match &proteoform_ppi.protein_1 {
            &Some(ref protein_1) => {
                protein_1_pro_id = protein_1.pro_id.clone();
                protein_1_label = protein_1.label.clone();
            },
            &None => {
                protein_1_pro_id = None;
                protein_1_label = None;
            }
        }

        //build protein_2
        let protein_2_pro_id;
        let protein_2_label;
        match &proteoform_ppi.protein_2 {
            &Some(ref protein_2) => {
                protein_2_pro_id = protein_2.pro_id.clone();
                protein_2_label = protein_2.label.clone();
            },
            &None => {
                protein_2_pro_id = None;
                protein_2_label = None;
            }
        }


        //pmids
        let pmids = misc::str_vec_to_str(&proteoform_ppi.pmids);


        //source
        let source_name;
        match &proteoform_ppi.source {
            &Some(ref source) => {
                source_name = source.name.clone(); 
            },
            &None => {
                source_name = None;
            }
        }

        //build flat proteoform_ppi
        let proteoform_ppi_flat = ProteoformPPIFlat {
            protein_1_pro_id: protein_1_pro_id,
            protein_1_label: protein_1_label,
            protein_2_pro_id: protein_2_pro_id,
            protein_2_label: protein_2_label,
            relation: proteoform_ppi.relation.clone(),
            source: source_name,
            pmids: Some(pmids)
        };

        //add to vector
        proteoforms_ppi_flat.push(proteoform_ppi_flat);
    }
    return proteoforms_ppi_flat;
}

pub fn ptm_ppi(ptm_ppis: &Vec<PTMPPI>) -> Vec<PTMPPIFlat> {
    let mut ptm_ppis_flat: Vec<PTMPPIFlat> = Vec::new();
    for ptm_ppi in ptm_ppis {
        //flatten substrate
        let substrate_uniprot_id;
        let substrate_name;
        match &ptm_ppi.substrate {
            &Some(ref substrate) => {
                substrate_uniprot_id = substrate.uniprot_id.clone();
                substrate_name = substrate.name.clone();
            },
            &None => {
                substrate_uniprot_id = None;
                substrate_name = None;
            }
        }       

        //flatten interactant
        let interactant_uniprot_id;
        let interactant_name;
        match &ptm_ppi.interactant {
            &Some(ref interactant) => {
                interactant_uniprot_id = interactant.uniprot_id.clone();
                interactant_name = interactant.name.clone();
            },
            &None => {
                interactant_uniprot_id = None;
                interactant_name = None;
            }
        }

        //source
        let source_name;
        match &ptm_ppi.source {
            &Some(ref source) => {
                source_name = source.name.clone(); 
            },
            &None => {
                source_name = None;
            }
        }

        //build flat ptm_ppi
        let ptm_ppi_flat = PTMPPIFlat {
            ptm_type: ptm_ppi.ptm_type.clone(),
            substrate_uniprot_id: substrate_uniprot_id,
            substrate_name: substrate_name,
            site: ptm_ppi.site.clone(),
            interactant_uniprot_id: interactant_uniprot_id,
            interactant_name: interactant_name,
            association_type: ptm_ppi.association_type.clone(),
            source: source_name,
            pmid: ptm_ppi.pmid.clone()
        };

        //add to vector
        ptm_ppis_flat.push(ptm_ppi_flat);
    }

    return ptm_ppis_flat;
}

pub fn batch_ptm_enzymes(batch_ptm_enzymes: &Vec<BatchPTMEnzyme>) -> Vec<BatchPTMEnzymeFlat>{
    
    let mut batch_ptm_enzymes_flat: Vec<BatchPTMEnzymeFlat> = Vec::new();

    for batch_ptm_enzyme in batch_ptm_enzymes {
        //flatten enzyme
        let enz_name;
        let enz_id;
        match &batch_ptm_enzyme.enzyme {
            &Some(ref enzyme) => {
                enz_name = enzyme.name.clone();
                enz_id = enzyme.uniprot_id.clone();
            },
            &None => {
                enz_name = None;
                enz_id = None;
            }
        }

        //flatten substrate
        let sub_name;
        let sub_id;
        match &batch_ptm_enzyme.substrate {
            &Some(ref substrate) => {
                sub_name = substrate.name.clone();
                sub_id = substrate.uniprot_id.clone();
            },
            &None => {
                sub_name = None;
                sub_id = None;
            }
        }

        //fatten sources
        let sources = sources(&batch_ptm_enzyme.source);

        //build flat batch_ptm_enzyme
        let batch_ptm_enzyme_flat = BatchPTMEnzymeFlat {
            enz_id: enz_id,
            enz_name: enz_name,
            sub_id: sub_id,
            sub_name: sub_name,
            ptm_type: batch_ptm_enzyme.ptm_type.clone(),
            site: batch_ptm_enzyme.site.clone(),
            site_position: batch_ptm_enzyme.site_position.clone(),
            score: batch_ptm_enzyme.score.clone(),
            source: Some(sources),
            pmids: Some(misc::str_vec_to_str(&batch_ptm_enzyme.pmids)),
        };

        batch_ptm_enzymes_flat.push(batch_ptm_enzyme_flat);

    }

    return batch_ptm_enzymes_flat;

}

pub fn batch_ptm_ppi(batch_ptm_ppis: &Vec<BatchPTMPPI>) -> Vec<BatchPTMPPIFlat> {
    let mut batch_ptm_ppis_flat: Vec<BatchPTMPPIFlat> = Vec::new();

    for batch_ptm_ppi in batch_ptm_ppis {
        //flatten interactant
        let interactant_uniprot_id;
        let interactant_name;
        match &batch_ptm_ppi.interactant {
            &Some(ref interactant) => {
                interactant_uniprot_id = interactant.uniprot_id.clone();
                interactant_name = interactant.name.clone();
            },
            &None => {
                interactant_uniprot_id = None;
                interactant_name = None;
            }
        }

        //flatten substrate
        let substrate_uniprot_id;
        let substrate_name;
        match &batch_ptm_ppi.substrate {
            &Some(ref substrate) => {
                substrate_uniprot_id = substrate.uniprot_id.clone();
                substrate_name = substrate.name.clone();
            },
            &None => {
                substrate_uniprot_id = None;
                substrate_name = None;
            }
        }

        //source
        let source_name;
        match &batch_ptm_ppi.source {
            &Some(ref source) => {
                source_name = source.name.clone(); 
            },
            &None => {
                source_name = None;
            }
        }

        // build flat batch_ptm_ppi
        let batch_ptm_ppi_flat = BatchPTMPPIFlat {
            ptm_type: batch_ptm_ppi.ptm_type.clone(),
            site: batch_ptm_ppi.site.clone(),
            site_position: batch_ptm_ppi.site_position.clone(),
            association_type: batch_ptm_ppi.association_type.clone(),
            interactant_id: interactant_uniprot_id,
            interactant_name: interactant_name,
            substrate_id: substrate_uniprot_id,
            substrate_name: substrate_name,
            source: source_name,
            pmids: Some(misc::str_vec_to_str(&batch_ptm_ppi.pmids))
        };

        //add to vector
        batch_ptm_ppis_flat.push(batch_ptm_ppi_flat);

    }

    return batch_ptm_ppis_flat;
}

pub fn sources(sources: &Vec<Source>) -> String {
    let mut sources_str: String = String::new();

    for (index,source) in sources.iter().enumerate(){
        if index == 0 {
            match &source.name {
                &Some(ref name) => {sources_str = name.clone()},
                &None => {}
            }
        }else {
            match &source.name {
                &Some(ref name) => {
                    sources_str = format!("{prev_str},{curr_str}",prev_str=sources_str,curr_str=name);
                },
                &None => {}
            }    
        }
    }

    return sources_str;
}