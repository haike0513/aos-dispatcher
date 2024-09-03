use diesel::PgConnection;
use nostr_sdk::{Event, Kind, SingleLetterTag, TagKind};

use crate::tee::model::{get_tee_question, Params, Question};

pub struct  AosTask {
  pub model: Option<String>,
  pub prompt: Option<String>,
  pub params: Params,
}

impl  AosTask {
    pub fn parse_event(event: &Event) -> Result<Self, ()> {
      let mut e_model = None;
      let mut e_prompt = None;

      if event.kind() == Kind::JobRequest(5050) {

                  let model_tag = event.tags.iter().find(|t| { 
            if t.kind() != TagKind::Custom("param".into()) {
              return  false;
            }
            let content = t.as_vec();
            if let Some(p) = content.get(1) {
              if p.eq(&String::from("model")) {
                e_model = content.get(2).map(|m| {
                  m.clone()
                })
              }
              ;
            }
            return  false;
          });

          let _ = event.tags.iter().find(|t| { 
            if t.kind() != TagKind::SingleLetter(SingleLetterTag::lowercase(nostr_sdk::Alphabet::I)) {
              return  false;
            }
            let content = t.as_vec();
            if let Some(p) = content.get(2) {
              if p.eq(&String::from("prompt")) {
                e_prompt = content.get(1).map(|m| {
                  m.clone()
                })
              }
              ;
            }
            return  false;
          });
      }
      let params = Params {
        temperature: 1.0,
        top_p: 1.0,
        max_tokens: 1024,
      };
      Ok(Self {
        model: e_model,
        prompt: e_prompt,
        params,
      })
    }
}

pub fn query_question(conn: &mut PgConnection) -> Result<Vec<Question>, diesel::result::Error> {
  get_tee_question(conn)
}