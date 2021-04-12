#![allow(non_snake_case)]

use neon::prelude::*;

mod client_core;
use crate::client_core::*;

register_module!(mut cx, {
    cx.export_function("login", login_clicked)?;
    cx.export_function("send_message", send_message_clicked)?;
    cx.export_function("poll_messages", poll_messages_clicked)?;
    cx.export_function("poll_friends", handle_poll_friends)?;
    Ok(())
});

fn handle_poll_friends(mut cx: FunctionContext) -> JsResult<JsArray> {
  let username = cx.argument::<JsString>(0)?.value();

  let friends_list = poll_friends(username).expect("TODO");
  let JsFriendsList = JsArray::new(&mut cx, friends_list.len() as u32);
  for (i, friend) in friends_list.iter().enumerate() {
    let friend = cx.string(friend);
    JsFriendsList.set(&mut cx, i as u32, friend)?;
  }

  Ok(JsFriendsList)
}

fn login_clicked(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    // Building the login request - 29 seperating header information, 31 declaring end of data
    let username = cx.argument::<JsString>(0)?.value();
    let password = cx.argument::<JsString>(1)?.value();

    match login(username, password) {
      Ok(success) => {
        Ok(cx.boolean(success))
      }
      Err(login_err) => {
        panic!("{}", login_err) 
      }
    }
}

fn send_message_clicked(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    // Gathering parameters from JavaScript
    let sender = cx.argument::<JsString>(0)?.value();
    let target = cx.argument::<JsString>(1)?.value();
    let message = cx.argument::<JsString>(2)?.value();
    send_message(sender, target, message).unwrap();
    Ok(cx.undefined())
}

fn poll_messages_clicked(mut cx:FunctionContext) -> JsResult<JsArray> {
  let user = cx.argument::<JsString>(0)?.value();

  // Use internal function to get (potentially) a vector of messages
  match poll_messages(user).unwrap() {
    Some(messages) => {
      // Initialize a new JS array, notice the CamelCase for JS objects/vars
      let JsMessageArray = JsArray::new(&mut cx, messages.len() as u32);

      // iterate (enumerate) the (Rust) vector of messages, and map each message to our JS array
      // by creating new JSON objects for each entry in the vector
      for (i, message) in messages.iter().enumerate() { 
        let JsMessageObject = JsObject::new(&mut cx);
        let source = cx.string(message.source.clone());
        let content = cx.string(message.content.clone());

        JsMessageObject.set(&mut cx, "Source", source)?;
        JsMessageObject.set(&mut cx, "Content", content)?;

        // Set the newly created JSON Message object into the array
        JsMessageArray.set(&mut cx, i as u32, JsMessageObject).unwrap();
      }
      // Return the array
      Ok(JsMessageArray)
    },
    None => {
      // Return an empty array, as there are no messages
      Ok(cx.empty_array())
    }
  }
}


// These tests are UGLY! But they work for now
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_login() {
    assert_eq!(login("test".to_owned(), "test".to_owned()).unwrap(), true);
  }

  #[test]
  fn test_messaging() {
    send_message("test".to_string(), "test".to_string(), "hello".to_string()).unwrap();
    let empty_messages = vec![Message {source:"test".to_string(),content:"hello".to_string()}];
    assert_eq!(poll_messages("test".to_string()).unwrap().expect("").get(0).expect(":w"), empty_messages.get(0).expect(""));
  }

  #[test]
  fn test_friendslist() {
    let debug_vec = vec!["asdf".to_string(), "asdf".to_string()];
    assert_eq!(poll_friends("test".to_string()).unwrap(), debug_vec);
  }
}
