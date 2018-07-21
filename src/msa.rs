use models::Sequence;
use database;
use models::Alignment;
use models::AlignmentItem;
use std::io::{Write};
use std::process::{Command, Stdio};
use errors::*;
use bio::io;
use rayon::prelude::*;

fn to_fasta(sequences: &Vec<Sequence>) -> String {
    let mut fasta_string = String::from("");
    for sequence in sequences {
        fasta_string = format!("{prev_seq}>{id}\n{seq}\n",prev_seq=fasta_string,id=sequence.id,seq=sequence.sequence);
    }

    return fasta_string;
}

pub fn align(sequences: &Vec<Sequence>) -> Result<String> {
    let fasta_string = to_fasta(sequences);

    let mut muscle = Command::new("muscle")
        .arg("-quiet")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    {
        let child_stdin_option = muscle.stdin.as_mut();
        match child_stdin_option {
            Some(child_stdin) => {
                child_stdin.write_all(&fasta_string.as_bytes())?;
            },
            None => {
               return Err("stdin is null".into());     
            }
        }
    }

    let output = muscle.wait_with_output()?;

    if output.status.success() {
        let alignment_str = String::from_utf8(output.stdout)?;
        //info!("{}",alignment_str);    
        return Ok(alignment_str);
    }else{
        let error_str = String::from_utf8(output.stderr)?;
        return Err(error_str.into());
    }
}

pub fn decorate(id: &str,alignmened_sequences: &str,db_params: database::DBParams) -> Result<Vec<Alignment>> {
    let mut alignments : Vec<Alignment> = Vec::new();

    //decode the fasta string
    let fasta_reader = io::fasta::Reader::new(alignmened_sequences.as_bytes());
       
    for record_result in fasta_reader.records(){
        let record = record_result?;
        let form_id = record.id();
        let seq = record.seq();

        let closure = |seq_item| decorate_item(seq_item,String::from(id),String::from(form_id),db_params.clone());
       
        let alignment_items = seq.par_iter().enumerate().map(closure).collect::<Result<Vec<AlignmentItem>>>()?;

        let alignment = Alignment {
            id: String::from(form_id),
            sequence: alignment_items //for now we add an empty list
        };
        alignments.push(alignment);
    }   

    return Ok(alignments);
}


fn decorate_item(seq_item: (usize, &u8),id: String,form_id: String,db_params: database::DBParams) -> Result<AlignmentItem> {    
    let str_vec = [*(seq_item.1)].to_vec();
    let site;
    match String::from_utf8(str_vec) {
        Ok(val) => {
            site = val;
        },
        Err(_) => {
            site = String::from("-");
        }
    }

    let mut position = seq_item.0 as i16;
    position = position + 1;

    let conn = database::connect(&db_params)?;
    let decorations = database::get_decorations(&id,&form_id,position as i64,&site,&conn)?;

    let alignment_item = AlignmentItem {
        site: site,
        position: position,
        decorations: decorations
    };
    return Ok(alignment_item);       
}

