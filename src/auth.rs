use rocket::request::{FromRequest,Request,Outcome};
use base64::engine::{general_purpose,Engine};
use rocket::http::Status;




pub struct BasicAuth{
    pub username:String,
    pub password:String,
  }
  
   impl  BasicAuth {
       fn from_authorization_header(header: &str) -> Option<BasicAuth>{
        let split = header.split_whitespace().collect::<Vec<_>>();
        if split.len() != 2 {
          return None;
        }
  
        if split[0] != "Basic"{
          return None;
        }
  
        Self::from_base64_encode(split[1])
       }
  
  
  
       fn from_base64_encode(base64_string : &str) -> Option<BasicAuth>{
        let decode = general_purpose::STANDARD.decode(base64_string).ok()?;
        let decode_str = String::from_utf8(decode).ok()?;
        let splite = decode_str.split(":").collect::<Vec<_>>();
      
        if splite.len() != 2{
          return None;
        } 
       
        let (username, password) = (splite[0].to_string(),splite[1].to_string());
      
        let values = Some(BasicAuth{
          username,
          password, 
      
        });
  
        values
       }
   }
  
  #[rocket::async_trait]
  
  impl<'r> FromRequest<'r> for BasicAuth{
    type Error = ();
    
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error>{
      let auth_header = request.headers().get_one("Authorization");
      if let Some(auth_header) = auth_header {
        if let Some(auth) = Self::from_authorization_header(auth_header){
          return Outcome::Success(auth)
        }
      }
  
      Outcome::Error((Status::Unauthorized, ())) 
    }
  }
  