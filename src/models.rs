
#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    pub uniprot_ac: String,
    pub uniprot_id: String,
    pub protein_name: Option<String>,
    pub gene_name: Option<String>,
    pub synonyms: Vec<String>,
    pub organism: Organism,
    pub pro: Option<Pro>
}

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct Organism {
    pub taxon_code: Option<String>,
    pub species: Option<String>,
    pub common_name: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pro {
    pub id: String,
    pub name: Option<String>,
    pub definition: Option<String>,
    pub short_label: Option<String>,
    pub category: Option<String>,
}

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct SearchResult {
    pub iptm_id: Option<String>,
    pub uniprot_ac: Option<String>,
    pub protein_name: Option<String>,
    pub gene_name: Option<String>,
    pub synonyms: Vec<String>,
    pub organism: Option<Organism>,
    pub substrate_role: bool,
    pub substrate_num: Option<i64>,
    pub enzyme_role: bool,
    pub enzyme_num: Option<i64>,
    pub ptm_dependent_ppi_role: bool,
    pub ptm_dependent_ppi_num: Option<i64>,
    pub sites: Option<i64>,
    pub isoforms: Option<i64>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResultFlat {
    pub iptm_id: Option<String>,
    pub protein_name: Option<String>,
    pub gene_name: Option<String>,
    pub synonyms: Option<String>,
    pub organism_taxon_code: Option<String>,
    pub organism_species: Option<String>,
    pub organism_common_name: Option<String>,
    pub substrate_role: bool,
    pub substrate_num: Option<i64>,
    pub enzyme_role: bool,
    pub enzyme_num: Option<i64>,
    pub ptm_dependent_ppi_role: bool,
    pub ptm_dependent_ppi_num: Option<i64>,
    pub sites: Option<i64>,
    pub isoforms: Option<i64>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubstrateEvent {
    pub residue: Option<String>,
    pub site: Option<String>,
    pub ptm_type: Option<String>,
    pub score: Option<i64>,
    pub sources: Vec<Source>,
    pub enzymes: Vec<Enzyme>,
    pub pmids: Vec<String>, 
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubstrateEventFlat {
    pub sub_form: Option<String>,
    pub residue: Option<String>,
    pub site: Option<String>,
    pub ptm_type: Option<String>,
    pub score: Option<i64>,
    pub sources: Option<String>,
    pub enzymes: Option<String>,
    pub pmids: Option<String>, 
}

#[derive(Serialize, Deserialize, Debug, PartialEq,Clone)]
pub struct Enzyme {
    pub id: Option<String>,
    pub enz_type: Option<String>,
    pub name: Option<String>, 
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Proteoform {
    pub pro_id: Option<String>,
    pub label: Option<String>,
    pub sites: Vec<String>,
    pub ptm_enzyme: Option<Protein>,
    pub source: Option<Source>,
    pub pmids: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProteoformFlat {
    pub pro_id: Option<String>,
    pub label: Option<String>,
    pub sites: Option<String>,
    pub ptm_enzyme_id: Option<String>,
    pub ptm_enzyme_label: Option<String>,
    pub source: Option<String>,
    pub pmids: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Protein {
    pub pro_id: Option<String>,
    pub label: Option<String>
}

#[derive(Serialize, Deserialize, Debug, PartialEq,Clone)]
pub struct Source {
    pub name: Option<String>,
    pub label: Option<String>,
    pub url: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProteoformPPI {
    pub protein_1: Option<Protein>,
    pub relation: Option<String>,
    pub protein_2: Option<Protein>,
    pub source: Option<Source>,
    pub pmids: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProteoformPPIFlat {
    pub protein_1_pro_id: Option<String>,
    pub protein_1_label: Option<String>,
    pub relation: Option<String>,
    pub protein_2_pro_id: Option<String>,
    pub protein_2_label: Option<String>,
    pub source: Option<String>,
    pub pmids: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PTMPPI {
    pub ptm_type : Option<String>,
    pub substrate : Option<Entity>,
    pub site: Option<String>,
    pub interactant: Option<Entity>,
    pub association_type : Option<String>,
    pub source : Option<Source>,
    pub pmid : Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PTMPPIFlat {
    pub ptm_type : Option<String>,
    pub substrate_uniprot_id : Option<String>,
    pub substrate_name : Option<String>,
    pub site: Option<String>,
    pub interactant_uniprot_id: Option<String>,
    pub interactant_name: Option<String>,
    pub association_type : Option<String>,
    pub source : Option<String>,
    pub pmid : Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entity {
    pub uniprot_id: Option<String>,
    pub name: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuerySubstrate {
    pub substrate_ac: String,
    pub site_residue: String,
    pub site_position: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BatchPTMEnzyme {
    pub enzyme: Option<Entity>,
    pub substrate: Option<Entity>,
    pub ptm_type: Option<String>,
    pub site: Option<String>,
    pub site_position: Option<i64>,
    pub score: i64,
    pub source: Vec<Source>,
    pub pmids: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BatchPTMEnzymeFlat {
    pub enz_name: Option<String>,
    pub enz_id: Option<String>,
    pub sub_name: Option<String>,
    pub sub_id: Option<String>,
    pub ptm_type: Option<String>,
    pub site: Option<String>,
    pub site_position: Option<i64>,
    pub score: i64,
    pub source: Option<String>,
    pub pmids: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BatchPTMPPI {
    pub ptm_type: Option<String>,
    pub site: Option<String>,
    pub site_position: Option<i64>,
    pub association_type: Option<String>,
    pub interactant: Option<Entity>,
    pub substrate: Option<Entity>,
    pub source: Option<Source>,
    pub pmids: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BatchPTMPPIFlat {
    pub ptm_type: Option<String>,
    pub site: Option<String>,
    pub site_position: Option<i64>,
    pub association_type: Option<String>,
    pub interactant_id: Option<String>,
    pub interactant_name: Option<String>,
    pub substrate_id: Option<String>,
    pub substrate_name: Option<String>,
    pub source: Option<String>,
    pub pmids: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Alignment {
    pub id: String,
    pub sequence: String,
}


