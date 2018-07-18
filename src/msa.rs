use models::Sequence;
use models::Alignment;
use models::AlignmentItem;
use std::io::{Write};
use std::process::{Command, Stdio};
use errors::*;
use bio::io;

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

pub fn decorate(alignmened_sequences: &str) -> Result<Vec<Alignment>> {
    let mut alignments : Vec<Alignment> = Vec::new();

    //decode the fasta string
    let fasta_reader = io::fasta::Reader::new(alignmened_sequences.as_bytes());

    for record_result in fasta_reader.records(){
        let record = record_result?;
        let id = record.id();
        let seq = record.seq();

        let mut alignmentitems: Vec<AlignmentItem> = Vec::new();
        for(position,seq_item) in seq.iter().enumerate(){
            let alignment_item = AlignmentItem{
                site: String::from_utf8(vec![*seq_item])?,
                position: position as i16,
                decorations: Vec::new()
            };
            alignmentitems.push(alignment_item);
        }

        let alignment = Alignment {
            id: String::from(id),
            sequence: alignmentitems 
        };
        alignments.push(alignment);

    }

    return Ok(alignments);
}