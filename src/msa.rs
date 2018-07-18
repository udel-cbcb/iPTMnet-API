use models::Sequence;
use std::io::{Write};
use std::process::{Command, Stdio};
use errors::*;

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
        return Ok(alignment_str);
    }else{
        let error_str = String::from_utf8(output.stderr)?;
        return Err(error_str.into());
    }
}