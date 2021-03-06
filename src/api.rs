use nickel::{FormBody, Request, Response, MiddlewareResult, QueryString};
use reject::{bad_request, not_allowed};
use store::Store;

pub fn get<'app>(req: &mut Request, res: Response<'app>) -> MiddlewareResult<'app> {
  if !req.origin.remote_addr.ip().is_loopback() {
    return not_allowed(res);
  }

  let mut body = String::from("[");
  let desk = Store::new("desk");

  for address in desk.shelf.keys() {
    body.push_str("\"");
    body.push_str(&address);
    body.push_str("\",");
  }
  body.pop();
  body.push_str("]");

  res.send(body)
}

pub fn delete<'app>(req: &mut Request, res: Response<'app>) -> MiddlewareResult<'app> {
  if !req.origin.remote_addr.ip().is_loopback() {
    return not_allowed(res);
  }

  let desk = Store::new("desk");

  match req.query().get("site") {
    Some(site) => desk.shelf.delete(site),
    None => ()
  }

  res.send("Two'O'Four")
}

pub fn post<'app>(req: &mut Request, res: Response<'app>) -> MiddlewareResult<'app> {
  if !req.origin.remote_addr.ip().is_loopback() {
    return not_allowed(res);
  }

  let desk = Store::new("desk");

  match req.form_body() {
    Err(_) => bad_request(res),
    Ok(data) => match data.get("site") {
      None => bad_request(res),
      Some(site) => match data.get("target") {
        None => bad_request(res),
        Some(target) => {
          desk.shelf.set(site, target);
          res.send("Two'O'Four")
        }
      }
    }
  }
}
